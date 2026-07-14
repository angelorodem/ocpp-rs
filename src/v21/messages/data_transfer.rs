//! OCPP 2.1 `DataTransfer` request/response payloads.

use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::StatusInfoType;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DataTransferStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "UnknownMessageId")]
    UnknownMessageId,
    #[serde(rename = "UnknownVendorId")]
    UnknownVendorId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DataTransferRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub data: Option<serde_json::Value>,
    pub vendor_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DataTransferResponse {
    pub status: DataTransferStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub status_info: Option<StatusInfoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
