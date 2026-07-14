//! OCPP 2.1 StatusNotification request/response payloads.

use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::DateTimeWrapper;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ConnectorStatusEnumType {
    #[serde(rename = "Available")]
    Available,
    #[serde(rename = "Occupied")]
    Occupied,
    #[serde(rename = "Reserved")]
    Reserved,
    #[serde(rename = "Unavailable")]
    Unavailable,
    #[serde(rename = "Faulted")]
    Faulted,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StatusNotificationRequest {
    #[serde(with = "crate::v21::utils::rfc3339_date_time")]
    pub timestamp: DateTimeWrapper,
    pub connector_status: ConnectorStatusEnumType,
    pub evse_id: i32,
    pub connector_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StatusNotificationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
