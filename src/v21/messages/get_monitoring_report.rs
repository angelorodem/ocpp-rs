//! OCPP 2.1 `GetMonitoringReport` request/response payloads.

use crate::v21::datatypes::ComponentVariableType;
use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::StatusInfoType;
use crate::v21::enumerations::GenericDeviceModelStatusEnumType;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MonitoringCriterionEnumType {
    #[serde(rename = "ThresholdMonitoring")]
    ThresholdMonitoring,
    #[serde(rename = "DeltaMonitoring")]
    DeltaMonitoring,
    #[serde(rename = "PeriodicMonitoring")]
    PeriodicMonitoring,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GetMonitoringReportRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub component_variable: Option<Vec<ComponentVariableType>>,
    pub request_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub monitoring_criteria: Option<Vec<MonitoringCriterionEnumType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GetMonitoringReportResponse {
    pub status: GenericDeviceModelStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub status_info: Option<StatusInfoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
