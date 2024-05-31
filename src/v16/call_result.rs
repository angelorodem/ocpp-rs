use super::enums::{
    AvailabilityStatus,
    CancelReservationStatus,
    CertificateSignedStatus,
    CertificateStatus,
    ChargingProfileStatus,
    ClearCacheStatus,
    ClearChargingProfileStatus,
    ConfigurationStatus,
    DataTransferStatus,
    DeleteCertificateStatus,
    GenericStatus,
    GetCompositeScheduleStatus,
    GetInstalledCertificateStatus,
    LogStatus,
    RegistrationStatus,
    RemoteStartStopStatus,
    ReservationStatus,
    ResetStatus,
    TriggerMessageStatus,
    UnlockStatus,
    UpdateFirmwareStatus,
    UpdateStatus,
};
use super::data_types::IdTagInfo;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::EnumString;

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Result {
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
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CallResult	 {
    pub unique_id: String,
    pub payload: Result,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Authorize {
    pub id_tag_info: IdTagInfo,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BootNotification {
    pub current_time: String,
    pub interval: i32,
    pub status: RegistrationStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticsStatusNotification {}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareStatusNotification {}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Heartbeat {
    pub current_time: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LogStatusNotification {}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SecurityEventNotification {}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SignCertificate {
    pub status: GenericStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MeterValues {}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartTransaction {
    pub transaction_id: i32,
    pub id_tag_info: IdTagInfo,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct StatusNotification {}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct StopTransaction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_tag_info: Option<IdTagInfo>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelReservation {
    pub status: CancelReservationStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSigned {
    pub status: CertificateSignedStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAvailability {
    pub status: AvailabilityStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChangeConfiguration {
    pub status: ConfigurationStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearCache {
    pub status: ClearCacheStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfile {
    pub status: ClearChargingProfileStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCertificate {
    pub status: DeleteCertificateStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedTriggerMessage {
    pub status: TriggerMessageStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetInstalledCertificateIds {
    pub status: GetInstalledCertificateStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_hash_data: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCompositeSchedule {
    pub status: GetCompositeScheduleStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_schedule: Option<HashMap<String, String>>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_key: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_key: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDiagnostics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLocalListVersion {
    pub list_version: i32,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLog {
    pub status: LogStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct InstallCertificate {
    pub status: CertificateStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStartTransaction {
    pub status: RemoteStartStopStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStopTransaction {
    pub status: RemoteStartStopStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReserveNow {
    pub status: ReservationStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Reset {
    pub status: ResetStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SendLocalList {
    pub status: UpdateStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetChargingProfile {
    pub status: ChargingProfileStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SignedFirmwareStatusNotification {}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SignedUpdateFirmware {
    pub status: UpdateFirmwareStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TriggerMessage {
    pub status: TriggerMessageStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct UnlockConnector {
    pub status: UnlockStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFirmware {}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DataTransfer {
    pub status: DataTransferStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}
