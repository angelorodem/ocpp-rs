//! OCPP 2.1 SecurityEventNotification request/response payloads.

use alloc::string::String;
use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::DateTimeWrapper;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecurityEventNotificationRequest {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(with = "crate::v21::utils::rfc3339_date_time")]
    pub timestamp: DateTimeWrapper,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tech_info: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecurityEventNotificationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
