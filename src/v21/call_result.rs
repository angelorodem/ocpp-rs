//! OCPP-J CALLRESULT (message type 3).
//!
//! `CallResult` payloads have **no action field** on the wire. Blind deserialization therefore
//! yields [`CallResultRaw`]; use [`crate::v21::pending::PendingCalls`] (or
//! [`CallResultRaw::into_typed`]) to obtain a concrete [`crate::v21::typed_call_result::TypedCallResult`].

use alloc::string::String;
use serde::de::DeserializeOwned;
use serde_json::Value;
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

use crate::errors::{Error, Result};

/// Untyped CALLRESULT as received from the wire.
#[derive(Debug, PartialEq, Eq, Serialize_tuple, Deserialize_tuple, Clone)]
pub struct CallResultRaw {
    pub(super) message_id: i32,
    pub unique_id: String,
    pub payload: Value,
}

impl CallResultRaw {
    #[must_use]
    pub const fn new(unique_id: String, payload: Value) -> Self {
        Self {
            message_id: 3,
            unique_id,
            payload,
        }
    }

    /// Deserialize the payload into a concrete response type.
    ///
    /// # Errors
    /// Returns [`Error::SerdeJson`] if the payload does not match `T`.
    pub fn into_typed<T: DeserializeOwned>(self) -> Result<CallResult<T>> {
        let payload = serde_json::from_value(self.payload).map_err(Error::SerdeJson)?;
        Ok(CallResult {
            message_id: 3,
            unique_id: self.unique_id,
            payload,
        })
    }

    /// Try every known `*Response` schema against this payload.
    ///
    /// **Shortcoming:** empty (`{}`) and status-only objects often match many actions.
    /// Prefer correlating via action name ([`crate::v21::pending`]) in multi-node deploys.
    /// See also [`crate::v21::pending::try_resolve_unique`].
    #[must_use]
    pub fn probe_candidates(
        &self,
    ) -> alloc::vec::Vec<crate::v21::typed_call_result::TypedCallResult> {
        crate::v21::typed_call_result::TypedCallResult::probe_from_raw(self)
    }
}

/// Typed CALLRESULT used after resolving a raw result, or when building a response in memory.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CallResult<T> {
    pub(super) message_id: i32,
    pub unique_id: String,
    pub payload: T,
}

impl<T> CallResult<T> {
    #[must_use]
    pub const fn new(unique_id: String, payload: T) -> Self {
        Self {
            message_id: 3,
            unique_id,
            payload,
        }
    }
}

/// Deserialize a CALLRESULT JSON array into a concrete response type.
///
/// # Errors
/// Returns an error if the frame is not type 3 or the payload does not match `T`.
pub fn deserialize_call_result<T: DeserializeOwned>(data: &str) -> Result<CallResult<T>> {
    let raw: CallResultRaw = serde_json::from_str(data).map_err(Error::SerdeJson)?;
    if raw.message_id != 3 {
        return Err(Error::InvalidMessageCallType);
    }
    raw.into_typed()
}
