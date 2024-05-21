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

#[derive(Debug, Serialize, Deserialize)]
pub struct Authorize {
    id_tag_info: IdTagInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BootNotification {
    current_time: String,
    interval: i32,
    status: RegistrationStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosticsStatusNotification {}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirmwareStatusNotification {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Heartbeat {
    current_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogStatusNotification {}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityEventNotification {}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignCertificate {
    status: GenericStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeterValues {}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartTransaction {
    transaction_id: i32,
    id_tag_info: IdTagInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusNotification {}

#[derive(Debug, Serialize, Deserialize)]
pub struct StopTransaction {
    id_tag_info: Option<IdTagInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelReservation {
    status: CancelReservationStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificateSigned {
    status: CertificateSignedStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeAvailability {
    status: AvailabilityStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeConfiguration {
    status: ConfigurationStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClearCache {
    status: ClearCacheStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClearChargingProfile {
    status: ClearChargingProfileStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteCertificate {
    status: DeleteCertificateStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendedTriggerMessage {
    status: TriggerMessageStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetInstalledCertificateIds {
    status: GetInstalledCertificateStatus,
    certificate_hash_data: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCompositeSchedule {
    status: GetCompositeScheduleStatus,
    connector_id: Option<i32>,
    schedule_start: Option<String>,
    charging_schedule: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetConfiguration {
    configuration_key: Option<Vec<String>>,
    unknown_key: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDiagnostics {
    file_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetLocalListVersion {
    list_version: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetLog {
    status: LogStatus,
    filename: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstallCertificate {
    status: CertificateStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteStartTransaction {
    status: RemoteStartStopStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteStopTransaction {
    status: RemoteStartStopStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReserveNow {
    status: ReservationStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reset {
    status: ResetStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendLocalList {
    status: UpdateStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetChargingProfile {
    status: ChargingProfileStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignedFirmwareStatusNotification {}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignedUpdateFirmware {
    status: UpdateFirmwareStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TriggerMessage {
    status: TriggerMessageStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnlockConnector {
    status: UnlockStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateFirmware {}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataTransfer {
    status: DataTransferStatus,
    data: Option<String>,
}
