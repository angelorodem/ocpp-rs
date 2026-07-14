//! `ChargingProfileType`
use super::ChargingScheduleType;
use super::CustomDataType;
use super::DateTimeWrapper;
use crate::v21::enumerations::ChargingProfileKindEnumType;
use crate::v21::enumerations::ChargingProfilePurposeEnumType;
use crate::v21::enumerations::RecurrencyKindEnumType;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChargingProfileType {
    pub id: i32,
    pub stack_level: i32,
    pub charging_profile_purpose: ChargingProfilePurposeEnumType,
    pub charging_profile_kind: ChargingProfileKindEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub recurrency_kind: Option<RecurrencyKindEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::v21::utils::rfc3339_date_time_optional")]
    pub valid_from: Option<DateTimeWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::v21::utils::rfc3339_date_time_optional")]
    pub valid_to: Option<DateTimeWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub transaction_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_offline_duration: Option<i32>,
    pub charging_schedule: Vec<ChargingScheduleType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub invalid_after_offline_duration: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub dyn_update_interval: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::v21::utils::rfc3339_date_time_optional")]
    pub dyn_update_time: Option<DateTimeWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub price_schedule_signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
