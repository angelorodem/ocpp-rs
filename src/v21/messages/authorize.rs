//! OCPP 2.1 Authorize request/response payloads.

use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::IdTokenInfoType;
use crate::v21::datatypes::IdTokenType;
use crate::v21::datatypes::OCSPRequestDataType;
use crate::v21::datatypes::TariffType;
use crate::v21::enumerations::EnergyTransferModeEnumType;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AuthorizeCertificateStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "SignatureError")]
    SignatureError,
    #[serde(rename = "CertificateExpired")]
    CertificateExpired,
    #[serde(rename = "CertificateRevoked")]
    CertificateRevoked,
    #[serde(rename = "NoCertificateAvailable")]
    NoCertificateAvailable,
    #[serde(rename = "CertChainError")]
    CertChainError,
    #[serde(rename = "ContractCancelled")]
    ContractCancelled,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AuthorizeRequest {
    pub id_token: IdTokenType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub certificate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub iso15118_certificate_hash_data: Option<Vec<OCSPRequestDataType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AuthorizeResponse {
    pub id_token_info: IdTokenInfoType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub certificate_status: Option<AuthorizeCertificateStatusEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub allowed_energy_transfer: Option<Vec<EnergyTransferModeEnumType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tariff: Option<TariffType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
