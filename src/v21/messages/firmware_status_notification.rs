//! OCPP 2.1 `FirmwareStatusNotification` request/response payloads.

use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::StatusInfoType;
use serde::{Deserialize, Serialize};

crate::lenient_str_enum! {
    pub enum FirmwareStatusEnumType {
        Downloaded,
        DownloadFailed,
        Downloading,
        DownloadScheduled,
        DownloadPaused,
        Idle,
        InstallationFailed,
        Installing,
        Installed,
        InstallRebooting,
        InstallScheduled,
        InstallVerificationFailed,
        InvalidSignature,
        SignatureVerified,
    }
    @unknown Unknown
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FirmwareStatusNotificationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
