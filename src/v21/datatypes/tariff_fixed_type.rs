//! TariffFixedType
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use super::CustomDataType;
use super::TariffFixedPriceType;
use super::TaxRateType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TariffFixedType {
    pub prices: Vec<TariffFixedPriceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tax_rates: Option<Vec<TaxRateType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
