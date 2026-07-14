//! CertificateSigningUseEnumType
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CertificateSigningUseEnumType {
    #[serde(rename = "ChargingStationCertificate")]
    ChargingStationCertificate,
    #[serde(rename = "V2GCertificate")]
    V2GCertificate,
    #[serde(rename = "V2G20Certificate")]
    V2G20Certificate,
}
