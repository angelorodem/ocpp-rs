//! OCPP 2.1 UnpublishFirmware request/response payloads.

use crate::v21::datatypes::CustomDataType;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UnpublishFirmwareStatusEnumType {
    #[serde(rename = "DownloadOngoing")]
    DownloadOngoing,
    #[serde(rename = "NoFirmware")]
    NoFirmware,
    #[serde(rename = "Unpublished")]
    Unpublished,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UnpublishFirmwareRequest {
    pub checksum: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UnpublishFirmwareResponse {
    pub status: UnpublishFirmwareStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
