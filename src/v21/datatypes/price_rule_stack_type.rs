//! PriceRuleStackType
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use super::CustomDataType;
use super::PriceRuleType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PriceRuleStackType {
    pub duration: i32,
    pub price_rule: Vec<PriceRuleType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
