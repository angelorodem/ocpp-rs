//! OCPP 2.1 `TriggerMessage` request/response payloads.

use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::EVSEType;
use crate::v21::datatypes::StatusInfoType;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MessageTriggerEnumType {
    #[serde(rename = "BootNotification")]
    BootNotification,
    #[serde(rename = "LogStatusNotification")]
    LogStatusNotification,
    #[serde(rename = "FirmwareStatusNotification")]
    FirmwareStatusNotification,
    #[serde(rename = "Heartbeat")]
    Heartbeat,
    #[serde(rename = "MeterValues")]
    MeterValues,
    #[serde(rename = "SignChargingStationCertificate")]
    SignChargingStationCertificate,
    #[serde(rename = "SignV2GCertificate")]
    SignV2GCertificate,
    #[serde(rename = "SignV2G20Certificate")]
    SignV2G20Certificate,
    #[serde(rename = "StatusNotification")]
    StatusNotification,
    #[serde(rename = "TransactionEvent")]
    TransactionEvent,
    #[serde(rename = "SignCombinedCertificate")]
    SignCombinedCertificate,
    #[serde(rename = "PublishFirmwareStatusNotification")]
    PublishFirmwareStatusNotification,
    #[serde(rename = "CustomTrigger")]
    CustomTrigger,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TriggerMessageStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "NotImplemented")]
    NotImplemented,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TriggerMessageRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub evse: Option<EVSEType>,
    pub requested_message: MessageTriggerEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_trigger: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TriggerMessageResponse {
    pub status: TriggerMessageStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub status_info: Option<StatusInfoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
