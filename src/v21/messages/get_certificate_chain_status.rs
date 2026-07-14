//! OCPP 2.1 `GetCertificateChainStatus` request/response payloads.

use crate::v21::datatypes::CertificateHashDataType;
use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::DateTimeWrapper;
use crate::v21::enumerations::CertificateStatusSourceEnumType;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CertificateStatusEnumType {
    #[serde(rename = "Good")]
    Good,
    #[serde(rename = "Revoked")]
    Revoked,
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Failed")]
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CertificateStatusRequestInfoType {
    pub certificate_hash_data: CertificateHashDataType,
    pub source: CertificateStatusSourceEnumType,
    pub urls: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CertificateStatusType {
    pub certificate_hash_data: CertificateHashDataType,
    pub source: CertificateStatusSourceEnumType,
    pub status: CertificateStatusEnumType,
    #[serde(with = "crate::v21::utils::rfc3339_date_time")]
    pub next_update: DateTimeWrapper,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GetCertificateChainStatusRequest {
    pub certificate_status_requests: Vec<CertificateStatusRequestInfoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GetCertificateChainStatusResponse {
    pub certificate_status: Vec<CertificateStatusType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
