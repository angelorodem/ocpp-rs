//! Correlate outbound CALLs so inbound CALLRESULTs can be typed without ambiguity.
//!
//! # Why this exists
//!
//! OCPP 1.6 CALLRESULT frames are `[3, messageId, payload]` with **no action name**. Many
//! responses share shapes (`{}`, `{"status":"Accepted"}`, …). Older versions of this crate used
//! untagged enums (`ResultPayload` / `EmptyResponses` / `StatusResponses`) that silently picked
//! the wrong variant. That path is gone: you must supply the expected action or keep
//! [`CallResultRaw`](crate::v16::call_result::CallResultRaw).
//!
//! Same pattern as [`crate::v21::pending`].
//!
//! # Shortcomings of in-memory [`PendingCalls`]
//!
//! - **Process-local only.** Behind a load balancer, CALLRESULT may hit another instance →
//!   [`Error::UnknownPendingMessageId`](crate::errors::Error::UnknownPendingMessageId).
//! - **Not durable** across restarts.
//! - Prefer sticky WebSocket sessions, or persist `messageId → action name` (see
//!   [`PendingActionNames`] / [`resolve_with_action_name`]).
//!
//! # Load-balanced / unknown type
//!
//! 1. Sticky connection → [`PendingCalls`]
//! 2. Shared store of action names → [`resolve_with_action_name`]
//! 3. Known `T` → [`crate::v16::call_result::deserialize_call_result`]
//! 4. Last resort → [`try_resolve_unique`] / [`CallResultRaw::probe_candidates`](crate::v16::call_result::CallResultRaw::probe_candidates)
//!    (ambiguous for empty/status-only JSON)

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use super::call::{Action, Call};
use super::call_result::CallResultRaw;
use super::parse::{self, Message, TypedMessage};
use super::typed_call_result::TypedCallResult;
use crate::errors::{Error, Result};

/// Process-local `messageId` → [`Action`] map.
#[derive(Debug, Default, Clone)]
pub struct PendingCalls {
    pending: BTreeMap<String, Action>,
}

impl PendingCalls {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, unique_id: impl Into<String>, action: Action) {
        self.pending.insert(unique_id.into(), action);
    }

    pub fn register_call(&mut self, call: &Call) {
        self.register(call.unique_id.clone(), call.payload.clone());
    }

    pub fn take(&mut self, unique_id: &str) -> Option<Action> {
        self.pending.remove(unique_id)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.pending.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }

    /// # Errors
    /// Unknown id or payload mismatch.
    pub fn resolve(&mut self, raw: CallResultRaw) -> Result<TypedCallResult> {
        let action = self
            .pending
            .remove(&raw.unique_id)
            .ok_or_else(|| Error::UnknownPendingMessageId(raw.unique_id.clone()))?;
        TypedCallResult::resolve(raw, &action)
    }

    /// # Errors
    /// Serialization errors.
    pub fn send_call(&mut self, call: Call) -> Result<String> {
        self.register_call(&call);
        parse::serialize_message(&Message::Call(call))
    }

    /// # Errors
    /// Parse / resolve errors.
    pub fn deserialize_typed(&mut self, data: &str) -> Result<TypedMessage> {
        let message = parse::deserialize_to_message(data)?;
        self.message_to_typed(message)
    }

    /// # Errors
    /// Resolve errors for CALLRESULT.
    pub fn message_to_typed(&mut self, message: Message) -> Result<TypedMessage> {
        match message {
            Message::Call(call) => Ok(TypedMessage::Call(call)),
            Message::CallResult(raw) => Ok(TypedMessage::CallResult(self.resolve(raw)?)),
            Message::CallError(e) => Ok(TypedMessage::CallError(e)),
        }
    }
}

/// Process-local `messageId` → action **name** string (Redis-friendly).
#[derive(Debug, Default, Clone)]
pub struct PendingActionNames {
    pending: BTreeMap<String, String>,
}

impl PendingActionNames {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, unique_id: impl Into<String>, action_name: impl Into<String>) {
        self.pending.insert(unique_id.into(), action_name.into());
    }

    pub fn register_call(&mut self, call: &Call) {
        self.register(call.unique_id.clone(), call.payload.as_ref());
    }

    pub fn take(&mut self, unique_id: &str) -> Option<String> {
        self.pending.remove(unique_id)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.pending.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }

    /// # Errors
    /// Unknown id or resolve failure.
    pub fn resolve(&mut self, raw: CallResultRaw) -> Result<TypedCallResult> {
        let action_name = self
            .pending
            .remove(&raw.unique_id)
            .ok_or_else(|| Error::UnknownPendingMessageId(raw.unique_id.clone()))?;
        TypedCallResult::resolve_from_action_name(raw, &action_name)
    }

    /// # Errors
    /// Serialization errors.
    pub fn send_call(&mut self, call: Call) -> Result<String> {
        self.register_call(&call);
        parse::serialize_message(&Message::Call(call))
    }

    /// # Errors
    /// Parse / resolve errors.
    pub fn deserialize_typed(&mut self, data: &str) -> Result<TypedMessage> {
        let message = parse::deserialize_to_message(data)?;
        match message {
            Message::Call(call) => Ok(TypedMessage::Call(call)),
            Message::CallResult(raw) => Ok(TypedMessage::CallResult(self.resolve(raw)?)),
            Message::CallError(e) => Ok(TypedMessage::CallError(e)),
        }
    }
}

/// Resolve after loading the action name from an external store.
///
/// # Errors
/// Propagates [`TypedCallResult::resolve_from_action_name`].
pub fn resolve_with_action_name(raw: CallResultRaw, action_name: &str) -> Result<TypedCallResult> {
    TypedCallResult::resolve_from_action_name(raw, action_name)
}

/// Last resort when no correlation exists. Errors if 0 or ≥2 schemas match.
///
/// # Errors
/// [`Error::AmbiguousCallResult`]
pub fn try_resolve_unique(raw: &CallResultRaw) -> Result<TypedCallResult> {
    let mut candidates = raw.probe_candidates();
    match candidates.len() {
        1 => Ok(candidates.remove(0)),
        0 => Err(Error::AmbiguousCallResult(
            "payload matched no known OCPP 1.6 response schema".to_string(),
        )),
        n => {
            let names: Vec<&str> = candidates
                .iter()
                .map(TypedCallResult::action_name)
                .collect();
            Err(Error::AmbiguousCallResult(format!(
                "payload matched {n} response schemas: {names:?}"
            )))
        }
    }
}
