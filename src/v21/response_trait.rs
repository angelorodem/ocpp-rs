//! Maps each `*Request` to its `*Response` and builds a CALLRESULT [`Message`](parse::Message).

use alloc::string::String;
use serde::Serialize;

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
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> Result<parse::Message> {
        let value = serde_json::to_value(&payload).map_err(Error::SerdeJson)?;
        Ok(parse::Message::CallResult(call_result::CallResultRaw::new(
            unique_id, value,
        )))
    }
}

impl Response for crate::v21::messages::afrr_signal::AFRRSignalRequest {
    type ResponseType = crate::v21::messages::afrr_signal::AFRRSignalResponse;
}
impl Response for crate::v21::messages::adjust_periodic_event_stream::AdjustPeriodicEventStreamRequest {
    type ResponseType = crate::v21::messages::adjust_periodic_event_stream::AdjustPeriodicEventStreamResponse;
}
impl Response for crate::v21::messages::authorize::AuthorizeRequest {
    type ResponseType = crate::v21::messages::authorize::AuthorizeResponse;
}
impl Response for crate::v21::messages::battery_swap::BatterySwapRequest {
    type ResponseType = crate::v21::messages::battery_swap::BatterySwapResponse;
}
impl Response for crate::v21::messages::boot_notification::BootNotificationRequest {
    type ResponseType = crate::v21::messages::boot_notification::BootNotificationResponse;
}
impl Response for crate::v21::messages::cancel_reservation::CancelReservationRequest {
    type ResponseType = crate::v21::messages::cancel_reservation::CancelReservationResponse;
}
impl Response for crate::v21::messages::certificate_signed::CertificateSignedRequest {
    type ResponseType = crate::v21::messages::certificate_signed::CertificateSignedResponse;
}
impl Response for crate::v21::messages::change_availability::ChangeAvailabilityRequest {
    type ResponseType = crate::v21::messages::change_availability::ChangeAvailabilityResponse;
}
impl Response for crate::v21::messages::change_transaction_tariff::ChangeTransactionTariffRequest {
    type ResponseType = crate::v21::messages::change_transaction_tariff::ChangeTransactionTariffResponse;
}
impl Response for crate::v21::messages::clear_cache::ClearCacheRequest {
    type ResponseType = crate::v21::messages::clear_cache::ClearCacheResponse;
}
impl Response for crate::v21::messages::clear_charging_profile::ClearChargingProfileRequest {
    type ResponseType = crate::v21::messages::clear_charging_profile::ClearChargingProfileResponse;
}
impl Response for crate::v21::messages::clear_der_control::ClearDERControlRequest {
    type ResponseType = crate::v21::messages::clear_der_control::ClearDERControlResponse;
}
impl Response for crate::v21::messages::clear_display_message::ClearDisplayMessageRequest {
    type ResponseType = crate::v21::messages::clear_display_message::ClearDisplayMessageResponse;
}
impl Response for crate::v21::messages::clear_tariffs::ClearTariffsRequest {
    type ResponseType = crate::v21::messages::clear_tariffs::ClearTariffsResponse;
}
impl Response for crate::v21::messages::clear_variable_monitoring::ClearVariableMonitoringRequest {
    type ResponseType = crate::v21::messages::clear_variable_monitoring::ClearVariableMonitoringResponse;
}
impl Response for crate::v21::messages::cleared_charging_limit::ClearedChargingLimitRequest {
    type ResponseType = crate::v21::messages::cleared_charging_limit::ClearedChargingLimitResponse;
}
impl Response for crate::v21::messages::close_periodic_event_stream::ClosePeriodicEventStreamRequest {
    type ResponseType = crate::v21::messages::close_periodic_event_stream::ClosePeriodicEventStreamResponse;
}
impl Response for crate::v21::messages::cost_updated::CostUpdatedRequest {
    type ResponseType = crate::v21::messages::cost_updated::CostUpdatedResponse;
}
impl Response for crate::v21::messages::customer_information::CustomerInformationRequest {
    type ResponseType = crate::v21::messages::customer_information::CustomerInformationResponse;
}
impl Response for crate::v21::messages::data_transfer::DataTransferRequest {
    type ResponseType = crate::v21::messages::data_transfer::DataTransferResponse;
}
impl Response for crate::v21::messages::delete_certificate::DeleteCertificateRequest {
    type ResponseType = crate::v21::messages::delete_certificate::DeleteCertificateResponse;
}
impl Response for crate::v21::messages::firmware_status_notification::FirmwareStatusNotificationRequest {
    type ResponseType = crate::v21::messages::firmware_status_notification::FirmwareStatusNotificationResponse;
}
impl Response for crate::v21::messages::get15118_ev_certificate::Get15118EVCertificateRequest {
    type ResponseType = crate::v21::messages::get15118_ev_certificate::Get15118EVCertificateResponse;
}
impl Response for crate::v21::messages::get_base_report::GetBaseReportRequest {
    type ResponseType = crate::v21::messages::get_base_report::GetBaseReportResponse;
}
impl Response for crate::v21::messages::get_certificate_chain_status::GetCertificateChainStatusRequest {
    type ResponseType = crate::v21::messages::get_certificate_chain_status::GetCertificateChainStatusResponse;
}
impl Response for crate::v21::messages::get_certificate_status::GetCertificateStatusRequest {
    type ResponseType = crate::v21::messages::get_certificate_status::GetCertificateStatusResponse;
}
impl Response for crate::v21::messages::get_charging_profiles::GetChargingProfilesRequest {
    type ResponseType = crate::v21::messages::get_charging_profiles::GetChargingProfilesResponse;
}
impl Response for crate::v21::messages::get_composite_schedule::GetCompositeScheduleRequest {
    type ResponseType = crate::v21::messages::get_composite_schedule::GetCompositeScheduleResponse;
}
impl Response for crate::v21::messages::get_der_control::GetDERControlRequest {
    type ResponseType = crate::v21::messages::get_der_control::GetDERControlResponse;
}
impl Response for crate::v21::messages::get_display_messages::GetDisplayMessagesRequest {
    type ResponseType = crate::v21::messages::get_display_messages::GetDisplayMessagesResponse;
}
impl Response for crate::v21::messages::get_installed_certificate_ids::GetInstalledCertificateIdsRequest {
    type ResponseType = crate::v21::messages::get_installed_certificate_ids::GetInstalledCertificateIdsResponse;
}
impl Response for crate::v21::messages::get_local_list_version::GetLocalListVersionRequest {
    type ResponseType = crate::v21::messages::get_local_list_version::GetLocalListVersionResponse;
}
impl Response for crate::v21::messages::get_log::GetLogRequest {
    type ResponseType = crate::v21::messages::get_log::GetLogResponse;
}
impl Response for crate::v21::messages::get_monitoring_report::GetMonitoringReportRequest {
    type ResponseType = crate::v21::messages::get_monitoring_report::GetMonitoringReportResponse;
}
impl Response for crate::v21::messages::get_periodic_event_stream::GetPeriodicEventStreamRequest {
    type ResponseType = crate::v21::messages::get_periodic_event_stream::GetPeriodicEventStreamResponse;
}
impl Response for crate::v21::messages::get_report::GetReportRequest {
    type ResponseType = crate::v21::messages::get_report::GetReportResponse;
}
impl Response for crate::v21::messages::get_tariffs::GetTariffsRequest {
    type ResponseType = crate::v21::messages::get_tariffs::GetTariffsResponse;
}
impl Response for crate::v21::messages::get_transaction_status::GetTransactionStatusRequest {
    type ResponseType = crate::v21::messages::get_transaction_status::GetTransactionStatusResponse;
}
impl Response for crate::v21::messages::get_variables::GetVariablesRequest {
    type ResponseType = crate::v21::messages::get_variables::GetVariablesResponse;
}
impl Response for crate::v21::messages::heartbeat::HeartbeatRequest {
    type ResponseType = crate::v21::messages::heartbeat::HeartbeatResponse;
}
impl Response for crate::v21::messages::install_certificate::InstallCertificateRequest {
    type ResponseType = crate::v21::messages::install_certificate::InstallCertificateResponse;
}
impl Response for crate::v21::messages::log_status_notification::LogStatusNotificationRequest {
    type ResponseType = crate::v21::messages::log_status_notification::LogStatusNotificationResponse;
}
impl Response for crate::v21::messages::meter_values::MeterValuesRequest {
    type ResponseType = crate::v21::messages::meter_values::MeterValuesResponse;
}
impl Response for crate::v21::messages::notify_allowed_energy_transfer::NotifyAllowedEnergyTransferRequest {
    type ResponseType = crate::v21::messages::notify_allowed_energy_transfer::NotifyAllowedEnergyTransferResponse;
}
impl Response for crate::v21::messages::notify_charging_limit::NotifyChargingLimitRequest {
    type ResponseType = crate::v21::messages::notify_charging_limit::NotifyChargingLimitResponse;
}
impl Response for crate::v21::messages::notify_customer_information::NotifyCustomerInformationRequest {
    type ResponseType = crate::v21::messages::notify_customer_information::NotifyCustomerInformationResponse;
}
impl Response for crate::v21::messages::notify_der_alarm::NotifyDERAlarmRequest {
    type ResponseType = crate::v21::messages::notify_der_alarm::NotifyDERAlarmResponse;
}
impl Response for crate::v21::messages::notify_der_start_stop::NotifyDERStartStopRequest {
    type ResponseType = crate::v21::messages::notify_der_start_stop::NotifyDERStartStopResponse;
}
impl Response for crate::v21::messages::notify_display_messages::NotifyDisplayMessagesRequest {
    type ResponseType = crate::v21::messages::notify_display_messages::NotifyDisplayMessagesResponse;
}
impl Response for crate::v21::messages::notify_ev_charging_needs::NotifyEVChargingNeedsRequest {
    type ResponseType = crate::v21::messages::notify_ev_charging_needs::NotifyEVChargingNeedsResponse;
}
impl Response for crate::v21::messages::notify_ev_charging_schedule::NotifyEVChargingScheduleRequest {
    type ResponseType = crate::v21::messages::notify_ev_charging_schedule::NotifyEVChargingScheduleResponse;
}
impl Response for crate::v21::messages::notify_event::NotifyEventRequest {
    type ResponseType = crate::v21::messages::notify_event::NotifyEventResponse;
}
impl Response for crate::v21::messages::notify_monitoring_report::NotifyMonitoringReportRequest {
    type ResponseType = crate::v21::messages::notify_monitoring_report::NotifyMonitoringReportResponse;
}
impl Response for crate::v21::messages::notify_priority_charging::NotifyPriorityChargingRequest {
    type ResponseType = crate::v21::messages::notify_priority_charging::NotifyPriorityChargingResponse;
}
impl Response for crate::v21::messages::notify_report::NotifyReportRequest {
    type ResponseType = crate::v21::messages::notify_report::NotifyReportResponse;
}
impl Response for crate::v21::messages::notify_settlement::NotifySettlementRequest {
    type ResponseType = crate::v21::messages::notify_settlement::NotifySettlementResponse;
}
impl Response for crate::v21::messages::notify_web_payment_started::NotifyWebPaymentStartedRequest {
    type ResponseType = crate::v21::messages::notify_web_payment_started::NotifyWebPaymentStartedResponse;
}
impl Response for crate::v21::messages::open_periodic_event_stream::OpenPeriodicEventStreamRequest {
    type ResponseType = crate::v21::messages::open_periodic_event_stream::OpenPeriodicEventStreamResponse;
}
impl Response for crate::v21::messages::publish_firmware::PublishFirmwareRequest {
    type ResponseType = crate::v21::messages::publish_firmware::PublishFirmwareResponse;
}
impl Response for crate::v21::messages::publish_firmware_status_notification::PublishFirmwareStatusNotificationRequest {
    type ResponseType = crate::v21::messages::publish_firmware_status_notification::PublishFirmwareStatusNotificationResponse;
}
impl Response for crate::v21::messages::pull_dynamic_schedule_update::PullDynamicScheduleUpdateRequest {
    type ResponseType = crate::v21::messages::pull_dynamic_schedule_update::PullDynamicScheduleUpdateResponse;
}
impl Response for crate::v21::messages::report_charging_profiles::ReportChargingProfilesRequest {
    type ResponseType = crate::v21::messages::report_charging_profiles::ReportChargingProfilesResponse;
}
impl Response for crate::v21::messages::report_der_control::ReportDERControlRequest {
    type ResponseType = crate::v21::messages::report_der_control::ReportDERControlResponse;
}
impl Response for crate::v21::messages::request_battery_swap::RequestBatterySwapRequest {
    type ResponseType = crate::v21::messages::request_battery_swap::RequestBatterySwapResponse;
}
impl Response for crate::v21::messages::request_start_transaction::RequestStartTransactionRequest {
    type ResponseType = crate::v21::messages::request_start_transaction::RequestStartTransactionResponse;
}
impl Response for crate::v21::messages::request_stop_transaction::RequestStopTransactionRequest {
    type ResponseType = crate::v21::messages::request_stop_transaction::RequestStopTransactionResponse;
}
impl Response for crate::v21::messages::reservation_status_update::ReservationStatusUpdateRequest {
    type ResponseType = crate::v21::messages::reservation_status_update::ReservationStatusUpdateResponse;
}
impl Response for crate::v21::messages::reserve_now::ReserveNowRequest {
    type ResponseType = crate::v21::messages::reserve_now::ReserveNowResponse;
}
impl Response for crate::v21::messages::reset::ResetRequest {
    type ResponseType = crate::v21::messages::reset::ResetResponse;
}
impl Response for crate::v21::messages::security_event_notification::SecurityEventNotificationRequest {
    type ResponseType = crate::v21::messages::security_event_notification::SecurityEventNotificationResponse;
}
impl Response for crate::v21::messages::send_local_list::SendLocalListRequest {
    type ResponseType = crate::v21::messages::send_local_list::SendLocalListResponse;
}
impl Response for crate::v21::messages::set_charging_profile::SetChargingProfileRequest {
    type ResponseType = crate::v21::messages::set_charging_profile::SetChargingProfileResponse;
}
impl Response for crate::v21::messages::set_der_control::SetDERControlRequest {
    type ResponseType = crate::v21::messages::set_der_control::SetDERControlResponse;
}
impl Response for crate::v21::messages::set_default_tariff::SetDefaultTariffRequest {
    type ResponseType = crate::v21::messages::set_default_tariff::SetDefaultTariffResponse;
}
impl Response for crate::v21::messages::set_display_message::SetDisplayMessageRequest {
    type ResponseType = crate::v21::messages::set_display_message::SetDisplayMessageResponse;
}
impl Response for crate::v21::messages::set_monitoring_base::SetMonitoringBaseRequest {
    type ResponseType = crate::v21::messages::set_monitoring_base::SetMonitoringBaseResponse;
}
impl Response for crate::v21::messages::set_monitoring_level::SetMonitoringLevelRequest {
    type ResponseType = crate::v21::messages::set_monitoring_level::SetMonitoringLevelResponse;
}
impl Response for crate::v21::messages::set_network_profile::SetNetworkProfileRequest {
    type ResponseType = crate::v21::messages::set_network_profile::SetNetworkProfileResponse;
}
impl Response for crate::v21::messages::set_variable_monitoring::SetVariableMonitoringRequest {
    type ResponseType = crate::v21::messages::set_variable_monitoring::SetVariableMonitoringResponse;
}
impl Response for crate::v21::messages::set_variables::SetVariablesRequest {
    type ResponseType = crate::v21::messages::set_variables::SetVariablesResponse;
}
impl Response for crate::v21::messages::sign_certificate::SignCertificateRequest {
    type ResponseType = crate::v21::messages::sign_certificate::SignCertificateResponse;
}
impl Response for crate::v21::messages::status_notification::StatusNotificationRequest {
    type ResponseType = crate::v21::messages::status_notification::StatusNotificationResponse;
}
impl Response for crate::v21::messages::transaction_event::TransactionEventRequest {
    type ResponseType = crate::v21::messages::transaction_event::TransactionEventResponse;
}
impl Response for crate::v21::messages::trigger_message::TriggerMessageRequest {
    type ResponseType = crate::v21::messages::trigger_message::TriggerMessageResponse;
}
impl Response for crate::v21::messages::unlock_connector::UnlockConnectorRequest {
    type ResponseType = crate::v21::messages::unlock_connector::UnlockConnectorResponse;
}
impl Response for crate::v21::messages::unpublish_firmware::UnpublishFirmwareRequest {
    type ResponseType = crate::v21::messages::unpublish_firmware::UnpublishFirmwareResponse;
}
impl Response for crate::v21::messages::update_dynamic_schedule::UpdateDynamicScheduleRequest {
    type ResponseType = crate::v21::messages::update_dynamic_schedule::UpdateDynamicScheduleResponse;
}
impl Response for crate::v21::messages::update_firmware::UpdateFirmwareRequest {
    type ResponseType = crate::v21::messages::update_firmware::UpdateFirmwareResponse;
}
impl Response for crate::v21::messages::use_priority_charging::UsePriorityChargingRequest {
    type ResponseType = crate::v21::messages::use_priority_charging::UsePriorityChargingResponse;
}
impl Response for crate::v21::messages::vat_number_validation::VatNumberValidationRequest {
    type ResponseType = crate::v21::messages::vat_number_validation::VatNumberValidationResponse;
}
