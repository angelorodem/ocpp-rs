//! Maps each Call request type to its response payload and builds a CALLRESULT [`Message`](parse::Message).

use alloc::string::String;
use serde::Serialize;

use super::call;
use super::call_result;
use super::parse;
use crate::errors::{Error, Result};

/// Trait pairing a request payload with its response type.
pub trait Response {
    type ResponseType: Serialize;

    /// Build a [`parse::Message::CallResult`] for this request's response type.
    ///
    /// # Errors
    /// Returns [`Error::SerdeJson`] if the payload cannot be serialized.
    fn get_response(
        &self,
        unique_id: String,
        payload: Self::ResponseType,
    ) -> Result<parse::Message> {
        let value = serde_json::to_value(&payload).map_err(Error::SerdeJson)?;
        Ok(parse::Message::CallResult(call_result::CallResultRaw::new(
            unique_id, value,
        )))
    }
}

impl Response for call::Authorize {
    type ResponseType = call_result::Authorize;
}
impl Response for call::BootNotification {
    type ResponseType = call_result::BootNotification;
}
impl Response for call::CancelReservation {
    type ResponseType = call_result::CancelReservation;
}
impl Response for call::CertificateSigned {
    type ResponseType = call_result::CertificateSigned;
}
impl Response for call::ChangeAvailability {
    type ResponseType = call_result::ChangeAvailability;
}
impl Response for call::ChangeConfiguration {
    type ResponseType = call_result::ChangeConfiguration;
}
impl Response for call::ClearCache {
    type ResponseType = call_result::ClearCache;
}
impl Response for call::ClearChargingProfile {
    type ResponseType = call_result::ClearChargingProfile;
}
impl Response for call::DataTransfer {
    type ResponseType = call_result::DataTransfer;
}
impl Response for call::DeleteCertificate {
    type ResponseType = call_result::DeleteCertificate;
}
impl Response for call::DiagnosticsStatusNotification {
    type ResponseType = call_result::EmptyResponse;
}
impl Response for call::ExtendedTriggerMessage {
    type ResponseType = call_result::ExtendedTriggerMessage;
}
impl Response for call::FirmwareStatusNotification {
    type ResponseType = call_result::EmptyResponse;
}
impl Response for call::GetCompositeSchedule {
    type ResponseType = call_result::GetCompositeSchedule;
}
impl Response for call::GetConfiguration {
    type ResponseType = call_result::GetConfiguration;
}
impl Response for call::GetDiagnostics {
    type ResponseType = call_result::GetDiagnostics;
}
impl Response for call::GetInstalledCertificateIds {
    type ResponseType = call_result::GetInstalledCertificateIds;
}
impl Response for call::GetLocalListVersion {
    type ResponseType = call_result::GetLocalListVersion;
}
impl Response for call::GetLog {
    type ResponseType = call_result::GetLog;
}
impl Response for call::Heartbeat {
    type ResponseType = call_result::Heartbeat;
}
impl Response for call::InstallCertificate {
    type ResponseType = call_result::InstallCertificate;
}
impl Response for call::LogStatusNotification {
    type ResponseType = call_result::EmptyResponse;
}
impl Response for call::MeterValues {
    type ResponseType = call_result::EmptyResponse;
}
impl Response for call::RemoteStartTransaction {
    type ResponseType = call_result::RemoteStartTransaction;
}
impl Response for call::RemoteStopTransaction {
    type ResponseType = call_result::RemoteStopTransaction;
}
impl Response for call::ReserveNow {
    type ResponseType = call_result::ReserveNow;
}
impl Response for call::Reset {
    type ResponseType = call_result::Reset;
}
impl Response for call::SecurityEventNotification {
    type ResponseType = call_result::EmptyResponse;
}
impl Response for call::SendLocalList {
    type ResponseType = call_result::SendLocalList;
}
impl Response for call::SetChargingProfile {
    type ResponseType = call_result::SetChargingProfile;
}
impl Response for call::SignCertificate {
    type ResponseType = call_result::SignCertificate;
}
impl Response for call::SignedFirmwareStatusNotification {
    type ResponseType = call_result::EmptyResponse;
}
impl Response for call::SignedUpdateFirmware {
    type ResponseType = call_result::SignedUpdateFirmware;
}
impl Response for call::StartTransaction {
    type ResponseType = call_result::StartTransaction;
}
impl Response for call::StatusNotification {
    type ResponseType = call_result::EmptyResponse;
}
impl Response for call::StopTransaction {
    type ResponseType = call_result::StopTransaction;
}
impl Response for call::TriggerMessage {
    type ResponseType = call_result::TriggerMessage;
}
impl Response for call::UnlockConnector {
    type ResponseType = call_result::UnlockConnector;
}
impl Response for call::UpdateFirmware {
    type ResponseType = call_result::EmptyResponse;
}
