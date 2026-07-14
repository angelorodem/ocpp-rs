//! OCPP 2.1 SetVariableMonitoring request/response payloads.

use crate::v21::datatypes::ComponentType;
use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::PeriodicEventStreamParamsType;
use crate::v21::datatypes::StatusInfoType;
use crate::v21::datatypes::VariableType;
use crate::v21::enumerations::MonitorEnumType;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetMonitoringDataType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub periodic_event_stream: Option<PeriodicEventStreamParamsType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub transaction: Option<bool>,
    pub value: f64,
    #[serde(rename = "type")]
    pub type_: MonitorEnumType,
    pub severity: i32,
    pub component: ComponentType,
    pub variable: VariableType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SetMonitoringStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "UnknownComponent")]
    UnknownComponent,
    #[serde(rename = "UnknownVariable")]
    UnknownVariable,
    #[serde(rename = "UnsupportedMonitorType")]
    UnsupportedMonitorType,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "Duplicate")]
    Duplicate,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetMonitoringResultType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub status_info: Option<StatusInfoType>,
    pub status: SetMonitoringStatusEnumType,
    #[serde(rename = "type")]
    pub type_: MonitorEnumType,
    pub component: ComponentType,
    pub variable: VariableType,
    pub severity: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetVariableMonitoringRequest {
    pub set_monitoring_data: Vec<SetMonitoringDataType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetVariableMonitoringResponse {
    pub set_monitoring_result: Vec<SetMonitoringResultType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
