//! `TariffFixedPriceType`
use super::CustomDataType;
use super::TariffConditionsFixedType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TariffFixedPriceType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub conditions: Option<TariffConditionsFixedType>,
    pub price_fixed: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
