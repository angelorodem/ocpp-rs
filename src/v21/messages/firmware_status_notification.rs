//! OCPP 2.1 FirmwareStatusNotification request/response payloads.

use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::StatusInfoType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FirmwareStatusEnumType {
    #[serde(rename = "Downloaded")]
    Downloaded,
    #[serde(rename = "DownloadFailed")]
    DownloadFailed,
    #[serde(rename = "Downloading")]
    Downloading,
    #[serde(rename = "DownloadScheduled")]
    DownloadScheduled,
    #[serde(rename = "DownloadPaused")]
    DownloadPaused,
    #[serde(rename = "Idle")]
    Idle,
    #[serde(rename = "InstallationFailed")]
    InstallationFailed,
    #[serde(rename = "Installing")]
    Installing,
    #[serde(rename = "Installed")]
    Installed,
    #[serde(rename = "InstallRebooting")]
    InstallRebooting,
    #[serde(rename = "InstallScheduled")]
    InstallScheduled,
    #[serde(rename = "InstallVerificationFailed")]
    InstallVerificationFailed,
    #[serde(rename = "InvalidSignature")]
    InvalidSignature,
    #[serde(rename = "SignatureVerified")]
    SignatureVerified,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FirmwareStatusNotificationRequest {
    pub status: FirmwareStatusEnumType,
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
pub struct FirmwareStatusNotificationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
