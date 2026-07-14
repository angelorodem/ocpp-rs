//! OCPP 2.1 `DeleteCertificate` request/response payloads.

use crate::v21::datatypes::CertificateHashDataType;
use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::StatusInfoType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DeleteCertificateStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Failed")]
    Failed,
    #[serde(rename = "NotFound")]
    NotFound,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeleteCertificateRequest {
    pub certificate_hash_data: CertificateHashDataType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeleteCertificateResponse {
    pub status: DeleteCertificateStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub status_info: Option<StatusInfoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
