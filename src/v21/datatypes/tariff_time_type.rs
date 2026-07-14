//! `TariffTimeType`
use super::CustomDataType;
use super::TariffTimePriceType;
use super::TaxRateType;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TariffTimeType {
    pub prices: Vec<TariffTimePriceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tax_rates: Option<Vec<TaxRateType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
