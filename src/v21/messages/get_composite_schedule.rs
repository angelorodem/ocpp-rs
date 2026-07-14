//! OCPP 2.1 GetCompositeSchedule request/response payloads.

use crate::v21::datatypes::ChargingSchedulePeriodType;
use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::DateTimeWrapper;
use crate::v21::datatypes::StatusInfoType;
use crate::v21::enumerations::ChargingRateUnitEnumType;
use crate::v21::enumerations::GenericStatusEnumType;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CompositeScheduleType {
    pub evse_id: i32,
    pub duration: i32,
    #[serde(with = "crate::v21::utils::rfc3339_date_time")]
    pub schedule_start: DateTimeWrapper,
    pub charging_rate_unit: ChargingRateUnitEnumType,
    pub charging_schedule_period: Vec<ChargingSchedulePeriodType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GetCompositeScheduleRequest {
    pub duration: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub charging_rate_unit: Option<ChargingRateUnitEnumType>,
    pub evse_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GetCompositeScheduleResponse {
    pub status: GenericStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub status_info: Option<StatusInfoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub schedule: Option<CompositeScheduleType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
