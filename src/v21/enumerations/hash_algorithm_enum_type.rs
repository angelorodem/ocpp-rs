//! `HashAlgorithmEnumType`
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HashAlgorithmEnumType {
    #[serde(rename = "SHA256")]
    SHA256,
    #[serde(rename = "SHA384")]
    SHA384,
    #[serde(rename = "SHA512")]
    SHA512,
}
