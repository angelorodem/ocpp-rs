//! OCPP 2.1 ReportChargingProfiles request/response payloads.

use alloc::string::String;
use alloc::vec::Vec;
use crate::v21::datatypes::ChargingProfileType;
use crate::v21::datatypes::CustomDataType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReportChargingProfilesRequest {
    pub request_id: i32,
    pub charging_limit_source: String,
    pub charging_profile: Vec<ChargingProfileType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tbc: Option<bool>,
    pub evse_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReportChargingProfilesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
