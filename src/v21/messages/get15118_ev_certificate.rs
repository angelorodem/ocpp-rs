//! OCPP 2.1 `Get15118EVCertificate` request/response payloads.

use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::StatusInfoType;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CertificateActionEnumType {
    #[serde(rename = "Install")]
    Install,
    #[serde(rename = "Update")]
    Update,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Iso15118EVCertificateStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Failed")]
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Get15118EVCertificateRequest {
    pub iso15118_schema_version: String,
    pub action: CertificateActionEnumType,
    pub exi_request: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub maximum_contract_certificate_chains: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub prioritized_emai_ds: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Get15118EVCertificateResponse {
    pub status: Iso15118EVCertificateStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub status_info: Option<StatusInfoType>,
    pub exi_response: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub remaining_contracts: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
