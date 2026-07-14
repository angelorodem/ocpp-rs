//! `GetCertificateIdUseEnumType`
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GetCertificateIdUseEnumType {
    #[serde(rename = "V2GRootCertificate")]
    V2GRootCertificate,
    #[serde(rename = "MORootCertificate")]
    MORootCertificate,
    #[serde(rename = "CSMSRootCertificate")]
    CSMSRootCertificate,
    #[serde(rename = "V2GCertificateChain")]
    V2GCertificateChain,
    #[serde(rename = "ManufacturerRootCertificate")]
    ManufacturerRootCertificate,
    #[serde(rename = "OEMRootCertificate")]
    OEMRootCertificate,
}
