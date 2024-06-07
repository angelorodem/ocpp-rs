use super::enums::{
    AvailabilityType, CertificateUse, ChargePointErrorCode, ChargePointStatus,
    ChargingProfilePurposeType, ChargingRateUnitType, DiagnosticsStatus, FirmwareStatus, Log,
    MessageTrigger, Reason, ResetType, UpdateType, UploadLogStatus,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::EnumString;
use serde_tuple::*;
use super::utils::{iso8601_date_time_optional,iso8601_date_time};


// Call action enum
#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Action {
    Authorize(Authorize),
    BootNotification(BootNotification),
    CancelReservation(CancelReservation),
    CertificateSigned(CertificateSigned),
    ChangeAvailability(ChangeAvailability),
    ChangeConfiguration(ChangeConfiguration),
    ClearCache(ClearCache),
    ClearChargingProfile(ClearChargingProfile),
    DataTransfer(DataTransfer),
    DeleteCertificate(DeleteCertificate),
    DiagnosticsStatusNotification(DiagnosticsStatusNotification),
    ExtendedTriggerMessage(ExtendedTriggerMessage),
    FirmwareStatusNotification(FirmwareStatusNotification),
    GetCompositeSchedule(GetCompositeSchedule),
    GetConfiguration(GetConfiguration),
    GetDiagnostics(GetDiagnostics),
    GetInstalledCertificateIds(GetInstalledCertificateIds),
    GetLocalListVersion(GetLocalListVersion),
    GetLog(GetLog),
    Heartbeat(Heartbeat),
    InstallCertificate(InstallCertificate),
    LogStatusNotification(LogStatusNotification),
    MeterValues(MeterValues),
    RemoteStartTransaction(RemoteStartTransaction),
    RemoteStopTransaction(RemoteStopTransaction),
    ReserveNow(ReserveNow),
    Reset(Reset),
    SecurityEventNotification(SecurityEventNotification),
    SendLocalList(SendLocalList),
    SetChargingProfile(SetChargingProfile),
    SignCertificate(SignCertificate),
    SignedFirmwareStatusNotification(SignedFirmwareStatusNotification),
    SignedUpdateFirmware(SignedUpdateFirmware),
    StartTransaction(StartTransaction),
    StatusNotification(StatusNotification),
    StopTransaction(StopTransaction),
    TriggerMessage(TriggerMessage),
    UnlockConnector(UnlockConnector),
    UpdateFirmware(UpdateFirmware),
}

#[derive(Debug, PartialEq, Eq, Serialize_tuple, Deserialize_tuple, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Call {
    pub unique_id: String,
    pub action: String,
    pub payload: Action,
}

//////////////////////////// Call structs ////////////////////////////

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelReservation {
    pub reservation_id: i32,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSigned {
    pub certificate_chain: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAvailability {
    pub connector_id: i32,
    #[serde(rename = "type")]
    pub availability_type: AvailabilityType,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChangeConfiguration {
    pub key: String,
    pub value: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearCache {}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_purpose: Option<ChargingProfilePurposeType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_level: Option<i32>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCertificate {
    pub certificate_hash_data: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedTriggerMessage {
    pub requested_message: MessageTrigger,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<i32>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCompositeSchedule {
    pub connector_id: i32,
    pub duration: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_rate_unit: Option<ChargingRateUnitType>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDiagnostics {
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "iso8601_date_time_optional")]
    pub start_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "iso8601_date_time_optional")]
    pub stop_time: Option<DateTime<Utc>>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetInstalledCertificateIds {
    pub certificate_type: CertificateUse,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLocalListVersion {}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLog {
    pub log: HashMap<String, String>,
    pub log_type: Log,
    pub request_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i32>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct InstallCertificate {
    pub certificate_type: CertificateUse,
    pub certificate: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStartTransaction {
    pub id_tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile: Option<HashMap<String, String>>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStopTransaction {
    pub transaction_id: i32,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReserveNow {
    pub connector_id: i32,
    pub expiry_date: String,
    pub id_tag: String,
    pub reservation_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id_tag: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Reset {
    #[serde(rename = "type")]
    pub reset_type: ResetType,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SendLocalList {
    pub list_version: i32,
    pub update_type: UpdateType,
    pub local_authorization_list: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetChargingProfile {
    pub connector_id: i32,
    pub cs_charging_profiles: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SignedUpdateFirmware {
    pub request_id: i32,
    pub firmware: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i32>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TriggerMessage {
    pub requested_message: MessageTrigger,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<i32>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct UnlockConnector {
    pub connector_id: i32,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFirmware {
    pub location: String,
    pub retrieve_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i32>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Authorize {
    pub id_tag: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BootNotification {
    pub charge_point_model: String,
    pub charge_point_vendor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_box_serial_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_point_serial_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iccid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imsi: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_serial_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_type: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticsStatusNotification {
    pub status: DiagnosticsStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareStatusNotification {
    pub status: FirmwareStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Heartbeat {}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LogStatusNotification {
    pub status: UploadLogStatus,
    pub request_id: i32,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MeterValues {
    pub connector_id: i32,
    pub meter_value: Vec<String>,
    pub transaction_id: Option<i32>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SecurityEventNotification {
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(with = "iso8601_date_time")]
    pub timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tech_info: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SignCertificate {
    pub csr: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SignedFirmwareStatusNotification {
    pub status: FirmwareStatus,
    pub request_id: i32,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartTransaction {
    pub connector_id: i32,
    pub id_tag: String,
    pub meter_start: i32,
    #[serde(with = "iso8601_date_time")]
    pub timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<i32>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct StopTransaction {
    pub meter_stop: i32,
    #[serde(with = "iso8601_date_time")]
    pub timestamp: DateTime<Utc>,
    pub transaction_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Reason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_data: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct StatusNotification {
    pub connector_id: i32,
    pub error_code: ChargePointErrorCode,
    pub status: ChargePointStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "iso8601_date_time_optional")]
    pub timestamp: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_error_code: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DataTransfer {
    pub vendor_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}
