//! OCPP 2.1 NotifyEvent request/response payloads.

use alloc::string::String;
use alloc::vec::Vec;
use crate::v21::datatypes::ComponentType;
use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::DateTimeWrapper;
use crate::v21::datatypes::VariableType;
use crate::v21::enumerations::EventNotificationEnumType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EventTriggerEnumType {
    #[serde(rename = "Alerting")]
    Alerting,
    #[serde(rename = "Delta")]
    Delta,
    #[serde(rename = "Periodic")]
    Periodic,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EventDataType {
    pub event_id: i32,
    #[serde(with = "crate::v21::utils::rfc3339_date_time")]
    pub timestamp: DateTimeWrapper,
    pub trigger: EventTriggerEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cause: Option<i32>,
    pub actual_value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tech_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tech_info: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cleared: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub transaction_id: Option<String>,
    pub component: ComponentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub variable_monitoring_id: Option<i32>,
    pub event_notification_type: EventNotificationEnumType,
    pub variable: VariableType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub severity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NotifyEventRequest {
    #[serde(with = "crate::v21::utils::rfc3339_date_time")]
    pub generated_at: DateTimeWrapper,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tbc: Option<bool>,
    pub seq_no: i32,
    pub event_data: Vec<EventDataType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NotifyEventResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
