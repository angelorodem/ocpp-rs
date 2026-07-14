//! OCPP-J CALLRESULTERROR (message type 5) — OCPP 2.1.

use alloc::{collections::BTreeMap, string::String};
use serde_json::Value;
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

use super::rpc_error_code::RpcErrorCode;

#[derive(Debug, PartialEq, Serialize_tuple, Deserialize_tuple, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CallResultError {
    pub(super) message_id: i32,
    pub unique_id: String,
    pub error_code: RpcErrorCode,
    pub error_description: String,
    pub error_details: BTreeMap<String, Value>,
}

impl CallResultError {
    #[must_use]
    pub fn new(
        unique_id: String,
        error_code: RpcErrorCode,
        error_description: String,
        error_details: BTreeMap<String, Value>,
    ) -> Self {
        Self {
            message_id: 5,
            unique_id,
            error_code,
            error_description,
            error_details,
        }
    }

    #[must_use]
    pub fn generic_error(unique_id: String, error_description: String) -> Self {
        Self::new(
            unique_id,
            RpcErrorCode::GenericError,
            error_description,
            BTreeMap::new(),
        )
    }
}
