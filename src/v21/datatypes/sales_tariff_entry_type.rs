//! SalesTariffEntryType
use super::ConsumptionCostType;
use super::CustomDataType;
use super::RelativeTimeIntervalType;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SalesTariffEntryType {
    pub relative_time_interval: RelativeTimeIntervalType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub e_price_level: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub consumption_cost: Option<Vec<ConsumptionCostType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
