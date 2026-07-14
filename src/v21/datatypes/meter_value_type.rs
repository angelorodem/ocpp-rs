//! MeterValueType
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use super::CustomDataType;
use super::DateTimeWrapper;
use super::SampledValueType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MeterValueType {
    pub sampled_value: Vec<SampledValueType>,
    #[serde(with = "crate::v21::utils::rfc3339_date_time")]
    pub timestamp: DateTimeWrapper,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
