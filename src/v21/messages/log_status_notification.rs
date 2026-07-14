//! OCPP 2.1 LogStatusNotification request/response payloads.

use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::StatusInfoType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UploadLogStatusEnumType {
    #[serde(rename = "BadMessage")]
    BadMessage,
    #[serde(rename = "Idle")]
    Idle,
    #[serde(rename = "NotSupportedOperation")]
    NotSupportedOperation,
    #[serde(rename = "PermissionDenied")]
    PermissionDenied,
    #[serde(rename = "Uploaded")]
    Uploaded,
    #[serde(rename = "UploadFailure")]
    UploadFailure,
    #[serde(rename = "Uploading")]
    Uploading,
    #[serde(rename = "AcceptedCanceled")]
    AcceptedCanceled,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LogStatusNotificationRequest {
    pub status: UploadLogStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub request_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub status_info: Option<StatusInfoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LogStatusNotificationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
