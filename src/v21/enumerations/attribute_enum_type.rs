//! `AttributeEnumType`
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AttributeEnumType {
    #[serde(rename = "Actual")]
    Actual,
    #[serde(rename = "Target")]
    Target,
    #[serde(rename = "MinSet")]
    MinSet,
    #[serde(rename = "MaxSet")]
    MaxSet,
}
