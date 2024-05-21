use super::enums::{
    AvailabilityType,
    CertificateUse,
    ChargePointErrorCode,
    ChargePointStatus,
    ChargingProfilePurposeType,
    ChargingRateUnitType,
    DiagnosticsStatus,
    FirmwareStatus,
    Log,
    MessageTrigger,
    Reason,
    ResetType,
    UpdateType,
    UploadLogStatus,
};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelReservation {
    reservation_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificateSigned {
    certificate_chain: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeAvailability {
    connector_id: i32,
    #[serde(rename = "type")]
    availability_type: AvailabilityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeConfiguration {
    key: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClearCache {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClearChargingProfile {
    id: Option<i32>,
    connector_id: Option<i32>,
    charging_profile_purpose: Option<ChargingProfilePurposeType>,
    stack_level: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteCertificate {
    certificate_hash_data: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendedTriggerMessage {
    requested_message: MessageTrigger,
    connector_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCompositeSchedule {
    connector_id: i32,
    duration: i32,
    charging_rate_unit: Option<ChargingRateUnitType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetConfiguration {
    key: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDiagnostics {
    location: String,
    retries: Option<i32>,
    retry_interval: Option<i32>,
    start_time: Option<String>,
    stop_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetInstalledCertificateIds {
    certificate_type: CertificateUse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetLocalListVersion {}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetLog {
    log: HashMap<String, String>,
    log_type: Log,
    request_id: i32,
    retries: Option<i32>,
    retry_interval: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstallCertificate {
    certificate_type: CertificateUse,
    certificate: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteStartTransaction {
    id_tag: String,
    connector_id: Option<i32>,
    charging_profile: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteStopTransaction {
    transaction_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReserveNow {
    connector_id: i32,
    expiry_date: String,
    id_tag: String,
    reservation_id: i32,
    parent_id_tag: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reset {
    #[serde(rename = "type")]
    reset_type: ResetType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendLocalList {
    list_version: i32,
    update_type: UpdateType,
    local_authorization_list: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetChargingProfile {
    connector_id: i32,
    cs_charging_profiles: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignedUpdateFirmware {
    request_id: i32,
    firmware: HashMap<String, String>,
    retries: Option<i32>,
    retry_interval: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TriggerMessage {
    requested_message: MessageTrigger,
    connector_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnlockConnector {
    connector_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateFirmware {
    location: String,
    retrieve_date: String,
    retries: Option<i32>,
    retry_interval: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Authorize {
    id_tag: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BootNotification {
    charge_point_model: String,
    charge_point_vendor: String,
    charge_box_serial_number: Option<String>,
    charge_point_serial_number: Option<String>,
    firmware_version: Option<String>,
    iccid: Option<String>,
    imsi: Option<String>,
    meter_serial_number: Option<String>,
    meter_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosticsStatusNotification {
    status: DiagnosticsStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirmwareStatusNotification {
    status: FirmwareStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Heartbeat {}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogStatusNotification {
    status: UploadLogStatus,
    request_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeterValues {
    connector_id: i32,
    meter_value: Vec<String>,
    transaction_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityEventNotification {
    #[serde(rename = "type")]
    event_type: String,
    timestamp: String,
    tech_info: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignCertificate {
    csr: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignedFirmwareStatusNotification {
    status: FirmwareStatus,
    request_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartTransaction {
    connector_id: i32,
    id_tag: String,
    meter_start: i32,
    timestamp: String,
    reservation_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StopTransaction {
    meter_stop: i32,
    timestamp: String,
    transaction_id: i32,
    reason: Option<Reason>,
    id_tag: Option<String>,
    transaction_data: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusNotification {
    connector_id: i32,
    error_code: ChargePointErrorCode,
    status: ChargePointStatus,
    timestamp: Option<String>,
    info: Option<String>,
    vendor_id: Option<String>,
    vendor_error_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataTransfer {
    vendor_id: String,
    message_id: Option<String>,
    data: Option<String>,
}





