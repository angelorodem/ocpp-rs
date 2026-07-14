//! VoltageParamsType
use crate::v21::enumerations::PowerDuringCessationEnumType;
use serde::{Deserialize, Serialize};
use super::CustomDataType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VoltageParamsType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub hv10_min_mean_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub hv10_min_mean_trip_delay: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub power_during_cessation: Option<PowerDuringCessationEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
