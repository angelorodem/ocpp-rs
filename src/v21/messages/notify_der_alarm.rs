//! OCPP 2.1 `NotifyDERAlarm` request/response payloads.

use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::DateTimeWrapper;
use crate::v21::enumerations::DERControlEnumType;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GridEventFaultEnumType {
    #[serde(rename = "CurrentImbalance")]
    CurrentImbalance,
    #[serde(rename = "LocalEmergency")]
    LocalEmergency,
    #[serde(rename = "LowInputPower")]
    LowInputPower,
    #[serde(rename = "OverCurrent")]
    OverCurrent,
    #[serde(rename = "OverFrequency")]
    OverFrequency,
    #[serde(rename = "OverVoltage")]
    OverVoltage,
    #[serde(rename = "PhaseRotation")]
    PhaseRotation,
    #[serde(rename = "RemoteEmergency")]
    RemoteEmergency,
    #[serde(rename = "UnderFrequency")]
    UnderFrequency,
    #[serde(rename = "UnderVoltage")]
    UnderVoltage,
    #[serde(rename = "VoltageImbalance")]
    VoltageImbalance,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NotifyDERAlarmRequest {
    pub control_type: DERControlEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub grid_event_fault: Option<GridEventFaultEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub alarm_ended: Option<bool>,
    #[serde(with = "crate::v21::utils::rfc3339_date_time")]
    pub timestamp: DateTimeWrapper,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub extra_info: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NotifyDERAlarmResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
