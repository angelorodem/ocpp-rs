//! `TariffConditionsType`
use super::CustomDataType;
use crate::v21::enumerations::DayOfWeekEnumType;
use crate::v21::enumerations::EvseKindEnumType;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TariffConditionsType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub start_time_of_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub end_time_of_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub day_of_week: Option<Vec<DayOfWeekEnumType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub valid_from_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub valid_to_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub evse_kind: Option<EvseKindEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub min_energy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_energy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub min_current: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_current: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub min_power: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_power: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub min_time: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_time: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub min_charging_time: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_charging_time: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub min_idle_time: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_idle_time: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
