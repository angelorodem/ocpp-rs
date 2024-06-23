use super::data_types::MeterValue;
use super::enums::{
    AvailabilityType, CertificateUse, ChargePointErrorCode, ChargePointStatus,
    ChargingProfilePurposeType, ChargingRateUnitType, DiagnosticsStatus, FirmwareStatus, Log,
    MessageTrigger, Reason, ResetType, UpdateType, UploadLogStatus,
};
use chrono::{DateTime, Utc};
use serde::de::SeqAccess;
use serde::Deserializer;
use serde::{Deserialize, Serialize};
use serde_tuple::Serialize_tuple;
use super::utils::{iso8601_date_time_optional,iso8601_date_time};

use std::collections::HashMap;
use strum_macros::AsRefStr;
use rand::Rng;

// Call action enum
#[derive(AsRefStr, Debug, PartialEq, Serialize, Deserialize, Clone)]
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

#[derive(Debug, PartialEq, Serialize_tuple, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Call {
    pub(super) message_id: i32,
    pub unique_id: String,
    pub action: String,
    pub payload: Action,
}

impl Call {
    pub fn new(unique_id: Option<String>, action: String, payload: Action) -> Self {
        let unique_id = unique_id.unwrap_or_else(|| {
            let mut rng = rand::thread_rng();
            rng.gen::<u32>().to_string()
        });

        Self {
            message_id: 2,
            unique_id,
            action,
            payload,
        }
    }
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
    pub connector_id: u32,
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
    pub connector_id: u32,
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
    pub connector_id: u32,
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
    pub connector_id: u32,
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
    pub connector_id: Option<u32>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct UnlockConnector {
    pub connector_id: u32,
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

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MeterValues {
    pub connector_id: u32,
    pub meter_value: Vec<MeterValue>,
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
    pub connector_id: u32,
    pub id_tag: String,
    pub meter_start: u32,
    #[serde(with = "iso8601_date_time")]
    pub timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<i32>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct StopTransaction {
    pub meter_stop: u32,
    #[serde(with = "iso8601_date_time")]
    pub timestamp: DateTime<Utc>,
    pub transaction_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Reason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_data: Option<Vec<MeterValue>>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct StatusNotification {
    pub connector_id: u32,
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


impl<'de> Deserialize<'de> for Call {
    /// We need to manually implement the deserialization of the Call struct because the payload
    /// which has some variant types cannot be deserialized automatically, like Heartbeat and ClearCache which are empty structs.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MyVisitor;

        impl<'de> serde::de::Visitor<'de> for MyVisitor {
            type Value = Call;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence with at least two elements")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let _message_id: i32 = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let unique_id: String = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                let action: String = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;
                let payload: serde_json::Value = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(3, &self))?;

                match action.as_str() {
                    "Authorize" => {
                        let data: Authorize =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(Some(unique_id), action, Action::Authorize(data)))
                    }
                    "BootNotification" => {
                        let data: BootNotification =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(Some(unique_id), action, Action::BootNotification(data)))
                    }
                    "CancelReservation" => {
                        let data: CancelReservation =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(
                            Some(unique_id),
                            action,
                            Action::CancelReservation(data),
                        ))
                    }
                    "CertificateSigned" => {
                        let data: CertificateSigned =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(
                            Some(unique_id),
                            action,
                            Action::CertificateSigned(data),
                        ))
                    }
                    "ChangeAvailability" => {
                        let data: ChangeAvailability =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(
                            Some(unique_id),
                            action,
                            Action::ChangeAvailability(data),
                        ))
                    }
                    "ChangeConfiguration" => {
                        let data: ChangeConfiguration =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(
                            Some(unique_id),
                            action,
                            Action::ChangeConfiguration(data),
                        ))
                    }
                    "ClearCache" => {
                        let data: ClearCache =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(Some(unique_id), action, Action::ClearCache(data)))
                    }
                    "ClearChargingProfile" => {
                        let data: ClearChargingProfile =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(
                            Some(unique_id),
                            action,
                            Action::ClearChargingProfile(data),
                        ))
                    }
                    "DataTransfer" => {
                        let data: DataTransfer =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(Some(unique_id), action, Action::DataTransfer(data)))
                    }
                    "DeleteCertificate" => {
                        let data: DeleteCertificate =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(
                            Some(unique_id),
                            action,
                            Action::DeleteCertificate(data),
                        ))
                    }
                    "DiagnosticsStatusNotification" => {
                        let data: DiagnosticsStatusNotification =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(
                            Some(unique_id),
                            action,
                            Action::DiagnosticsStatusNotification(data),
                        ))
                    }
                    "ExtendedTriggerMessage" => {
                        let data: ExtendedTriggerMessage =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(
                            Some(unique_id),
                            action,
                            Action::ExtendedTriggerMessage(data),
                        ))
                    }
                    "FirmwareStatusNotification" => {
                        let data: FirmwareStatusNotification =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(
                            Some(unique_id),
                            action,
                            Action::FirmwareStatusNotification(data),
                        ))
                    }
                    "GetCompositeSchedule" => {
                        let data: GetCompositeSchedule =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(
                            Some(unique_id),
                            action,
                            Action::GetCompositeSchedule(data),
                        ))
                    }
                    "GetConfiguration" => {
                        let data: GetConfiguration =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(Some(unique_id), action, Action::GetConfiguration(data)))
                    }
                    "GetDiagnostics" => {
                        let data: GetDiagnostics =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(Some(unique_id), action, Action::GetDiagnostics(data)))
                    }
                    "GetInstalledCertificateIds" => {
                        let data: GetInstalledCertificateIds =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(
                            Some(unique_id),
                            action,
                            Action::GetInstalledCertificateIds(data),
                        ))
                    }
                    "GetLocalListVersion" => {
                        let data: GetLocalListVersion =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(
                            Some(unique_id),
                            action,
                            Action::GetLocalListVersion(data),
                        ))
                    }
                    "GetLog" => {
                        let data: GetLog =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(Some(unique_id), action, Action::GetLog(data)))
                    }
                    "Heartbeat" => {
                        let data: Heartbeat =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(Some(unique_id), action, Action::Heartbeat(data)))
                    }
                    "InstallCertificate" => {
                        let data: InstallCertificate =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(
                            Some(unique_id),
                            action,
                            Action::InstallCertificate(data),
                        ))
                    }
                    "LogStatusNotification" => {
                        let data: LogStatusNotification =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(
                            Some(unique_id),
                            action,
                            Action::LogStatusNotification(data),
                        ))
                    }
                    "MeterValues" => {
                        let data: MeterValues =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(Some(unique_id), action, Action::MeterValues(data)))
                    }
                    "RemoteStartTransaction" => {
                        let data: RemoteStartTransaction =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(
                            Some(unique_id),
                            action,
                            Action::RemoteStartTransaction(data),
                        ))
                    }
                    "RemoteStopTransaction" => {
                        let data: RemoteStopTransaction =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(
                            Some(unique_id),
                            action,
                            Action::RemoteStopTransaction(data),
                        ))
                    }
                    "ReserveNow" => {
                        let data: ReserveNow =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(Some(unique_id), action, Action::ReserveNow(data)))
                    }
                    "Reset" => {
                        let data: Reset =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(Some(unique_id), action, Action::Reset(data)))
                    }
                    "SecurityEventNotification" => {
                        let data: SecurityEventNotification =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(
                            Some(unique_id),
                            action,
                            Action::SecurityEventNotification(data),
                        ))
                    }
                    "SendLocalList" => {
                        let data: SendLocalList =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(Some(unique_id), action, Action::SendLocalList(data)))
                    }
                    "SetChargingProfile" => {
                        let data: SetChargingProfile =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(
                            Some(unique_id),
                            action,
                            Action::SetChargingProfile(data),
                        ))
                    }
                    "SignCertificate" => {
                        let data: SignCertificate =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(Some(unique_id), action, Action::SignCertificate(data)))
                    }
                    "SignedFirmwareStatusNotification" => {
                        let data: SignedFirmwareStatusNotification =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(
                            Some(unique_id),
                            action,
                            Action::SignedFirmwareStatusNotification(data),
                        ))
                    }
                    "SignedUpdateFirmware" => {
                        let data: SignedUpdateFirmware =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(
                            Some(unique_id),
                            action,
                            Action::SignedUpdateFirmware(data),
                        ))
                    }
                    "StartTransaction" => {
                        let data: StartTransaction =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(Some(unique_id), action, Action::StartTransaction(data)))
                    }
                    "StatusNotification" => {
                        let data: StatusNotification =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(
                            Some(unique_id),
                            action,
                            Action::StatusNotification(data),
                        ))
                    }
                    "StopTransaction" => {
                        let data: StopTransaction =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(Some(unique_id), action, Action::StopTransaction(data)))
                    }
                    "TriggerMessage" => {
                        let data: TriggerMessage =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(Some(unique_id), action, Action::TriggerMessage(data)))
                    }
                    "UnlockConnector" => {
                        let data: UnlockConnector =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(Some(unique_id), action, Action::UnlockConnector(data)))
                    }
                    "UpdateFirmware" => {
                        let data: UpdateFirmware =
                            serde_json::from_value(payload).map_err(serde::de::Error::custom)?;
                        Ok(Call::new(Some(unique_id), action, Action::UpdateFirmware(data)))
                    }

                    _ => Err(serde::de::Error::unknown_variant(
                        &action,
                        &[
                            "Authorize",
                            "BootNotification",
                            "CancelReservation",
                            "CertificateSigned",
                            "ChangeAvailability",
                            "ChangeConfiguration",
                            "ClearCache",
                            "ClearChargingProfile",
                            "DataTransfer",
                            "DeleteCertificate",
                            "DiagnosticsStatusNotification",
                            "ExtendedTriggerMessage",
                            "FirmwareStatusNotification",
                            "GetCompositeSchedule",
                            "GetConfiguration",
                            "GetDiagnostics",
                            "GetInstalledCertificateIds",
                            "GetLocalListVersion",
                            "GetLog",
                            "Heartbeat",
                            "InstallCertificate",
                            "LogStatusNotification",
                            "MeterValues",
                            "RemoteStartTransaction",
                            "RemoteStopTransaction",
                            "ReserveNow",
                            "Reset",
                            "SecurityEventNotification",
                            "SendLocalList",
                            "SetChargingProfile",
                            "SignCertificate",
                            "SignedFirmwareStatusNotification",
                            "SignedUpdateFirmware",
                            "StartTransaction",
                            "StatusNotification",
                            "StopTransaction",
                            "TriggerMessage",
                            "UnlockConnector",
                            "UpdateFirmware",
                        ],
                    )),
                }
            }
        }

        deserializer.deserialize_seq(MyVisitor)
    }
}
