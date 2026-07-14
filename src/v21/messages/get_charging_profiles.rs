//! OCPP 2.1 `GetChargingProfiles` request/response payloads.

use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::StatusInfoType;
use crate::v21::enumerations::ChargingProfilePurposeEnumType;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChargingProfileCriterionType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub charging_profile_purpose: Option<ChargingProfilePurposeEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stack_level: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub charging_profile_id: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub charging_limit_source: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GetChargingProfileStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "NoProfiles")]
    NoProfiles,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GetChargingProfilesRequest {
    pub request_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub evse_id: Option<i32>,
    pub charging_profile: ChargingProfileCriterionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GetChargingProfilesResponse {
    pub status: GetChargingProfileStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub status_info: Option<StatusInfoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
