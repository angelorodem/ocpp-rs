//! OCPP 2.1 `UpdateFirmware` request/response payloads.

use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::DateTimeWrapper;
use crate::v21::datatypes::StatusInfoType;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FirmwareType {
    pub location: String,
    #[serde(with = "crate::v21::utils::rfc3339_date_time")]
    pub retrieve_date_time: DateTimeWrapper,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::v21::utils::rfc3339_date_time_optional")]
    pub install_date_time: Option<DateTimeWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub signing_certificate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UpdateFirmwareStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "AcceptedCanceled")]
    AcceptedCanceled,
    #[serde(rename = "InvalidCertificate")]
    InvalidCertificate,
    #[serde(rename = "RevokedCertificate")]
    RevokedCertificate,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UpdateFirmwareRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub retries: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub retry_interval: Option<i32>,
    pub request_id: i32,
    pub firmware: FirmwareType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UpdateFirmwareResponse {
    pub status: UpdateFirmwareStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub status_info: Option<StatusInfoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
