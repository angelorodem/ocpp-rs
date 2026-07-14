//! `TaxRuleType`
use super::CustomDataType;
use super::RationalNumberType;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TaxRuleType {
    pub tax_rule_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tax_rule_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tax_included_in_price: Option<bool>,
    pub applies_to_energy_fee: bool,
    pub applies_to_parking_fee: bool,
    pub applies_to_overstay_fee: bool,
    pub applies_to_minimum_maximum_cost: bool,
    pub tax_rate: RationalNumberType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
