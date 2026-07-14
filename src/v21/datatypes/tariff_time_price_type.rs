//! TariffTimePriceType
use super::CustomDataType;
use super::TariffConditionsType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TariffTimePriceType {
    pub price_minute: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub conditions: Option<TariffConditionsType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
