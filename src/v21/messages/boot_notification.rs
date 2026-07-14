//! OCPP 2.1 BootNotification request/response payloads.

use alloc::string::String;
use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::DateTimeWrapper;
use crate::v21::datatypes::StatusInfoType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BootReasonEnumType {
    #[serde(rename = "ApplicationReset")]
    ApplicationReset,
    #[serde(rename = "FirmwareUpdate")]
    FirmwareUpdate,
    #[serde(rename = "LocalReset")]
    LocalReset,
    #[serde(rename = "PowerUp")]
    PowerUp,
    #[serde(rename = "RemoteReset")]
    RemoteReset,
    #[serde(rename = "ScheduledReset")]
    ScheduledReset,
    #[serde(rename = "Triggered")]
    Triggered,
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Watchdog")]
    Watchdog,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ModemType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub iccid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub imsi: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChargingStationType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub serial_number: Option<String>,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub modem: Option<ModemType>,
    pub vendor_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub firmware_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RegistrationStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Rejected")]
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BootNotificationRequest {
    pub charging_station: ChargingStationType,
    pub reason: BootReasonEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BootNotificationResponse {
    #[serde(with = "crate::v21::utils::rfc3339_date_time")]
    pub current_time: DateTimeWrapper,
    pub interval: i32,
    pub status: RegistrationStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub status_info: Option<StatusInfoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
