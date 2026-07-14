//! OCPP 1.6 CALLRESULT payloads and untyped wire envelope.
//!
//! CALLRESULT frames are `[3, messageId, payload]` with **no action name**. Do **not** try to
//! guess the response type from JSON shape alone. Use:
//!
//! - [`crate::v16::pending::PendingCalls`] / [`crate::v16::pending::PendingActionNames`]
//! - [`CallResultRaw::into_typed`] / [`deserialize_call_result`]
//! - [`CallResultRaw::probe_candidates`] only as a last resort
//!
//! Datetime fields use the shared helpers behind [`crate::datetime`] (via [`crate::v16::utils`]).

use super::data_types::{
    CertificateHashData, ChargingSchedule, DateTimeWrapper, IdTagInfo, KeyValue,
};
use super::enums::{
    AvailabilityStatus, CancelReservationStatus, CertificateSignedStatus, ChargingProfileStatus,
    ClearCacheStatus, ClearChargingProfileStatus, ConfigurationStatus, DataTransferStatus,
    DeleteCertificateStatus, GenericCertificateStatus, GetCompositeScheduleStatus,
    GetInstalledCertificateStatus, LogStatus, RegistrationStatus, RemoteStartStopStatus,
    ReservationStatus, ResetStatus, TriggerMessageStatus, UnlockStatus, UpdateFirmwareStatus,
    UpdateStatus,
};
use super::utils::{iso8601_date_time, iso8601_date_time_optional};
use alloc::string::String;
use alloc::vec::Vec;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

use crate::errors::{Error, Result};

/// Untyped CALLRESULT as received from the wire.
#[derive(Debug, PartialEq, Serialize_tuple, Deserialize_tuple, Clone)]
pub struct CallResultRaw {
    pub(super) message_id: i32,
    pub unique_id: String,
    pub payload: Value,
}

impl CallResultRaw {
    #[must_use]
    pub const fn new(unique_id: String, payload: Value) -> Self {
        Self {
            message_id: 3,
            unique_id,
            payload,
        }
    }

    /// # Errors
    /// [`Error::SerdeJson`] if the payload does not match `T`.
    pub fn into_typed<T: DeserializeOwned>(self) -> Result<CallResult<T>> {
        let payload = serde_json::from_value(self.payload).map_err(Error::SerdeJson)?;
        Ok(CallResult {
            message_id: 3,
            unique_id: self.unique_id,
            payload,
        })
    }

    /// Try every known response schema. Often ambiguous for `{}` / status-only.
    #[must_use]
    pub fn probe_candidates(&self) -> alloc::vec::Vec<crate::v16::typed_call_result::TypedCallResult> {
        crate::v16::typed_call_result::TypedCallResult::probe_from_raw(self)
    }
}

/// Typed CALLRESULT used after resolving a raw result, or when holding a known payload in memory.
#[derive(Debug, PartialEq, Clone)]
pub struct CallResult<T> {
    pub(super) message_id: i32,
    pub unique_id: String,
    pub payload: T,
}

impl<T> CallResult<T> {
    #[must_use]
    pub const fn new(unique_id: String, payload: T) -> Self {
        Self {
            message_id: 3,
            unique_id,
            payload,
        }
    }
}

/// # Errors
/// Invalid message type or payload deserialize failure.
pub fn deserialize_call_result<T: DeserializeOwned>(data: &str) -> Result<CallResult<T>> {
    let raw: CallResultRaw = serde_json::from_str(data).map_err(Error::SerdeJson)?;
    if raw.message_id != 3 {
        return Err(Error::InvalidMessageCallType);
    }
    raw.into_typed()
}

// --- Response body structs (wire shapes unchanged) ---

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BootNotification {
    #[serde(with = "iso8601_date_time")]
    pub current_time: DateTimeWrapper,
    pub interval: i32,
    pub status: RegistrationStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Heartbeat {
    #[serde(with = "iso8601_date_time")]
    pub current_time: DateTimeWrapper,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EmptyResponse {}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Authorize {
    pub id_tag_info: IdTagInfo,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StartTransaction {
    pub transaction_id: i32,
    pub id_tag_info: IdTagInfo,
}

/// StopTransaction.conf — `idTagInfo` is optional per schema.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StopTransaction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_tag_info: Option<IdTagInfo>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CancelReservation {
    pub status: CancelReservationStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CertificateSigned {
    pub status: CertificateSignedStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChangeAvailability {
    pub status: AvailabilityStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChangeConfiguration {
    pub status: ConfigurationStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ClearCache {
    pub status: ClearCacheStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ClearChargingProfile {
    pub status: ClearChargingProfileStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeleteCertificate {
    pub status: DeleteCertificateStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExtendedTriggerMessage {
    pub status: TriggerMessageStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InstallCertificate {
    pub status: GenericCertificateStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RemoteStartTransaction {
    pub status: RemoteStartStopStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RemoteStopTransaction {
    pub status: RemoteStartStopStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReserveNow {
    pub status: ReservationStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Reset {
    pub status: ResetStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SendLocalList {
    pub status: UpdateStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetChargingProfile {
    pub status: ChargingProfileStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SignCertificate {
    pub status: CertificateSignedStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SignedUpdateFirmware {
    pub status: UpdateFirmwareStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TriggerMessage {
    pub status: TriggerMessageStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UnlockConnector {
    pub status: UnlockStatus,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GetInstalledCertificateIds {
    pub status: GetInstalledCertificateStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_hash_data: Option<Vec<CertificateHashData>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GetCompositeSchedule {
    pub status: GetCompositeScheduleStatus,
    pub connector_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "iso8601_date_time_optional")]
    pub schedule_start: Option<DateTimeWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_schedule: Option<ChargingSchedule>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GetConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_key: Option<Vec<KeyValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_key: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GetDiagnostics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GetLocalListVersion {
    pub list_version: i32,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GetLog {
    pub status: LogStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DataTransfer {
    pub status: DataTransferStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}
