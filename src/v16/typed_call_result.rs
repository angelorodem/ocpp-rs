//! Action-keyed CALLRESULT typing for OCPP 1.6 (never untagged serde).
//!
//! Prefer [`crate::v16::pending`] correlation. See also [`crate::v21::pending`] for the same pattern.

use super::call::Action;
use super::call_result::{CallResult, CallResultRaw};
use crate::errors::{Error, Result};
use alloc::string::ToString;
use alloc::vec::Vec;

/// Action-keyed CALLRESULT. Built only when the expected action is known.
#[derive(Debug, PartialEq, Clone)]
pub enum TypedCallResult {
    Authorize(super::call_result::CallResult<crate::v16::call_result::Authorize>),
    BootNotification(super::call_result::CallResult<crate::v16::call_result::BootNotification>),
    CancelReservation(super::call_result::CallResult<crate::v16::call_result::CancelReservation>),
    CertificateSigned(super::call_result::CallResult<crate::v16::call_result::CertificateSigned>),
    ChangeAvailability(super::call_result::CallResult<crate::v16::call_result::ChangeAvailability>),
    ChangeConfiguration(
        super::call_result::CallResult<crate::v16::call_result::ChangeConfiguration>,
    ),
    ClearCache(super::call_result::CallResult<crate::v16::call_result::ClearCache>),
    ClearChargingProfile(
        super::call_result::CallResult<crate::v16::call_result::ClearChargingProfile>,
    ),
    DataTransfer(super::call_result::CallResult<crate::v16::call_result::DataTransfer>),
    DeleteCertificate(super::call_result::CallResult<crate::v16::call_result::DeleteCertificate>),
    DiagnosticsStatusNotification(
        super::call_result::CallResult<crate::v16::call_result::EmptyResponse>,
    ),
    ExtendedTriggerMessage(
        super::call_result::CallResult<crate::v16::call_result::ExtendedTriggerMessage>,
    ),
    FirmwareStatusNotification(
        super::call_result::CallResult<crate::v16::call_result::EmptyResponse>,
    ),
    GetCompositeSchedule(
        super::call_result::CallResult<crate::v16::call_result::GetCompositeSchedule>,
    ),
    GetConfiguration(super::call_result::CallResult<crate::v16::call_result::GetConfiguration>),
    GetDiagnostics(super::call_result::CallResult<crate::v16::call_result::GetDiagnostics>),
    GetInstalledCertificateIds(
        super::call_result::CallResult<crate::v16::call_result::GetInstalledCertificateIds>,
    ),
    GetLocalListVersion(
        super::call_result::CallResult<crate::v16::call_result::GetLocalListVersion>,
    ),
    GetLog(super::call_result::CallResult<crate::v16::call_result::GetLog>),
    Heartbeat(super::call_result::CallResult<crate::v16::call_result::Heartbeat>),
    InstallCertificate(super::call_result::CallResult<crate::v16::call_result::InstallCertificate>),
    LogStatusNotification(super::call_result::CallResult<crate::v16::call_result::EmptyResponse>),
    MeterValues(super::call_result::CallResult<crate::v16::call_result::EmptyResponse>),
    RemoteStartTransaction(
        super::call_result::CallResult<crate::v16::call_result::RemoteStartTransaction>,
    ),
    RemoteStopTransaction(
        super::call_result::CallResult<crate::v16::call_result::RemoteStopTransaction>,
    ),
    ReserveNow(super::call_result::CallResult<crate::v16::call_result::ReserveNow>),
    Reset(super::call_result::CallResult<crate::v16::call_result::Reset>),
    SecurityEventNotification(
        super::call_result::CallResult<crate::v16::call_result::EmptyResponse>,
    ),
    SendLocalList(super::call_result::CallResult<crate::v16::call_result::SendLocalList>),
    SetChargingProfile(super::call_result::CallResult<crate::v16::call_result::SetChargingProfile>),
    SignCertificate(super::call_result::CallResult<crate::v16::call_result::SignCertificate>),
    SignedFirmwareStatusNotification(
        super::call_result::CallResult<crate::v16::call_result::EmptyResponse>,
    ),
    SignedUpdateFirmware(
        super::call_result::CallResult<crate::v16::call_result::SignedUpdateFirmware>,
    ),
    StartTransaction(super::call_result::CallResult<crate::v16::call_result::StartTransaction>),
    StatusNotification(super::call_result::CallResult<crate::v16::call_result::EmptyResponse>),
    StopTransaction(super::call_result::CallResult<crate::v16::call_result::StopTransaction>),
    TriggerMessage(super::call_result::CallResult<crate::v16::call_result::TriggerMessage>),
    UnlockConnector(super::call_result::CallResult<crate::v16::call_result::UnlockConnector>),
    UpdateFirmware(super::call_result::CallResult<crate::v16::call_result::EmptyResponse>),
}

impl TypedCallResult {
    #[must_use]
    pub const fn action_name(&self) -> &'static str {
        match self {
            Self::Authorize(_) => "Authorize",
            Self::BootNotification(_) => "BootNotification",
            Self::CancelReservation(_) => "CancelReservation",
            Self::CertificateSigned(_) => "CertificateSigned",
            Self::ChangeAvailability(_) => "ChangeAvailability",
            Self::ChangeConfiguration(_) => "ChangeConfiguration",
            Self::ClearCache(_) => "ClearCache",
            Self::ClearChargingProfile(_) => "ClearChargingProfile",
            Self::DataTransfer(_) => "DataTransfer",
            Self::DeleteCertificate(_) => "DeleteCertificate",
            Self::DiagnosticsStatusNotification(_) => "DiagnosticsStatusNotification",
            Self::ExtendedTriggerMessage(_) => "ExtendedTriggerMessage",
            Self::FirmwareStatusNotification(_) => "FirmwareStatusNotification",
            Self::GetCompositeSchedule(_) => "GetCompositeSchedule",
            Self::GetConfiguration(_) => "GetConfiguration",
            Self::GetDiagnostics(_) => "GetDiagnostics",
            Self::GetInstalledCertificateIds(_) => "GetInstalledCertificateIds",
            Self::GetLocalListVersion(_) => "GetLocalListVersion",
            Self::GetLog(_) => "GetLog",
            Self::Heartbeat(_) => "Heartbeat",
            Self::InstallCertificate(_) => "InstallCertificate",
            Self::LogStatusNotification(_) => "LogStatusNotification",
            Self::MeterValues(_) => "MeterValues",
            Self::RemoteStartTransaction(_) => "RemoteStartTransaction",
            Self::RemoteStopTransaction(_) => "RemoteStopTransaction",
            Self::ReserveNow(_) => "ReserveNow",
            Self::Reset(_) => "Reset",
            Self::SecurityEventNotification(_) => "SecurityEventNotification",
            Self::SendLocalList(_) => "SendLocalList",
            Self::SetChargingProfile(_) => "SetChargingProfile",
            Self::SignCertificate(_) => "SignCertificate",
            Self::SignedFirmwareStatusNotification(_) => "SignedFirmwareStatusNotification",
            Self::SignedUpdateFirmware(_) => "SignedUpdateFirmware",
            Self::StartTransaction(_) => "StartTransaction",
            Self::StatusNotification(_) => "StatusNotification",
            Self::StopTransaction(_) => "StopTransaction",
            Self::TriggerMessage(_) => "TriggerMessage",
            Self::UnlockConnector(_) => "UnlockConnector",
            Self::UpdateFirmware(_) => "UpdateFirmware",
        }
    }

    #[must_use]
    pub fn unique_id(&self) -> &str {
        match self {
            Self::Authorize(cr) => &cr.unique_id,
            Self::BootNotification(cr) => &cr.unique_id,
            Self::CancelReservation(cr) => &cr.unique_id,
            Self::CertificateSigned(cr) => &cr.unique_id,
            Self::ChangeAvailability(cr) => &cr.unique_id,
            Self::ChangeConfiguration(cr) => &cr.unique_id,
            Self::ClearCache(cr) => &cr.unique_id,
            Self::ClearChargingProfile(cr) => &cr.unique_id,
            Self::DataTransfer(cr) => &cr.unique_id,
            Self::DeleteCertificate(cr) => &cr.unique_id,
            Self::DiagnosticsStatusNotification(cr) => &cr.unique_id,
            Self::ExtendedTriggerMessage(cr) => &cr.unique_id,
            Self::FirmwareStatusNotification(cr) => &cr.unique_id,
            Self::GetCompositeSchedule(cr) => &cr.unique_id,
            Self::GetConfiguration(cr) => &cr.unique_id,
            Self::GetDiagnostics(cr) => &cr.unique_id,
            Self::GetInstalledCertificateIds(cr) => &cr.unique_id,
            Self::GetLocalListVersion(cr) => &cr.unique_id,
            Self::GetLog(cr) => &cr.unique_id,
            Self::Heartbeat(cr) => &cr.unique_id,
            Self::InstallCertificate(cr) => &cr.unique_id,
            Self::LogStatusNotification(cr) => &cr.unique_id,
            Self::MeterValues(cr) => &cr.unique_id,
            Self::RemoteStartTransaction(cr) => &cr.unique_id,
            Self::RemoteStopTransaction(cr) => &cr.unique_id,
            Self::ReserveNow(cr) => &cr.unique_id,
            Self::Reset(cr) => &cr.unique_id,
            Self::SecurityEventNotification(cr) => &cr.unique_id,
            Self::SendLocalList(cr) => &cr.unique_id,
            Self::SetChargingProfile(cr) => &cr.unique_id,
            Self::SignCertificate(cr) => &cr.unique_id,
            Self::SignedFirmwareStatusNotification(cr) => &cr.unique_id,
            Self::SignedUpdateFirmware(cr) => &cr.unique_id,
            Self::StartTransaction(cr) => &cr.unique_id,
            Self::StatusNotification(cr) => &cr.unique_id,
            Self::StopTransaction(cr) => &cr.unique_id,
            Self::TriggerMessage(cr) => &cr.unique_id,
            Self::UnlockConnector(cr) => &cr.unique_id,
            Self::UpdateFirmware(cr) => &cr.unique_id,
        }
    }

    /// # Errors
    /// Payload deserialize errors.
    pub fn resolve(raw: CallResultRaw, action: &Action) -> Result<Self> {
        Self::resolve_from_action_name(raw, action.as_ref())
    }

    /// Resolve using only the OCPP action name (Redis / DB friendly).
    ///
    /// # Errors
    /// [`Error::UnknownActionName`] or [`Error::SerdeJson`].
    pub fn resolve_from_action_name(raw: CallResultRaw, action_name: &str) -> Result<Self> {
        match action_name {
            "Authorize" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::Authorize(CallResult::new(raw.unique_id, payload)))
            }
            "BootNotification" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::BootNotification(CallResult::new(
                    raw.unique_id,
                    payload,
                )))
            }
            "CancelReservation" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::CancelReservation(CallResult::new(
                    raw.unique_id,
                    payload,
                )))
            }
            "CertificateSigned" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::CertificateSigned(CallResult::new(
                    raw.unique_id,
                    payload,
                )))
            }
            "ChangeAvailability" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::ChangeAvailability(CallResult::new(
                    raw.unique_id,
                    payload,
                )))
            }
            "ChangeConfiguration" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::ChangeConfiguration(CallResult::new(
                    raw.unique_id,
                    payload,
                )))
            }
            "ClearCache" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::ClearCache(CallResult::new(raw.unique_id, payload)))
            }
            "ClearChargingProfile" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::ClearChargingProfile(CallResult::new(
                    raw.unique_id,
                    payload,
                )))
            }
            "DataTransfer" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::DataTransfer(CallResult::new(raw.unique_id, payload)))
            }
            "DeleteCertificate" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::DeleteCertificate(CallResult::new(
                    raw.unique_id,
                    payload,
                )))
            }
            "DiagnosticsStatusNotification" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::DiagnosticsStatusNotification(CallResult::new(
                    raw.unique_id,
                    payload,
                )))
            }
            "ExtendedTriggerMessage" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::ExtendedTriggerMessage(CallResult::new(
                    raw.unique_id,
                    payload,
                )))
            }
            "FirmwareStatusNotification" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::FirmwareStatusNotification(CallResult::new(
                    raw.unique_id,
                    payload,
                )))
            }
            "GetCompositeSchedule" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::GetCompositeSchedule(CallResult::new(
                    raw.unique_id,
                    payload,
                )))
            }
            "GetConfiguration" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::GetConfiguration(CallResult::new(
                    raw.unique_id,
                    payload,
                )))
            }
            "GetDiagnostics" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::GetDiagnostics(CallResult::new(
                    raw.unique_id,
                    payload,
                )))
            }
            "GetInstalledCertificateIds" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::GetInstalledCertificateIds(CallResult::new(
                    raw.unique_id,
                    payload,
                )))
            }
            "GetLocalListVersion" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::GetLocalListVersion(CallResult::new(
                    raw.unique_id,
                    payload,
                )))
            }
            "GetLog" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::GetLog(CallResult::new(raw.unique_id, payload)))
            }
            "Heartbeat" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::Heartbeat(CallResult::new(raw.unique_id, payload)))
            }
            "InstallCertificate" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::InstallCertificate(CallResult::new(
                    raw.unique_id,
                    payload,
                )))
            }
            "LogStatusNotification" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::LogStatusNotification(CallResult::new(
                    raw.unique_id,
                    payload,
                )))
            }
            "MeterValues" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::MeterValues(CallResult::new(raw.unique_id, payload)))
            }
            "RemoteStartTransaction" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::RemoteStartTransaction(CallResult::new(
                    raw.unique_id,
                    payload,
                )))
            }
            "RemoteStopTransaction" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::RemoteStopTransaction(CallResult::new(
                    raw.unique_id,
                    payload,
                )))
            }
            "ReserveNow" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::ReserveNow(CallResult::new(raw.unique_id, payload)))
            }
            "Reset" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::Reset(CallResult::new(raw.unique_id, payload)))
            }
            "SecurityEventNotification" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::SecurityEventNotification(CallResult::new(
                    raw.unique_id,
                    payload,
                )))
            }
            "SendLocalList" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::SendLocalList(CallResult::new(raw.unique_id, payload)))
            }
            "SetChargingProfile" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::SetChargingProfile(CallResult::new(
                    raw.unique_id,
                    payload,
                )))
            }
            "SignCertificate" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::SignCertificate(CallResult::new(
                    raw.unique_id,
                    payload,
                )))
            }
            "SignedFirmwareStatusNotification" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::SignedFirmwareStatusNotification(CallResult::new(
                    raw.unique_id,
                    payload,
                )))
            }
            "SignedUpdateFirmware" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::SignedUpdateFirmware(CallResult::new(
                    raw.unique_id,
                    payload,
                )))
            }
            "StartTransaction" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::StartTransaction(CallResult::new(
                    raw.unique_id,
                    payload,
                )))
            }
            "StatusNotification" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::StatusNotification(CallResult::new(
                    raw.unique_id,
                    payload,
                )))
            }
            "StopTransaction" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::StopTransaction(CallResult::new(
                    raw.unique_id,
                    payload,
                )))
            }
            "TriggerMessage" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::TriggerMessage(CallResult::new(
                    raw.unique_id,
                    payload,
                )))
            }
            "UnlockConnector" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::UnlockConnector(CallResult::new(
                    raw.unique_id,
                    payload,
                )))
            }
            "UpdateFirmware" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::UpdateFirmware(CallResult::new(
                    raw.unique_id,
                    payload,
                )))
            }
            other => Err(Error::UnknownActionName(other.to_string())),
        }
    }

    /// # Errors
    /// Serialization errors.
    pub fn into_raw(self) -> Result<CallResultRaw> {
        match self {
            Self::Authorize(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::BootNotification(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::CancelReservation(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::CertificateSigned(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::ChangeAvailability(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::ChangeConfiguration(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::ClearCache(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::ClearChargingProfile(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::DataTransfer(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::DeleteCertificate(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::DiagnosticsStatusNotification(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::ExtendedTriggerMessage(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::FirmwareStatusNotification(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::GetCompositeSchedule(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::GetConfiguration(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::GetDiagnostics(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::GetInstalledCertificateIds(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::GetLocalListVersion(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::GetLog(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::Heartbeat(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::InstallCertificate(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::LogStatusNotification(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::MeterValues(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::RemoteStartTransaction(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::RemoteStopTransaction(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::ReserveNow(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::Reset(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::SecurityEventNotification(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::SendLocalList(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::SetChargingProfile(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::SignCertificate(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::SignedFirmwareStatusNotification(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::SignedUpdateFirmware(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::StartTransaction(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::StatusNotification(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::StopTransaction(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::TriggerMessage(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::UnlockConnector(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::UpdateFirmware(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
        }
    }

    /// # Errors
    /// Serialization errors.
    pub fn to_value(&self) -> Result<serde_json::Value> {
        let raw = self.clone().into_raw()?;
        serde_json::to_value(&raw).map_err(Error::SerdeJson)
    }

    /// Try every known response schema. **Ambiguous** for `{}` / status-only.
    #[must_use]
    pub fn probe_from_raw(raw: &CallResultRaw) -> Vec<Self> {
        let unique_id = raw.unique_id.clone();
        let payload = raw.payload.clone();
        let mut out = Vec::new();
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "Authorize",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "BootNotification",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "CancelReservation",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "CertificateSigned",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "ChangeAvailability",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "ChangeConfiguration",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "ClearCache",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "ClearChargingProfile",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "DataTransfer",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "DeleteCertificate",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "DiagnosticsStatusNotification",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "ExtendedTriggerMessage",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "FirmwareStatusNotification",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "GetCompositeSchedule",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "GetConfiguration",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "GetDiagnostics",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "GetInstalledCertificateIds",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "GetLocalListVersion",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "GetLog",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "Heartbeat",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "InstallCertificate",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "LogStatusNotification",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "MeterValues",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "RemoteStartTransaction",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "RemoteStopTransaction",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "ReserveNow",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "Reset",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "SecurityEventNotification",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "SendLocalList",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "SetChargingProfile",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "SignCertificate",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "SignedFirmwareStatusNotification",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "SignedUpdateFirmware",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "StartTransaction",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "StatusNotification",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "StopTransaction",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "TriggerMessage",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "UnlockConnector",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "UpdateFirmware",
        ) {
            out.push(typed);
        }
        out
    }
}
