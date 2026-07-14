//! TariffEnergyType
use super::CustomDataType;
use super::TariffEnergyPriceType;
use super::TaxRateType;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TariffEnergyType {
    pub prices: Vec<TariffEnergyPriceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tax_rates: Option<Vec<TaxRateType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
