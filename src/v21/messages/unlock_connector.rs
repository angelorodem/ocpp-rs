//! OCPP 2.1 `UnlockConnector` request/response payloads.

use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::StatusInfoType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UnlockStatusEnumType {
    #[serde(rename = "Unlocked")]
    Unlocked,
    #[serde(rename = "UnlockFailed")]
    UnlockFailed,
    #[serde(rename = "OngoingAuthorizedTransaction")]
    OngoingAuthorizedTransaction,
    #[serde(rename = "UnknownConnector")]
    UnknownConnector,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UnlockConnectorRequest {
    pub evse_id: i32,
    pub connector_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UnlockConnectorResponse {
    pub status: UnlockStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub status_info: Option<StatusInfoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
