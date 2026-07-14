//! `LimitMaxDischargeType`
use super::CustomDataType;
use super::DERCurveType;
use super::DateTimeWrapper;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LimitMaxDischargeType {
    pub priority: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub pct_max_discharge_power: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub power_monitoring_must_trip: Option<DERCurveType>,
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
