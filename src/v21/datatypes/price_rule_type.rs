//! PriceRuleType
use super::CustomDataType;
use super::RationalNumberType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PriceRuleType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub parking_fee_period: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub carbon_dioxide_emission: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub renewable_generation_percentage: Option<i32>,
    pub energy_fee: RationalNumberType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub parking_fee: Option<RationalNumberType>,
    pub power_range_start: RationalNumberType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
