//! OCPP 2.1 PullDynamicScheduleUpdate request/response payloads.

use crate::v21::datatypes::ChargingScheduleUpdateType;
use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::StatusInfoType;
use crate::v21::enumerations::ChargingProfileStatusEnumType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PullDynamicScheduleUpdateRequest {
    pub charging_profile_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PullDynamicScheduleUpdateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub schedule_update: Option<ChargingScheduleUpdateType>,
    pub status: ChargingProfileStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub status_info: Option<StatusInfoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
