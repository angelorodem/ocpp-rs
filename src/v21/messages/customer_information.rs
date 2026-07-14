//! OCPP 2.1 `CustomerInformation` request/response payloads.

use crate::v21::datatypes::CertificateHashDataType;
use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::IdTokenType;
use crate::v21::datatypes::StatusInfoType;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CustomerInformationStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "Invalid")]
    Invalid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CustomerInformationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub customer_certificate: Option<CertificateHashDataType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub id_token: Option<IdTokenType>,
    pub request_id: i32,
    pub report: bool,
    pub clear: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub customer_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CustomerInformationResponse {
    pub status: CustomerInformationStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub status_info: Option<StatusInfoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
