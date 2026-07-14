//! OCPP 2.1 `ClearedChargingLimit` request/response payloads.

use crate::v21::datatypes::CustomDataType;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ClearedChargingLimitRequest {
    pub charging_limit_source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub evse_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ClearedChargingLimitResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
