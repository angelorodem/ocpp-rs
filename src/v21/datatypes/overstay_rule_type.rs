//! `OverstayRuleType`
use super::CustomDataType;
use super::RationalNumberType;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OverstayRuleType {
    pub overstay_fee: RationalNumberType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub overstay_rule_description: Option<String>,
    pub start_time: i32,
    pub overstay_fee_period: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
