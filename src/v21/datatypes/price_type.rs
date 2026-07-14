//! PriceType
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use super::CustomDataType;
use super::TaxRateType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PriceType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub excl_tax: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub incl_tax: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tax_rates: Option<Vec<TaxRateType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
