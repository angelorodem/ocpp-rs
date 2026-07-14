//! PriceLevelScheduleEntryType
use serde::{Deserialize, Serialize};
use super::CustomDataType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PriceLevelScheduleEntryType {
    pub duration: i32,
    pub price_level: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
