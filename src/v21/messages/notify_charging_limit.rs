//! OCPP 2.1 NotifyChargingLimit request/response payloads.

use crate::v21::datatypes::ChargingScheduleType;
use crate::v21::datatypes::CustomDataType;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChargingLimitType {
    pub charging_limit_source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub is_local_generation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub is_grid_critical: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NotifyChargingLimitRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub charging_schedule: Option<Vec<ChargingScheduleType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub evse_id: Option<i32>,
    pub charging_limit: ChargingLimitType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NotifyChargingLimitResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
