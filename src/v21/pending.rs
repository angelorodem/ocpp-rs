//! Correlate outbound CALLs so inbound CALLRESULTs can be typed without ambiguity.
//!
//! # Why this exists
//!
//! OCPP CALLRESULT frames are `[3, messageId, payload]` with **no action name**. Many responses
//! share shapes (`{}`, `{"status":"Accepted"}`, …). This crate therefore never uses untagged
//! response enums. You must supply the expected action (from prior CALL correlation) or handle
//! [`CallResultRaw`](crate::v21::call_result::CallResultRaw) yourself.
//!
//! Same pattern as [`crate::v16::pending`].
//!
//! # Shortcomings of in-memory [`PendingCalls`]
//!
//! - **Process-local only.** The map lives in RAM on one node. Behind a load balancer, the
//!   CALLRESULT may arrive on a different instance → [`Error::UnknownPendingMessageId`].
//! - **Requires sticky sessions or shared storage** for multi-node WebSocket / RPC workers.
//! - **Does not survive restarts** unless you persist pending entries yourself.
//! - Storing a full [`Action`](crate::v21::call::Action) keeps the request payload in memory;
//!   for Redis/DB prefer [`PendingActionNames`] (action **string** only).
//!
//! # Load-balanced / stateless options
//!
//! 1. **Sticky connection** (one charging station → one process): use [`PendingCalls`].
//! 2. **Shared store** of `messageId → action name` (Redis, DB, …): on send, `PUT id, "Heartbeat"`;
//!    on result, [`TypedCallResult::resolve_from_action_name`].
//!    Use [`PendingActionNames`] in-process, or implement the same get/put/remove against Redis.
//! 3. **App already knows `T`:** [`call_result::deserialize_call_result`](crate::v21::call_result::deserialize_call_result)
//!    / [`CallResultRaw::into_typed`](crate::v21::call_result::CallResultRaw::into_typed).
//! 4. **Expected type truly unknown:** keep [`CallResultRaw`], or call
//!    [`CallResultRaw::probe_candidates`](crate::v21::call_result::CallResultRaw::probe_candidates) /
//!    [`try_resolve_unique`](try_resolve_unique). Probing is a **last resort** — empty and
//!    status-only payloads match many schemas ([`Error::AmbiguousCallResult`]).

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use super::call::{Action, Call};
use super::call_result::CallResultRaw;
use super::parse::{self, Message, TypedMessage};
use super::typed_call_result::TypedCallResult;
use crate::errors::{Error, Result};

/// Tracks in-flight CALL `messageId` → [`Action`] pairs (process-local).
///
/// Register **outbound** calls with [`Self::register`] / [`Self::send_call`], then resolve
/// type-3 frames to concrete `*Response` types.
///
/// See module docs for scaling shortcomings and alternatives.
#[derive(Debug, Default, Clone)]
pub struct PendingCalls {
    pending: BTreeMap<String, Action>,
}

impl PendingCalls {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Remember that `unique_id` was used for a CALL with this action/payload.
    pub fn register(&mut self, unique_id: impl Into<String>, action: Action) {
        self.pending.insert(unique_id.into(), action);
    }

    /// Register from a full [`Call`].
    pub fn register_call(&mut self, call: &Call) {
        self.register(call.unique_id.clone(), call.payload.clone());
    }

    /// Take the pending action for `unique_id`, if any.
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

    /// Resolve a raw CALLRESULT using a previously registered action.
    ///
    /// # Errors
    /// * [`Error::UnknownPendingMessageId`] if `unique_id` was not registered
    /// * [`Error::SerdeJson`] if the payload does not match the expected response
    pub fn resolve(&mut self, raw: CallResultRaw) -> Result<TypedCallResult> {
        let action = self
            .pending
            .remove(&raw.unique_id)
            .ok_or_else(|| Error::UnknownPendingMessageId(raw.unique_id.clone()))?;
        TypedCallResult::resolve(raw, &action)
    }

    /// Register an outbound CALL, then serialize it to JSON.
    ///
    /// # Errors
    /// Returns serialization errors from [`parse::serialize_message`].
    pub fn send_call(&mut self, call: Call) -> Result<String> {
        self.register_call(&call);
        parse::serialize_message(&Message::Call(call))
    }

    /// Deserialize a wire frame and resolve CALLRESULT when a matching pending CALL exists.
    ///
    /// - **`CallResult`**: requires a prior [`Self::register`] / [`Self::send_call`]
    /// - Other variants: passed through (inbound CALLs are **not** auto-registered)
    ///
    /// # Errors
    /// Propagates parse and resolve errors.
    pub fn deserialize_typed(&mut self, data: &str) -> Result<TypedMessage> {
        let message = parse::deserialize_to_message(data)?;
        self.message_to_typed(message)
    }

    /// Convert a blind [`Message`] into a [`TypedMessage`], resolving CALLRESULT via the map.
    ///
    /// # Errors
    /// Returns [`Error::UnknownPendingMessageId`] or payload deserialize errors for CALLRESULT.
    pub fn message_to_typed(&mut self, message: Message) -> Result<TypedMessage> {
        match message {
            Message::Call(call) => Ok(TypedMessage::Call(call)),
            Message::CallResult(raw) => {
                let typed = self.resolve(raw)?;
                Ok(TypedMessage::CallResult(typed))
            }
            Message::CallError(e) => Ok(TypedMessage::CallError(e)),
            Message::CallResultError(e) => Ok(TypedMessage::CallResultError(e)),
            Message::Send(s) => Ok(TypedMessage::Send(s)),
        }
    }
}

/// Process-local map of `messageId` → **action name string** (e.g. `"Heartbeat"`).
///
/// Prefer this over [`PendingCalls`] when you will mirror the same key/value into Redis or a DB:
/// values are small, `Copy`-friendly strings, and resolve via
/// [`TypedCallResult::resolve_from_action_name`].
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
        self.register(call.unique_id.clone(), call.payload.action_name());
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
    /// [`Error::UnknownPendingMessageId`] or payload / action-name errors from resolve.
    pub fn resolve(&mut self, raw: CallResultRaw) -> Result<TypedCallResult> {
        let action_name = self
            .pending
            .remove(&raw.unique_id)
            .ok_or_else(|| Error::UnknownPendingMessageId(raw.unique_id.clone()))?;
        TypedCallResult::resolve_from_action_name(raw, &action_name)
    }

    /// # Errors
    /// Serialization errors from [`parse::serialize_message`].
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
            Message::CallResultError(e) => Ok(TypedMessage::CallResultError(e)),
            Message::Send(s) => Ok(TypedMessage::Send(s)),
        }
    }
}

/// Resolve a CALLRESULT when you already loaded the action name from an external store.
///
/// Typical load-balanced flow:
/// 1. On CALL send: `redis.set(message_id, action_name)`
/// 2. On CALLRESULT receive (any node): `action = redis.getdel(message_id)`
/// 3. `resolve_with_action_name(raw, &action)`
///
/// # Errors
/// Propagates [`TypedCallResult::resolve_from_action_name`] errors.
pub fn resolve_with_action_name(raw: CallResultRaw, action_name: &str) -> Result<TypedCallResult> {
    TypedCallResult::resolve_from_action_name(raw, action_name)
}

/// Last-resort typing when **no** correlation data is available.
///
/// Runs [`CallResultRaw::probe_candidates`]. Returns the sole match, or errors if zero / many.
///
/// # Errors
/// * [`Error::AmbiguousCallResult`] if 0 or ≥2 schemas accept the payload
///
/// # Shortcoming
/// `{}` and `{"status":…}` payloads routinely match many actions — do not rely on this in
/// production for those shapes; persist action names instead.
pub fn try_resolve_unique(raw: &CallResultRaw) -> Result<TypedCallResult> {
    let mut candidates = raw.probe_candidates();
    match candidates.len() {
        1 => {
            // pop after len==1 check — infallible
            Ok(candidates.remove(0))
        }
        0 => Err(Error::AmbiguousCallResult(
            "payload matched no known OCPP 2.1 response schema".to_string(),
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
