//! TaxRateType
use alloc::string::String;
use serde::{Deserialize, Serialize};
use super::CustomDataType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TaxRateType {
    #[serde(rename = "type")]
    pub type_: String,
    pub tax: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stack: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
