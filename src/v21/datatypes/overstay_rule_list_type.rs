//! OverstayRuleListType
use super::CustomDataType;
use super::OverstayRuleType;
use super::RationalNumberType;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OverstayRuleListType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub overstay_power_threshold: Option<RationalNumberType>,
    pub overstay_rule: Vec<OverstayRuleType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub overstay_time_threshold: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
