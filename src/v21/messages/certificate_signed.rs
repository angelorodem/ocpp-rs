//! OCPP 2.1 `CertificateSigned` request/response payloads.

use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::StatusInfoType;
use crate::v21::enumerations::CertificateSigningUseEnumType;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CertificateSignedStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CertificateSignedRequest {
    pub certificate_chain: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub certificate_type: Option<CertificateSigningUseEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub request_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CertificateSignedResponse {
    pub status: CertificateSignedStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub status_info: Option<StatusInfoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
