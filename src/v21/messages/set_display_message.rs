//! OCPP 2.1 `SetDisplayMessage` request/response payloads.

use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::MessageInfoType;
use crate::v21::datatypes::StatusInfoType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DisplayMessageStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "NotSupportedMessageFormat")]
    NotSupportedMessageFormat,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "NotSupportedPriority")]
    NotSupportedPriority,
    #[serde(rename = "NotSupportedState")]
    NotSupportedState,
    #[serde(rename = "UnknownTransaction")]
    UnknownTransaction,
    #[serde(rename = "LanguageNotSupported")]
    LanguageNotSupported,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetDisplayMessageRequest {
    pub message: MessageInfoType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetDisplayMessageResponse {
    pub status: DisplayMessageStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub status_info: Option<StatusInfoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
