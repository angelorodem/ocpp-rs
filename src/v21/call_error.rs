//! OCPP-J CALLERROR (message type 4).

use alloc::{collections::BTreeMap, string::String};
use serde_json::Value;
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

#[derive(Debug, PartialEq, Serialize_tuple, Deserialize_tuple, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CallError {
    pub(super) message_id: i32,
    pub unique_id: String,
    pub error_code: String,
    pub error_description: String,
    pub error_details: BTreeMap<String, Value>,
}

impl CallError {
    #[must_use]
    pub fn new(
        unique_id: String,
        error_code: String,
        error_description: String,
        error_details: BTreeMap<String, Value>,
    ) -> Self {
        Self {
            message_id: 4,
            unique_id,
            error_code,
            error_description,
            error_details,
        }
    }
}
