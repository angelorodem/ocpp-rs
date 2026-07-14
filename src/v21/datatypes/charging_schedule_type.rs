//! `ChargingScheduleType`
use super::AbsolutePriceScheduleType;
use super::ChargingSchedulePeriodType;
use super::CustomDataType;
use super::DateTimeWrapper;
use super::LimitAtSoCType;
use super::PriceLevelScheduleType;
use super::SalesTariffType;
use crate::v21::enumerations::ChargingRateUnitEnumType;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChargingScheduleType {
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub limit_at_so_c: Option<LimitAtSoCType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::v21::utils::rfc3339_date_time_optional")]
    pub start_schedule: Option<DateTimeWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub duration: Option<i32>,
    pub charging_rate_unit: ChargingRateUnitEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub min_charging_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub power_tolerance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub signature_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub digest_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub use_local_time: Option<bool>,
    pub charging_schedule_period: Vec<ChargingSchedulePeriodType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub randomized_delay: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sales_tariff: Option<SalesTariffType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub absolute_price_schedule: Option<AbsolutePriceScheduleType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub price_level_schedule: Option<PriceLevelScheduleType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
