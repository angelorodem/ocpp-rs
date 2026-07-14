//! OCPP 2.1 NotifyMonitoringReport request/response payloads.

use crate::v21::datatypes::ComponentType;
use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::DateTimeWrapper;
use crate::v21::datatypes::VariableType;
use crate::v21::enumerations::EventNotificationEnumType;
use crate::v21::enumerations::MonitorEnumType;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VariableMonitoringType {
    pub id: i32,
    pub transaction: bool,
    pub value: f64,
    #[serde(rename = "type")]
    pub type_: MonitorEnumType,
    pub severity: i32,
    pub event_notification_type: EventNotificationEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MonitoringDataType {
    pub component: ComponentType,
    pub variable: VariableType,
    pub variable_monitoring: Vec<VariableMonitoringType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NotifyMonitoringReportRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub monitor: Option<Vec<MonitoringDataType>>,
    pub request_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tbc: Option<bool>,
    pub seq_no: i32,
    #[serde(with = "crate::v21::utils::rfc3339_date_time")]
    pub generated_at: DateTimeWrapper,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NotifyMonitoringReportResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
