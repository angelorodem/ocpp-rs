//! FreqDroopType
use serde::{Deserialize, Serialize};
use super::CustomDataType;
use super::DateTimeWrapper;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FreqDroopType {
    pub priority: i32,
    pub over_freq: f64,
    pub under_freq: f64,
    pub over_droop: f64,
    pub under_droop: f64,
    pub response_time: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::v21::utils::rfc3339_date_time_optional")]
    pub start_time: Option<DateTimeWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
