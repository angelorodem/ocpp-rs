use alloc::{collections::btree_map::BTreeMap, string::String};
use serde_json::Value;
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

use super::rpc_error_code::RpcErrorCode;

#[derive(Debug, PartialEq, Eq, Serialize_tuple, Deserialize_tuple, Clone)]
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
    pub const fn new(
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
    pub const fn not_implemented(unique_id: String, error_description: String) -> Self {
        Self::new(
            unique_id,
            RpcErrorCode::NotImplemented,
            error_description,
            BTreeMap::new(),
        )
    }

    #[must_use]
    pub const fn formation_violation(unique_id: String, error_description: String) -> Self {
        Self::new(
            unique_id,
            RpcErrorCode::FormationViolation,
            error_description,
            BTreeMap::new(),
        )
    }

    #[must_use]
    pub const fn property_constraint_violation(
        unique_id: String,
        error_description: String,
    ) -> Self {
        Self::new(
            unique_id,
            RpcErrorCode::PropertyConstraintViolation,
            error_description,
            BTreeMap::new(),
        )
    }

    #[must_use]
    pub const fn occurence_constraint_violation(
        unique_id: String,
        error_description: String,
    ) -> Self {
        Self::new(
            unique_id,
            RpcErrorCode::OccurenceConstraintViolation,
            error_description,
            BTreeMap::new(),
        )
    }
}
