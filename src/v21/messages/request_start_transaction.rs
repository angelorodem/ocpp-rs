//! OCPP 2.1 `RequestStartTransaction` request/response payloads.

use crate::v21::datatypes::ChargingProfileType;
use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::IdTokenType;
use crate::v21::datatypes::StatusInfoType;
use crate::v21::enumerations::RequestStartStopStatusEnumType;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RequestStartTransactionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub evse_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub group_id_token: Option<IdTokenType>,
    pub id_token: IdTokenType,
    pub remote_start_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub charging_profile: Option<ChargingProfileType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RequestStartTransactionResponse {
    pub status: RequestStartStopStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub status_info: Option<StatusInfoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub transaction_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
