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
use crate::v16::utils::DateTimeWrapper;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::AsRefStr;
use serde_tuple::{Serialize_tuple, Deserialize_tuple};
use super::utils::iso8601_date_time;
use arbitrary::Arbitrary;


#[derive(Arbitrary)]
#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ResultPayload {
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

#[derive(Debug, PartialEq, Eq, Serialize_tuple, Deserialize_tuple, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CallResult	 {
    pub(super) message_id: i32,
    pub unique_id: String,
    pub payload: ResultPayload,
}

impl CallResult {
    pub fn new(unique_id: String, payload: ResultPayload) -> Self {
        Self {
            message_id: 3,
            unique_id,
            payload,
        }
    }
}

impl Arbitrary<'_> for CallResult {
    fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
        let unique_id = i32::arbitrary(u)?;
        let payload = ResultPayload::arbitrary(u)?;

        Ok(CallResult {
            message_id: 3,
            unique_id: unique_id.to_string(),
            payload,
        }) 
    }
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Authorize {
    pub id_tag_info: IdTagInfo,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BootNotification {
    /// ISO 8601 timestamp
    #[serde(with = "iso8601_date_time")]
    pub current_time: DateTimeWrapper,
    /// Interval in seconds
    pub interval: i32,
    pub status: RegistrationStatus,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticsStatusNotification {}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareStatusNotification {}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Heartbeat {
    #[serde(with = "iso8601_date_time")]
    pub current_time: DateTimeWrapper,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LogStatusNotification {}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SecurityEventNotification {}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SignCertificate {
    pub status: GenericStatus,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MeterValues {}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartTransaction {
    pub transaction_id: i32,
    pub id_tag_info: IdTagInfo,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct StatusNotification {}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct StopTransaction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_tag_info: Option<IdTagInfo>,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelReservation {
    pub status: CancelReservationStatus,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSigned {
    pub status: CertificateSignedStatus,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAvailability {
    pub status: AvailabilityStatus,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChangeConfiguration {
    pub status: ConfigurationStatus,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearCache {
    pub status: ClearCacheStatus,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfile {
    pub status: ClearChargingProfileStatus,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCertificate {
    pub status: DeleteCertificateStatus,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedTriggerMessage {
    pub status: TriggerMessageStatus,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetInstalledCertificateIds {
    pub status: GetInstalledCertificateStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_hash_data: Option<Vec<String>>,
}

#[derive(Arbitrary)]
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

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_key: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_key: Option<Vec<String>>,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDiagnostics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLocalListVersion {
    pub list_version: i32,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLog {
    pub status: LogStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct InstallCertificate {
    pub status: CertificateStatus,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStartTransaction {
    pub status: RemoteStartStopStatus,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStopTransaction {
    pub status: RemoteStartStopStatus,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReserveNow {
    pub status: ReservationStatus,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Reset {
    pub status: ResetStatus,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SendLocalList {
    pub status: UpdateStatus,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetChargingProfile {
    pub status: ChargingProfileStatus,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SignedFirmwareStatusNotification {}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SignedUpdateFirmware {
    pub status: UpdateFirmwareStatus,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TriggerMessage {
    pub status: TriggerMessageStatus,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct UnlockConnector {
    pub status: UnlockStatus,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFirmware {}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DataTransfer {
    pub status: DataTransferStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}
