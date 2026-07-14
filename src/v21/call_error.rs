//! OCPP-J CALLERROR (message type 4).

use alloc::{collections::BTreeMap, string::String};
use serde_json::Value;
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

use super::rpc_error_code::RpcErrorCode;

#[derive(Debug, PartialEq, Serialize_tuple, Deserialize_tuple, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CallError {
    pub(super) message_id: i32,
    pub unique_id: String,
    pub error_code: RpcErrorCode,
    pub error_description: String,
    pub error_details: BTreeMap<String, Value>,
}

impl CallError {
    #[must_use]
    pub fn new(
        unique_id: String,
        error_code: RpcErrorCode,
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

    #[must_use]
    pub fn not_implemented(unique_id: String, error_description: String) -> Self {
        Self::new(
            unique_id,
            RpcErrorCode::NotImplemented,
            error_description,
            BTreeMap::new(),
        )
    }

    #[must_use]
    pub fn not_supported(unique_id: String, error_description: String) -> Self {
        Self::new(
            unique_id,
            RpcErrorCode::NotSupported,
            error_description,
            BTreeMap::new(),
        )
    }

    #[must_use]
    pub fn format_violation(unique_id: String, error_description: String) -> Self {
        Self::new(
            unique_id,
            RpcErrorCode::FormatViolation,
            error_description,
            BTreeMap::new(),
        )
    }

    #[must_use]
    pub fn property_constraint_violation(unique_id: String, error_description: String) -> Self {
        Self::new(
            unique_id,
            RpcErrorCode::PropertyConstraintViolation,
            error_description,
            BTreeMap::new(),
        )
    }

    #[must_use]
    pub fn occurrence_constraint_violation(unique_id: String, error_description: String) -> Self {
        Self::new(
            unique_id,
            RpcErrorCode::OccurrenceConstraintViolation,
            error_description,
            BTreeMap::new(),
        )
    }
}
