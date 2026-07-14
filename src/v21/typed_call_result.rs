//! Typed CALLRESULT variants (built via PendingCalls / resolve — never untagged serde).
//!
//! # Scaling / unknown expected type
//!
//! Prefer [`Self::resolve_from_action_name`] when you only stored the action string (e.g. in Redis).
//! If the expected action is truly unknown, [`CallResultRaw::probe_candidates`] may list every
//! response schema that accepts the JSON — **that list can be large and ambiguous** for `{}`
//! or status-only payloads. Prefer shared correlation state over probing in production.

use super::call::Action;
use super::call_result::{CallResult, CallResultRaw};
use crate::errors::{Error, Result};
use alloc::vec::Vec;

/// Action-keyed CALLRESULT. Constructed only when the expected action is known
/// (or selected from [`CallResultRaw::probe_candidates`]).
#[derive(Debug, PartialEq, Clone)]
pub enum TypedCallResult {
    AFRRSignal(super::call_result::CallResult<crate::v21::messages::afrr_signal::AFRRSignalResponse>),
    AdjustPeriodicEventStream(super::call_result::CallResult<crate::v21::messages::adjust_periodic_event_stream::AdjustPeriodicEventStreamResponse>),
    Authorize(super::call_result::CallResult<crate::v21::messages::authorize::AuthorizeResponse>),
    BatterySwap(super::call_result::CallResult<crate::v21::messages::battery_swap::BatterySwapResponse>),
    BootNotification(super::call_result::CallResult<crate::v21::messages::boot_notification::BootNotificationResponse>),
    CancelReservation(super::call_result::CallResult<crate::v21::messages::cancel_reservation::CancelReservationResponse>),
    CertificateSigned(super::call_result::CallResult<crate::v21::messages::certificate_signed::CertificateSignedResponse>),
    ChangeAvailability(super::call_result::CallResult<crate::v21::messages::change_availability::ChangeAvailabilityResponse>),
    ChangeTransactionTariff(super::call_result::CallResult<crate::v21::messages::change_transaction_tariff::ChangeTransactionTariffResponse>),
    ClearCache(super::call_result::CallResult<crate::v21::messages::clear_cache::ClearCacheResponse>),
    ClearChargingProfile(super::call_result::CallResult<crate::v21::messages::clear_charging_profile::ClearChargingProfileResponse>),
    ClearDERControl(super::call_result::CallResult<crate::v21::messages::clear_der_control::ClearDERControlResponse>),
    ClearDisplayMessage(super::call_result::CallResult<crate::v21::messages::clear_display_message::ClearDisplayMessageResponse>),
    ClearTariffs(super::call_result::CallResult<crate::v21::messages::clear_tariffs::ClearTariffsResponse>),
    ClearVariableMonitoring(super::call_result::CallResult<crate::v21::messages::clear_variable_monitoring::ClearVariableMonitoringResponse>),
    ClearedChargingLimit(super::call_result::CallResult<crate::v21::messages::cleared_charging_limit::ClearedChargingLimitResponse>),
    ClosePeriodicEventStream(super::call_result::CallResult<crate::v21::messages::close_periodic_event_stream::ClosePeriodicEventStreamResponse>),
    CostUpdated(super::call_result::CallResult<crate::v21::messages::cost_updated::CostUpdatedResponse>),
    CustomerInformation(super::call_result::CallResult<crate::v21::messages::customer_information::CustomerInformationResponse>),
    DataTransfer(super::call_result::CallResult<crate::v21::messages::data_transfer::DataTransferResponse>),
    DeleteCertificate(super::call_result::CallResult<crate::v21::messages::delete_certificate::DeleteCertificateResponse>),
    FirmwareStatusNotification(super::call_result::CallResult<crate::v21::messages::firmware_status_notification::FirmwareStatusNotificationResponse>),
    Get15118EVCertificate(super::call_result::CallResult<crate::v21::messages::get15118_ev_certificate::Get15118EVCertificateResponse>),
    GetBaseReport(super::call_result::CallResult<crate::v21::messages::get_base_report::GetBaseReportResponse>),
    GetCertificateChainStatus(super::call_result::CallResult<crate::v21::messages::get_certificate_chain_status::GetCertificateChainStatusResponse>),
    GetCertificateStatus(super::call_result::CallResult<crate::v21::messages::get_certificate_status::GetCertificateStatusResponse>),
    GetChargingProfiles(super::call_result::CallResult<crate::v21::messages::get_charging_profiles::GetChargingProfilesResponse>),
    GetCompositeSchedule(super::call_result::CallResult<crate::v21::messages::get_composite_schedule::GetCompositeScheduleResponse>),
    GetDERControl(super::call_result::CallResult<crate::v21::messages::get_der_control::GetDERControlResponse>),
    GetDisplayMessages(super::call_result::CallResult<crate::v21::messages::get_display_messages::GetDisplayMessagesResponse>),
    GetInstalledCertificateIds(super::call_result::CallResult<crate::v21::messages::get_installed_certificate_ids::GetInstalledCertificateIdsResponse>),
    GetLocalListVersion(super::call_result::CallResult<crate::v21::messages::get_local_list_version::GetLocalListVersionResponse>),
    GetLog(super::call_result::CallResult<crate::v21::messages::get_log::GetLogResponse>),
    GetMonitoringReport(super::call_result::CallResult<crate::v21::messages::get_monitoring_report::GetMonitoringReportResponse>),
    GetPeriodicEventStream(super::call_result::CallResult<crate::v21::messages::get_periodic_event_stream::GetPeriodicEventStreamResponse>),
    GetReport(super::call_result::CallResult<crate::v21::messages::get_report::GetReportResponse>),
    GetTariffs(super::call_result::CallResult<crate::v21::messages::get_tariffs::GetTariffsResponse>),
    GetTransactionStatus(super::call_result::CallResult<crate::v21::messages::get_transaction_status::GetTransactionStatusResponse>),
    GetVariables(super::call_result::CallResult<crate::v21::messages::get_variables::GetVariablesResponse>),
    Heartbeat(super::call_result::CallResult<crate::v21::messages::heartbeat::HeartbeatResponse>),
    InstallCertificate(super::call_result::CallResult<crate::v21::messages::install_certificate::InstallCertificateResponse>),
    LogStatusNotification(super::call_result::CallResult<crate::v21::messages::log_status_notification::LogStatusNotificationResponse>),
    MeterValues(super::call_result::CallResult<crate::v21::messages::meter_values::MeterValuesResponse>),
    NotifyAllowedEnergyTransfer(super::call_result::CallResult<crate::v21::messages::notify_allowed_energy_transfer::NotifyAllowedEnergyTransferResponse>),
    NotifyChargingLimit(super::call_result::CallResult<crate::v21::messages::notify_charging_limit::NotifyChargingLimitResponse>),
    NotifyCustomerInformation(super::call_result::CallResult<crate::v21::messages::notify_customer_information::NotifyCustomerInformationResponse>),
    NotifyDERAlarm(super::call_result::CallResult<crate::v21::messages::notify_der_alarm::NotifyDERAlarmResponse>),
    NotifyDERStartStop(super::call_result::CallResult<crate::v21::messages::notify_der_start_stop::NotifyDERStartStopResponse>),
    NotifyDisplayMessages(super::call_result::CallResult<crate::v21::messages::notify_display_messages::NotifyDisplayMessagesResponse>),
    NotifyEVChargingNeeds(super::call_result::CallResult<crate::v21::messages::notify_ev_charging_needs::NotifyEVChargingNeedsResponse>),
    NotifyEVChargingSchedule(super::call_result::CallResult<crate::v21::messages::notify_ev_charging_schedule::NotifyEVChargingScheduleResponse>),
    NotifyEvent(super::call_result::CallResult<crate::v21::messages::notify_event::NotifyEventResponse>),
    NotifyMonitoringReport(super::call_result::CallResult<crate::v21::messages::notify_monitoring_report::NotifyMonitoringReportResponse>),
    NotifyPriorityCharging(super::call_result::CallResult<crate::v21::messages::notify_priority_charging::NotifyPriorityChargingResponse>),
    NotifyReport(super::call_result::CallResult<crate::v21::messages::notify_report::NotifyReportResponse>),
    NotifySettlement(super::call_result::CallResult<crate::v21::messages::notify_settlement::NotifySettlementResponse>),
    NotifyWebPaymentStarted(super::call_result::CallResult<crate::v21::messages::notify_web_payment_started::NotifyWebPaymentStartedResponse>),
    OpenPeriodicEventStream(super::call_result::CallResult<crate::v21::messages::open_periodic_event_stream::OpenPeriodicEventStreamResponse>),
    PublishFirmware(super::call_result::CallResult<crate::v21::messages::publish_firmware::PublishFirmwareResponse>),
    PublishFirmwareStatusNotification(super::call_result::CallResult<crate::v21::messages::publish_firmware_status_notification::PublishFirmwareStatusNotificationResponse>),
    PullDynamicScheduleUpdate(super::call_result::CallResult<crate::v21::messages::pull_dynamic_schedule_update::PullDynamicScheduleUpdateResponse>),
    ReportChargingProfiles(super::call_result::CallResult<crate::v21::messages::report_charging_profiles::ReportChargingProfilesResponse>),
    ReportDERControl(super::call_result::CallResult<crate::v21::messages::report_der_control::ReportDERControlResponse>),
    RequestBatterySwap(super::call_result::CallResult<crate::v21::messages::request_battery_swap::RequestBatterySwapResponse>),
    RequestStartTransaction(super::call_result::CallResult<crate::v21::messages::request_start_transaction::RequestStartTransactionResponse>),
    RequestStopTransaction(super::call_result::CallResult<crate::v21::messages::request_stop_transaction::RequestStopTransactionResponse>),
    ReservationStatusUpdate(super::call_result::CallResult<crate::v21::messages::reservation_status_update::ReservationStatusUpdateResponse>),
    ReserveNow(super::call_result::CallResult<crate::v21::messages::reserve_now::ReserveNowResponse>),
    Reset(super::call_result::CallResult<crate::v21::messages::reset::ResetResponse>),
    SecurityEventNotification(super::call_result::CallResult<crate::v21::messages::security_event_notification::SecurityEventNotificationResponse>),
    SendLocalList(super::call_result::CallResult<crate::v21::messages::send_local_list::SendLocalListResponse>),
    SetChargingProfile(super::call_result::CallResult<crate::v21::messages::set_charging_profile::SetChargingProfileResponse>),
    SetDERControl(super::call_result::CallResult<crate::v21::messages::set_der_control::SetDERControlResponse>),
    SetDefaultTariff(super::call_result::CallResult<crate::v21::messages::set_default_tariff::SetDefaultTariffResponse>),
    SetDisplayMessage(super::call_result::CallResult<crate::v21::messages::set_display_message::SetDisplayMessageResponse>),
    SetMonitoringBase(super::call_result::CallResult<crate::v21::messages::set_monitoring_base::SetMonitoringBaseResponse>),
    SetMonitoringLevel(super::call_result::CallResult<crate::v21::messages::set_monitoring_level::SetMonitoringLevelResponse>),
    SetNetworkProfile(super::call_result::CallResult<crate::v21::messages::set_network_profile::SetNetworkProfileResponse>),
    SetVariableMonitoring(super::call_result::CallResult<crate::v21::messages::set_variable_monitoring::SetVariableMonitoringResponse>),
    SetVariables(super::call_result::CallResult<crate::v21::messages::set_variables::SetVariablesResponse>),
    SignCertificate(super::call_result::CallResult<crate::v21::messages::sign_certificate::SignCertificateResponse>),
    StatusNotification(super::call_result::CallResult<crate::v21::messages::status_notification::StatusNotificationResponse>),
    TransactionEvent(super::call_result::CallResult<crate::v21::messages::transaction_event::TransactionEventResponse>),
    TriggerMessage(super::call_result::CallResult<crate::v21::messages::trigger_message::TriggerMessageResponse>),
    UnlockConnector(super::call_result::CallResult<crate::v21::messages::unlock_connector::UnlockConnectorResponse>),
    UnpublishFirmware(super::call_result::CallResult<crate::v21::messages::unpublish_firmware::UnpublishFirmwareResponse>),
    UpdateDynamicSchedule(super::call_result::CallResult<crate::v21::messages::update_dynamic_schedule::UpdateDynamicScheduleResponse>),
    UpdateFirmware(super::call_result::CallResult<crate::v21::messages::update_firmware::UpdateFirmwareResponse>),
    UsePriorityCharging(super::call_result::CallResult<crate::v21::messages::use_priority_charging::UsePriorityChargingResponse>),
    VatNumberValidation(super::call_result::CallResult<crate::v21::messages::vat_number_validation::VatNumberValidationResponse>),
}

impl TypedCallResult {
    #[must_use]
    pub const fn action_name(&self) -> &'static str {
        match self {
            Self::AFRRSignal(_) => "AFRRSignal",
            Self::AdjustPeriodicEventStream(_) => "AdjustPeriodicEventStream",
            Self::Authorize(_) => "Authorize",
            Self::BatterySwap(_) => "BatterySwap",
            Self::BootNotification(_) => "BootNotification",
            Self::CancelReservation(_) => "CancelReservation",
            Self::CertificateSigned(_) => "CertificateSigned",
            Self::ChangeAvailability(_) => "ChangeAvailability",
            Self::ChangeTransactionTariff(_) => "ChangeTransactionTariff",
            Self::ClearCache(_) => "ClearCache",
            Self::ClearChargingProfile(_) => "ClearChargingProfile",
            Self::ClearDERControl(_) => "ClearDERControl",
            Self::ClearDisplayMessage(_) => "ClearDisplayMessage",
            Self::ClearTariffs(_) => "ClearTariffs",
            Self::ClearVariableMonitoring(_) => "ClearVariableMonitoring",
            Self::ClearedChargingLimit(_) => "ClearedChargingLimit",
            Self::ClosePeriodicEventStream(_) => "ClosePeriodicEventStream",
            Self::CostUpdated(_) => "CostUpdated",
            Self::CustomerInformation(_) => "CustomerInformation",
            Self::DataTransfer(_) => "DataTransfer",
            Self::DeleteCertificate(_) => "DeleteCertificate",
            Self::FirmwareStatusNotification(_) => "FirmwareStatusNotification",
            Self::Get15118EVCertificate(_) => "Get15118EVCertificate",
            Self::GetBaseReport(_) => "GetBaseReport",
            Self::GetCertificateChainStatus(_) => "GetCertificateChainStatus",
            Self::GetCertificateStatus(_) => "GetCertificateStatus",
            Self::GetChargingProfiles(_) => "GetChargingProfiles",
            Self::GetCompositeSchedule(_) => "GetCompositeSchedule",
            Self::GetDERControl(_) => "GetDERControl",
            Self::GetDisplayMessages(_) => "GetDisplayMessages",
            Self::GetInstalledCertificateIds(_) => "GetInstalledCertificateIds",
            Self::GetLocalListVersion(_) => "GetLocalListVersion",
            Self::GetLog(_) => "GetLog",
            Self::GetMonitoringReport(_) => "GetMonitoringReport",
            Self::GetPeriodicEventStream(_) => "GetPeriodicEventStream",
            Self::GetReport(_) => "GetReport",
            Self::GetTariffs(_) => "GetTariffs",
            Self::GetTransactionStatus(_) => "GetTransactionStatus",
            Self::GetVariables(_) => "GetVariables",
            Self::Heartbeat(_) => "Heartbeat",
            Self::InstallCertificate(_) => "InstallCertificate",
            Self::LogStatusNotification(_) => "LogStatusNotification",
            Self::MeterValues(_) => "MeterValues",
            Self::NotifyAllowedEnergyTransfer(_) => "NotifyAllowedEnergyTransfer",
            Self::NotifyChargingLimit(_) => "NotifyChargingLimit",
            Self::NotifyCustomerInformation(_) => "NotifyCustomerInformation",
            Self::NotifyDERAlarm(_) => "NotifyDERAlarm",
            Self::NotifyDERStartStop(_) => "NotifyDERStartStop",
            Self::NotifyDisplayMessages(_) => "NotifyDisplayMessages",
            Self::NotifyEVChargingNeeds(_) => "NotifyEVChargingNeeds",
            Self::NotifyEVChargingSchedule(_) => "NotifyEVChargingSchedule",
            Self::NotifyEvent(_) => "NotifyEvent",
            Self::NotifyMonitoringReport(_) => "NotifyMonitoringReport",
            Self::NotifyPriorityCharging(_) => "NotifyPriorityCharging",
            Self::NotifyReport(_) => "NotifyReport",
            Self::NotifySettlement(_) => "NotifySettlement",
            Self::NotifyWebPaymentStarted(_) => "NotifyWebPaymentStarted",
            Self::OpenPeriodicEventStream(_) => "OpenPeriodicEventStream",
            Self::PublishFirmware(_) => "PublishFirmware",
            Self::PublishFirmwareStatusNotification(_) => "PublishFirmwareStatusNotification",
            Self::PullDynamicScheduleUpdate(_) => "PullDynamicScheduleUpdate",
            Self::ReportChargingProfiles(_) => "ReportChargingProfiles",
            Self::ReportDERControl(_) => "ReportDERControl",
            Self::RequestBatterySwap(_) => "RequestBatterySwap",
            Self::RequestStartTransaction(_) => "RequestStartTransaction",
            Self::RequestStopTransaction(_) => "RequestStopTransaction",
            Self::ReservationStatusUpdate(_) => "ReservationStatusUpdate",
            Self::ReserveNow(_) => "ReserveNow",
            Self::Reset(_) => "Reset",
            Self::SecurityEventNotification(_) => "SecurityEventNotification",
            Self::SendLocalList(_) => "SendLocalList",
            Self::SetChargingProfile(_) => "SetChargingProfile",
            Self::SetDERControl(_) => "SetDERControl",
            Self::SetDefaultTariff(_) => "SetDefaultTariff",
            Self::SetDisplayMessage(_) => "SetDisplayMessage",
            Self::SetMonitoringBase(_) => "SetMonitoringBase",
            Self::SetMonitoringLevel(_) => "SetMonitoringLevel",
            Self::SetNetworkProfile(_) => "SetNetworkProfile",
            Self::SetVariableMonitoring(_) => "SetVariableMonitoring",
            Self::SetVariables(_) => "SetVariables",
            Self::SignCertificate(_) => "SignCertificate",
            Self::StatusNotification(_) => "StatusNotification",
            Self::TransactionEvent(_) => "TransactionEvent",
            Self::TriggerMessage(_) => "TriggerMessage",
            Self::UnlockConnector(_) => "UnlockConnector",
            Self::UnpublishFirmware(_) => "UnpublishFirmware",
            Self::UpdateDynamicSchedule(_) => "UpdateDynamicSchedule",
            Self::UpdateFirmware(_) => "UpdateFirmware",
            Self::UsePriorityCharging(_) => "UsePriorityCharging",
            Self::VatNumberValidation(_) => "VatNumberValidation",
        }
    }

    #[must_use]
    pub fn unique_id(&self) -> &str {
        match self {
            Self::AFRRSignal(cr) => &cr.unique_id,
            Self::AdjustPeriodicEventStream(cr) => &cr.unique_id,
            Self::Authorize(cr) => &cr.unique_id,
            Self::BatterySwap(cr) => &cr.unique_id,
            Self::BootNotification(cr) => &cr.unique_id,
            Self::CancelReservation(cr) => &cr.unique_id,
            Self::CertificateSigned(cr) => &cr.unique_id,
            Self::ChangeAvailability(cr) => &cr.unique_id,
            Self::ChangeTransactionTariff(cr) => &cr.unique_id,
            Self::ClearCache(cr) => &cr.unique_id,
            Self::ClearChargingProfile(cr) => &cr.unique_id,
            Self::ClearDERControl(cr) => &cr.unique_id,
            Self::ClearDisplayMessage(cr) => &cr.unique_id,
            Self::ClearTariffs(cr) => &cr.unique_id,
            Self::ClearVariableMonitoring(cr) => &cr.unique_id,
            Self::ClearedChargingLimit(cr) => &cr.unique_id,
            Self::ClosePeriodicEventStream(cr) => &cr.unique_id,
            Self::CostUpdated(cr) => &cr.unique_id,
            Self::CustomerInformation(cr) => &cr.unique_id,
            Self::DataTransfer(cr) => &cr.unique_id,
            Self::DeleteCertificate(cr) => &cr.unique_id,
            Self::FirmwareStatusNotification(cr) => &cr.unique_id,
            Self::Get15118EVCertificate(cr) => &cr.unique_id,
            Self::GetBaseReport(cr) => &cr.unique_id,
            Self::GetCertificateChainStatus(cr) => &cr.unique_id,
            Self::GetCertificateStatus(cr) => &cr.unique_id,
            Self::GetChargingProfiles(cr) => &cr.unique_id,
            Self::GetCompositeSchedule(cr) => &cr.unique_id,
            Self::GetDERControl(cr) => &cr.unique_id,
            Self::GetDisplayMessages(cr) => &cr.unique_id,
            Self::GetInstalledCertificateIds(cr) => &cr.unique_id,
            Self::GetLocalListVersion(cr) => &cr.unique_id,
            Self::GetLog(cr) => &cr.unique_id,
            Self::GetMonitoringReport(cr) => &cr.unique_id,
            Self::GetPeriodicEventStream(cr) => &cr.unique_id,
            Self::GetReport(cr) => &cr.unique_id,
            Self::GetTariffs(cr) => &cr.unique_id,
            Self::GetTransactionStatus(cr) => &cr.unique_id,
            Self::GetVariables(cr) => &cr.unique_id,
            Self::Heartbeat(cr) => &cr.unique_id,
            Self::InstallCertificate(cr) => &cr.unique_id,
            Self::LogStatusNotification(cr) => &cr.unique_id,
            Self::MeterValues(cr) => &cr.unique_id,
            Self::NotifyAllowedEnergyTransfer(cr) => &cr.unique_id,
            Self::NotifyChargingLimit(cr) => &cr.unique_id,
            Self::NotifyCustomerInformation(cr) => &cr.unique_id,
            Self::NotifyDERAlarm(cr) => &cr.unique_id,
            Self::NotifyDERStartStop(cr) => &cr.unique_id,
            Self::NotifyDisplayMessages(cr) => &cr.unique_id,
            Self::NotifyEVChargingNeeds(cr) => &cr.unique_id,
            Self::NotifyEVChargingSchedule(cr) => &cr.unique_id,
            Self::NotifyEvent(cr) => &cr.unique_id,
            Self::NotifyMonitoringReport(cr) => &cr.unique_id,
            Self::NotifyPriorityCharging(cr) => &cr.unique_id,
            Self::NotifyReport(cr) => &cr.unique_id,
            Self::NotifySettlement(cr) => &cr.unique_id,
            Self::NotifyWebPaymentStarted(cr) => &cr.unique_id,
            Self::OpenPeriodicEventStream(cr) => &cr.unique_id,
            Self::PublishFirmware(cr) => &cr.unique_id,
            Self::PublishFirmwareStatusNotification(cr) => &cr.unique_id,
            Self::PullDynamicScheduleUpdate(cr) => &cr.unique_id,
            Self::ReportChargingProfiles(cr) => &cr.unique_id,
            Self::ReportDERControl(cr) => &cr.unique_id,
            Self::RequestBatterySwap(cr) => &cr.unique_id,
            Self::RequestStartTransaction(cr) => &cr.unique_id,
            Self::RequestStopTransaction(cr) => &cr.unique_id,
            Self::ReservationStatusUpdate(cr) => &cr.unique_id,
            Self::ReserveNow(cr) => &cr.unique_id,
            Self::Reset(cr) => &cr.unique_id,
            Self::SecurityEventNotification(cr) => &cr.unique_id,
            Self::SendLocalList(cr) => &cr.unique_id,
            Self::SetChargingProfile(cr) => &cr.unique_id,
            Self::SetDERControl(cr) => &cr.unique_id,
            Self::SetDefaultTariff(cr) => &cr.unique_id,
            Self::SetDisplayMessage(cr) => &cr.unique_id,
            Self::SetMonitoringBase(cr) => &cr.unique_id,
            Self::SetMonitoringLevel(cr) => &cr.unique_id,
            Self::SetNetworkProfile(cr) => &cr.unique_id,
            Self::SetVariableMonitoring(cr) => &cr.unique_id,
            Self::SetVariables(cr) => &cr.unique_id,
            Self::SignCertificate(cr) => &cr.unique_id,
            Self::StatusNotification(cr) => &cr.unique_id,
            Self::TransactionEvent(cr) => &cr.unique_id,
            Self::TriggerMessage(cr) => &cr.unique_id,
            Self::UnlockConnector(cr) => &cr.unique_id,
            Self::UnpublishFirmware(cr) => &cr.unique_id,
            Self::UpdateDynamicSchedule(cr) => &cr.unique_id,
            Self::UpdateFirmware(cr) => &cr.unique_id,
            Self::UsePriorityCharging(cr) => &cr.unique_id,
            Self::VatNumberValidation(cr) => &cr.unique_id,
        }
    }

    /// Resolve a raw CALLRESULT using the action of the matching pending CALL.
    ///
    /// # Errors
    /// Returns [`Error::SerdeJson`] if the payload does not match the expected response schema.
    pub fn resolve(raw: CallResultRaw, action: &Action) -> Result<Self> {
        Self::resolve_from_action_name(raw, action.action_name())
    }

    /// Resolve using only the OCPP action name string (Redis / DB friendly).
    ///
    /// # Errors
    /// * [`Error::UnknownActionName`] if `action_name` is not a known OCPP 2.1 CALL action
    /// * [`Error::SerdeJson`] if the payload does not match that action's response schema
    pub fn resolve_from_action_name(raw: CallResultRaw, action_name: &str) -> Result<Self> {
        match action_name {
            "AFRRSignal" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::AFRRSignal(CallResult::new(raw.unique_id, payload)))
            }
            "AdjustPeriodicEventStream" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::AdjustPeriodicEventStream(CallResult::new(raw.unique_id, payload)))
            }
            "Authorize" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::Authorize(CallResult::new(raw.unique_id, payload)))
            }
            "BatterySwap" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::BatterySwap(CallResult::new(raw.unique_id, payload)))
            }
            "BootNotification" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::BootNotification(CallResult::new(raw.unique_id, payload)))
            }
            "CancelReservation" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::CancelReservation(CallResult::new(raw.unique_id, payload)))
            }
            "CertificateSigned" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::CertificateSigned(CallResult::new(raw.unique_id, payload)))
            }
            "ChangeAvailability" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::ChangeAvailability(CallResult::new(raw.unique_id, payload)))
            }
            "ChangeTransactionTariff" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::ChangeTransactionTariff(CallResult::new(raw.unique_id, payload)))
            }
            "ClearCache" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::ClearCache(CallResult::new(raw.unique_id, payload)))
            }
            "ClearChargingProfile" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::ClearChargingProfile(CallResult::new(raw.unique_id, payload)))
            }
            "ClearDERControl" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::ClearDERControl(CallResult::new(raw.unique_id, payload)))
            }
            "ClearDisplayMessage" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::ClearDisplayMessage(CallResult::new(raw.unique_id, payload)))
            }
            "ClearTariffs" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::ClearTariffs(CallResult::new(raw.unique_id, payload)))
            }
            "ClearVariableMonitoring" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::ClearVariableMonitoring(CallResult::new(raw.unique_id, payload)))
            }
            "ClearedChargingLimit" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::ClearedChargingLimit(CallResult::new(raw.unique_id, payload)))
            }
            "ClosePeriodicEventStream" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::ClosePeriodicEventStream(CallResult::new(raw.unique_id, payload)))
            }
            "CostUpdated" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::CostUpdated(CallResult::new(raw.unique_id, payload)))
            }
            "CustomerInformation" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::CustomerInformation(CallResult::new(raw.unique_id, payload)))
            }
            "DataTransfer" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::DataTransfer(CallResult::new(raw.unique_id, payload)))
            }
            "DeleteCertificate" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::DeleteCertificate(CallResult::new(raw.unique_id, payload)))
            }
            "FirmwareStatusNotification" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::FirmwareStatusNotification(CallResult::new(raw.unique_id, payload)))
            }
            "Get15118EVCertificate" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::Get15118EVCertificate(CallResult::new(raw.unique_id, payload)))
            }
            "GetBaseReport" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::GetBaseReport(CallResult::new(raw.unique_id, payload)))
            }
            "GetCertificateChainStatus" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::GetCertificateChainStatus(CallResult::new(raw.unique_id, payload)))
            }
            "GetCertificateStatus" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::GetCertificateStatus(CallResult::new(raw.unique_id, payload)))
            }
            "GetChargingProfiles" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::GetChargingProfiles(CallResult::new(raw.unique_id, payload)))
            }
            "GetCompositeSchedule" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::GetCompositeSchedule(CallResult::new(raw.unique_id, payload)))
            }
            "GetDERControl" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::GetDERControl(CallResult::new(raw.unique_id, payload)))
            }
            "GetDisplayMessages" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::GetDisplayMessages(CallResult::new(raw.unique_id, payload)))
            }
            "GetInstalledCertificateIds" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::GetInstalledCertificateIds(CallResult::new(raw.unique_id, payload)))
            }
            "GetLocalListVersion" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::GetLocalListVersion(CallResult::new(raw.unique_id, payload)))
            }
            "GetLog" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::GetLog(CallResult::new(raw.unique_id, payload)))
            }
            "GetMonitoringReport" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::GetMonitoringReport(CallResult::new(raw.unique_id, payload)))
            }
            "GetPeriodicEventStream" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::GetPeriodicEventStream(CallResult::new(raw.unique_id, payload)))
            }
            "GetReport" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::GetReport(CallResult::new(raw.unique_id, payload)))
            }
            "GetTariffs" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::GetTariffs(CallResult::new(raw.unique_id, payload)))
            }
            "GetTransactionStatus" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::GetTransactionStatus(CallResult::new(raw.unique_id, payload)))
            }
            "GetVariables" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::GetVariables(CallResult::new(raw.unique_id, payload)))
            }
            "Heartbeat" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::Heartbeat(CallResult::new(raw.unique_id, payload)))
            }
            "InstallCertificate" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::InstallCertificate(CallResult::new(raw.unique_id, payload)))
            }
            "LogStatusNotification" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::LogStatusNotification(CallResult::new(raw.unique_id, payload)))
            }
            "MeterValues" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::MeterValues(CallResult::new(raw.unique_id, payload)))
            }
            "NotifyAllowedEnergyTransfer" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::NotifyAllowedEnergyTransfer(CallResult::new(raw.unique_id, payload)))
            }
            "NotifyChargingLimit" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::NotifyChargingLimit(CallResult::new(raw.unique_id, payload)))
            }
            "NotifyCustomerInformation" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::NotifyCustomerInformation(CallResult::new(raw.unique_id, payload)))
            }
            "NotifyDERAlarm" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::NotifyDERAlarm(CallResult::new(raw.unique_id, payload)))
            }
            "NotifyDERStartStop" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::NotifyDERStartStop(CallResult::new(raw.unique_id, payload)))
            }
            "NotifyDisplayMessages" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::NotifyDisplayMessages(CallResult::new(raw.unique_id, payload)))
            }
            "NotifyEVChargingNeeds" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::NotifyEVChargingNeeds(CallResult::new(raw.unique_id, payload)))
            }
            "NotifyEVChargingSchedule" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::NotifyEVChargingSchedule(CallResult::new(raw.unique_id, payload)))
            }
            "NotifyEvent" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::NotifyEvent(CallResult::new(raw.unique_id, payload)))
            }
            "NotifyMonitoringReport" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::NotifyMonitoringReport(CallResult::new(raw.unique_id, payload)))
            }
            "NotifyPriorityCharging" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::NotifyPriorityCharging(CallResult::new(raw.unique_id, payload)))
            }
            "NotifyReport" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::NotifyReport(CallResult::new(raw.unique_id, payload)))
            }
            "NotifySettlement" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::NotifySettlement(CallResult::new(raw.unique_id, payload)))
            }
            "NotifyWebPaymentStarted" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::NotifyWebPaymentStarted(CallResult::new(raw.unique_id, payload)))
            }
            "OpenPeriodicEventStream" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::OpenPeriodicEventStream(CallResult::new(raw.unique_id, payload)))
            }
            "PublishFirmware" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::PublishFirmware(CallResult::new(raw.unique_id, payload)))
            }
            "PublishFirmwareStatusNotification" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::PublishFirmwareStatusNotification(CallResult::new(raw.unique_id, payload)))
            }
            "PullDynamicScheduleUpdate" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::PullDynamicScheduleUpdate(CallResult::new(raw.unique_id, payload)))
            }
            "ReportChargingProfiles" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::ReportChargingProfiles(CallResult::new(raw.unique_id, payload)))
            }
            "ReportDERControl" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::ReportDERControl(CallResult::new(raw.unique_id, payload)))
            }
            "RequestBatterySwap" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::RequestBatterySwap(CallResult::new(raw.unique_id, payload)))
            }
            "RequestStartTransaction" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::RequestStartTransaction(CallResult::new(raw.unique_id, payload)))
            }
            "RequestStopTransaction" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::RequestStopTransaction(CallResult::new(raw.unique_id, payload)))
            }
            "ReservationStatusUpdate" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::ReservationStatusUpdate(CallResult::new(raw.unique_id, payload)))
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
                Ok(Self::SecurityEventNotification(CallResult::new(raw.unique_id, payload)))
            }
            "SendLocalList" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::SendLocalList(CallResult::new(raw.unique_id, payload)))
            }
            "SetChargingProfile" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::SetChargingProfile(CallResult::new(raw.unique_id, payload)))
            }
            "SetDERControl" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::SetDERControl(CallResult::new(raw.unique_id, payload)))
            }
            "SetDefaultTariff" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::SetDefaultTariff(CallResult::new(raw.unique_id, payload)))
            }
            "SetDisplayMessage" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::SetDisplayMessage(CallResult::new(raw.unique_id, payload)))
            }
            "SetMonitoringBase" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::SetMonitoringBase(CallResult::new(raw.unique_id, payload)))
            }
            "SetMonitoringLevel" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::SetMonitoringLevel(CallResult::new(raw.unique_id, payload)))
            }
            "SetNetworkProfile" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::SetNetworkProfile(CallResult::new(raw.unique_id, payload)))
            }
            "SetVariableMonitoring" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::SetVariableMonitoring(CallResult::new(raw.unique_id, payload)))
            }
            "SetVariables" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::SetVariables(CallResult::new(raw.unique_id, payload)))
            }
            "SignCertificate" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::SignCertificate(CallResult::new(raw.unique_id, payload)))
            }
            "StatusNotification" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::StatusNotification(CallResult::new(raw.unique_id, payload)))
            }
            "TransactionEvent" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::TransactionEvent(CallResult::new(raw.unique_id, payload)))
            }
            "TriggerMessage" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::TriggerMessage(CallResult::new(raw.unique_id, payload)))
            }
            "UnlockConnector" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::UnlockConnector(CallResult::new(raw.unique_id, payload)))
            }
            "UnpublishFirmware" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::UnpublishFirmware(CallResult::new(raw.unique_id, payload)))
            }
            "UpdateDynamicSchedule" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::UpdateDynamicSchedule(CallResult::new(raw.unique_id, payload)))
            }
            "UpdateFirmware" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::UpdateFirmware(CallResult::new(raw.unique_id, payload)))
            }
            "UsePriorityCharging" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::UsePriorityCharging(CallResult::new(raw.unique_id, payload)))
            }
            "VatNumberValidation" => {
                let payload = serde_json::from_value(raw.payload).map_err(Error::SerdeJson)?;
                Ok(Self::VatNumberValidation(CallResult::new(raw.unique_id, payload)))
            }
            other => Err(Error::UnknownActionName(alloc::string::ToString::to_string(other))),
        }
    }

    /// Convert to a wire-oriented raw CALLRESULT.
    ///
    /// # Errors
    /// Returns [`Error::SerdeJson`] if the payload cannot be serialized.
    pub fn into_raw(self) -> Result<CallResultRaw> {
        match self {
            Self::AFRRSignal(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::AdjustPeriodicEventStream(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::Authorize(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::BatterySwap(cr) => {
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
            Self::ChangeTransactionTariff(cr) => {
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
            Self::ClearDERControl(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::ClearDisplayMessage(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::ClearTariffs(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::ClearVariableMonitoring(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::ClearedChargingLimit(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::ClosePeriodicEventStream(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::CostUpdated(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::CustomerInformation(cr) => {
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
            Self::FirmwareStatusNotification(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::Get15118EVCertificate(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::GetBaseReport(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::GetCertificateChainStatus(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::GetCertificateStatus(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::GetChargingProfiles(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::GetCompositeSchedule(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::GetDERControl(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::GetDisplayMessages(cr) => {
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
            Self::GetMonitoringReport(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::GetPeriodicEventStream(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::GetReport(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::GetTariffs(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::GetTransactionStatus(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::GetVariables(cr) => {
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
            Self::NotifyAllowedEnergyTransfer(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::NotifyChargingLimit(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::NotifyCustomerInformation(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::NotifyDERAlarm(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::NotifyDERStartStop(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::NotifyDisplayMessages(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::NotifyEVChargingNeeds(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::NotifyEVChargingSchedule(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::NotifyEvent(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::NotifyMonitoringReport(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::NotifyPriorityCharging(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::NotifyReport(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::NotifySettlement(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::NotifyWebPaymentStarted(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::OpenPeriodicEventStream(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::PublishFirmware(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::PublishFirmwareStatusNotification(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::PullDynamicScheduleUpdate(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::ReportChargingProfiles(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::ReportDERControl(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::RequestBatterySwap(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::RequestStartTransaction(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::RequestStopTransaction(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::ReservationStatusUpdate(cr) => {
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
            Self::SetDERControl(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::SetDefaultTariff(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::SetDisplayMessage(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::SetMonitoringBase(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::SetMonitoringLevel(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::SetNetworkProfile(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::SetVariableMonitoring(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::SetVariables(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::SignCertificate(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::StatusNotification(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::TransactionEvent(cr) => {
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
            Self::UnpublishFirmware(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::UpdateDynamicSchedule(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::UpdateFirmware(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::UsePriorityCharging(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
            Self::VatNumberValidation(cr) => {
                let payload = serde_json::to_value(&cr.payload).map_err(Error::SerdeJson)?;
                Ok(CallResultRaw::new(cr.unique_id.clone(), payload))
            }
        }
    }

    /// Serialize as a CALLRESULT wire JSON value (`[3, messageId, payload]`).
    ///
    /// # Errors
    /// Returns [`Error::SerdeJson`] on serialization failure.
    pub fn to_value(&self) -> Result<serde_json::Value> {
        let raw = self.clone().into_raw()?;
        serde_json::to_value(&raw).map_err(Error::SerdeJson)
    }

    /// Try every known `*Response` schema against `raw.payload`.
    ///
    /// **Shortcoming:** empty (`{}`) and status-only objects often match many actions.
    /// Use only as a last resort when correlation state is unavailable; prefer
    /// [`Self::resolve_from_action_name`] with a shared store.
    #[must_use]
    pub fn probe_from_raw(raw: &CallResultRaw) -> Vec<Self> {
        let unique_id = raw.unique_id.clone();
        let payload = raw.payload.clone();
        let mut out = Vec::new();
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "AFRRSignal",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "AdjustPeriodicEventStream",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "Authorize",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "BatterySwap",
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
            "ChangeTransactionTariff",
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
            "ClearDERControl",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "ClearDisplayMessage",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "ClearTariffs",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "ClearVariableMonitoring",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "ClearedChargingLimit",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "ClosePeriodicEventStream",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "CostUpdated",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "CustomerInformation",
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
            "FirmwareStatusNotification",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "Get15118EVCertificate",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "GetBaseReport",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "GetCertificateChainStatus",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "GetCertificateStatus",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "GetChargingProfiles",
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
            "GetDERControl",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "GetDisplayMessages",
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
            "GetMonitoringReport",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "GetPeriodicEventStream",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "GetReport",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "GetTariffs",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "GetTransactionStatus",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "GetVariables",
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
            "NotifyAllowedEnergyTransfer",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "NotifyChargingLimit",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "NotifyCustomerInformation",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "NotifyDERAlarm",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "NotifyDERStartStop",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "NotifyDisplayMessages",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "NotifyEVChargingNeeds",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "NotifyEVChargingSchedule",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "NotifyEvent",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "NotifyMonitoringReport",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "NotifyPriorityCharging",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "NotifyReport",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "NotifySettlement",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "NotifyWebPaymentStarted",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "OpenPeriodicEventStream",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "PublishFirmware",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "PublishFirmwareStatusNotification",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "PullDynamicScheduleUpdate",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "ReportChargingProfiles",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "ReportDERControl",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "RequestBatterySwap",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "RequestStartTransaction",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "RequestStopTransaction",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "ReservationStatusUpdate",
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
            "SetDERControl",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "SetDefaultTariff",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "SetDisplayMessage",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "SetMonitoringBase",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "SetMonitoringLevel",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "SetNetworkProfile",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "SetVariableMonitoring",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "SetVariables",
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
            "StatusNotification",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "TransactionEvent",
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
            "UnpublishFirmware",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "UpdateDynamicSchedule",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "UpdateFirmware",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "UsePriorityCharging",
        ) {
            out.push(typed);
        }
        if let Ok(typed) = Self::resolve_from_action_name(
            CallResultRaw::new(unique_id.clone(), payload.clone()),
            "VatNumberValidation",
        ) {
            out.push(typed);
        }
        out
    }
}
