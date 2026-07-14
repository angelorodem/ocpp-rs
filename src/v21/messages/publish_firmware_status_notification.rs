//! OCPP 2.1 `PublishFirmwareStatusNotification` request/response payloads.

use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::StatusInfoType;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PublishFirmwareStatusEnumType {
    #[serde(rename = "Idle")]
    Idle,
    #[serde(rename = "DownloadScheduled")]
    DownloadScheduled,
    #[serde(rename = "Downloading")]
    Downloading,
    #[serde(rename = "Downloaded")]
    Downloaded,
    #[serde(rename = "Published")]
    Published,
    #[serde(rename = "DownloadFailed")]
    DownloadFailed,
    #[serde(rename = "DownloadPaused")]
    DownloadPaused,
    #[serde(rename = "InvalidChecksum")]
    InvalidChecksum,
    #[serde(rename = "ChecksumVerified")]
    ChecksumVerified,
    #[serde(rename = "PublishFailed")]
    PublishFailed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PublishFirmwareStatusNotificationRequest {
    pub status: PublishFirmwareStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub location: Option<Vec<String>>,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PublishFirmwareStatusNotificationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
