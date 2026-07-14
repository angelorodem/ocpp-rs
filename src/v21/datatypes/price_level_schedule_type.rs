//! `PriceLevelScheduleType`
use super::CustomDataType;
use super::DateTimeWrapper;
use super::PriceLevelScheduleEntryType;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PriceLevelScheduleType {
    pub price_level_schedule_entries: Vec<PriceLevelScheduleEntryType>,
    #[serde(with = "crate::v21::utils::rfc3339_date_time")]
    pub time_anchor: DateTimeWrapper,
    pub price_schedule_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub price_schedule_description: Option<String>,
    pub number_of_price_levels: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
