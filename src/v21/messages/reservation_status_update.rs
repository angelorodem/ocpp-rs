//! OCPP 2.1 `ReservationStatusUpdate` request/response payloads.

use crate::v21::datatypes::CustomDataType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReservationUpdateStatusEnumType {
    #[serde(rename = "Expired")]
    Expired,
    #[serde(rename = "Removed")]
    Removed,
    #[serde(rename = "NoTransaction")]
    NoTransaction,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReservationStatusUpdateRequest {
    pub reservation_id: i32,
    pub reservation_update_status: ReservationUpdateStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReservationStatusUpdateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
