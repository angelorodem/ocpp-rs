//! OCPP-J CALL (message type 2).

use alloc::string::{String, ToString};
use serde::de::{self, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_tuple::Serialize_tuple;
use strum_macros::AsRefStr;

/// All OCPP 2.1 CALL actions and their request payloads.
#[derive(AsRefStr, Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Action {
    AFRRSignal(crate::v21::messages::afrr_signal::AFRRSignalRequest),
    AdjustPeriodicEventStream(crate::v21::messages::adjust_periodic_event_stream::AdjustPeriodicEventStreamRequest),
    Authorize(crate::v21::messages::authorize::AuthorizeRequest),
    BatterySwap(crate::v21::messages::battery_swap::BatterySwapRequest),
    BootNotification(crate::v21::messages::boot_notification::BootNotificationRequest),
    CancelReservation(crate::v21::messages::cancel_reservation::CancelReservationRequest),
    CertificateSigned(crate::v21::messages::certificate_signed::CertificateSignedRequest),
    ChangeAvailability(crate::v21::messages::change_availability::ChangeAvailabilityRequest),
    ChangeTransactionTariff(crate::v21::messages::change_transaction_tariff::ChangeTransactionTariffRequest),
    ClearCache(crate::v21::messages::clear_cache::ClearCacheRequest),
    ClearChargingProfile(crate::v21::messages::clear_charging_profile::ClearChargingProfileRequest),
    ClearDERControl(crate::v21::messages::clear_der_control::ClearDERControlRequest),
    ClearDisplayMessage(crate::v21::messages::clear_display_message::ClearDisplayMessageRequest),
    ClearTariffs(crate::v21::messages::clear_tariffs::ClearTariffsRequest),
    ClearVariableMonitoring(crate::v21::messages::clear_variable_monitoring::ClearVariableMonitoringRequest),
    ClearedChargingLimit(crate::v21::messages::cleared_charging_limit::ClearedChargingLimitRequest),
    ClosePeriodicEventStream(crate::v21::messages::close_periodic_event_stream::ClosePeriodicEventStreamRequest),
    CostUpdated(crate::v21::messages::cost_updated::CostUpdatedRequest),
    CustomerInformation(crate::v21::messages::customer_information::CustomerInformationRequest),
    DataTransfer(crate::v21::messages::data_transfer::DataTransferRequest),
    DeleteCertificate(crate::v21::messages::delete_certificate::DeleteCertificateRequest),
    FirmwareStatusNotification(crate::v21::messages::firmware_status_notification::FirmwareStatusNotificationRequest),
    Get15118EVCertificate(crate::v21::messages::get15118_ev_certificate::Get15118EVCertificateRequest),
    GetBaseReport(crate::v21::messages::get_base_report::GetBaseReportRequest),
    GetCertificateChainStatus(crate::v21::messages::get_certificate_chain_status::GetCertificateChainStatusRequest),
    GetCertificateStatus(crate::v21::messages::get_certificate_status::GetCertificateStatusRequest),
    GetChargingProfiles(crate::v21::messages::get_charging_profiles::GetChargingProfilesRequest),
    GetCompositeSchedule(crate::v21::messages::get_composite_schedule::GetCompositeScheduleRequest),
    GetDERControl(crate::v21::messages::get_der_control::GetDERControlRequest),
    GetDisplayMessages(crate::v21::messages::get_display_messages::GetDisplayMessagesRequest),
    GetInstalledCertificateIds(crate::v21::messages::get_installed_certificate_ids::GetInstalledCertificateIdsRequest),
    GetLocalListVersion(crate::v21::messages::get_local_list_version::GetLocalListVersionRequest),
    GetLog(crate::v21::messages::get_log::GetLogRequest),
    GetMonitoringReport(crate::v21::messages::get_monitoring_report::GetMonitoringReportRequest),
    GetPeriodicEventStream(crate::v21::messages::get_periodic_event_stream::GetPeriodicEventStreamRequest),
    GetReport(crate::v21::messages::get_report::GetReportRequest),
    GetTariffs(crate::v21::messages::get_tariffs::GetTariffsRequest),
    GetTransactionStatus(crate::v21::messages::get_transaction_status::GetTransactionStatusRequest),
    GetVariables(crate::v21::messages::get_variables::GetVariablesRequest),
    Heartbeat(crate::v21::messages::heartbeat::HeartbeatRequest),
    InstallCertificate(crate::v21::messages::install_certificate::InstallCertificateRequest),
    LogStatusNotification(crate::v21::messages::log_status_notification::LogStatusNotificationRequest),
    MeterValues(crate::v21::messages::meter_values::MeterValuesRequest),
    NotifyAllowedEnergyTransfer(crate::v21::messages::notify_allowed_energy_transfer::NotifyAllowedEnergyTransferRequest),
    NotifyChargingLimit(crate::v21::messages::notify_charging_limit::NotifyChargingLimitRequest),
    NotifyCustomerInformation(crate::v21::messages::notify_customer_information::NotifyCustomerInformationRequest),
    NotifyDERAlarm(crate::v21::messages::notify_der_alarm::NotifyDERAlarmRequest),
    NotifyDERStartStop(crate::v21::messages::notify_der_start_stop::NotifyDERStartStopRequest),
    NotifyDisplayMessages(crate::v21::messages::notify_display_messages::NotifyDisplayMessagesRequest),
    NotifyEVChargingNeeds(crate::v21::messages::notify_ev_charging_needs::NotifyEVChargingNeedsRequest),
    NotifyEVChargingSchedule(crate::v21::messages::notify_ev_charging_schedule::NotifyEVChargingScheduleRequest),
    NotifyEvent(crate::v21::messages::notify_event::NotifyEventRequest),
    NotifyMonitoringReport(crate::v21::messages::notify_monitoring_report::NotifyMonitoringReportRequest),
    NotifyPriorityCharging(crate::v21::messages::notify_priority_charging::NotifyPriorityChargingRequest),
    NotifyReport(crate::v21::messages::notify_report::NotifyReportRequest),
    NotifySettlement(crate::v21::messages::notify_settlement::NotifySettlementRequest),
    NotifyWebPaymentStarted(crate::v21::messages::notify_web_payment_started::NotifyWebPaymentStartedRequest),
    OpenPeriodicEventStream(crate::v21::messages::open_periodic_event_stream::OpenPeriodicEventStreamRequest),
    PublishFirmware(crate::v21::messages::publish_firmware::PublishFirmwareRequest),
    PublishFirmwareStatusNotification(crate::v21::messages::publish_firmware_status_notification::PublishFirmwareStatusNotificationRequest),
    PullDynamicScheduleUpdate(crate::v21::messages::pull_dynamic_schedule_update::PullDynamicScheduleUpdateRequest),
    ReportChargingProfiles(crate::v21::messages::report_charging_profiles::ReportChargingProfilesRequest),
    ReportDERControl(crate::v21::messages::report_der_control::ReportDERControlRequest),
    RequestBatterySwap(crate::v21::messages::request_battery_swap::RequestBatterySwapRequest),
    RequestStartTransaction(crate::v21::messages::request_start_transaction::RequestStartTransactionRequest),
    RequestStopTransaction(crate::v21::messages::request_stop_transaction::RequestStopTransactionRequest),
    ReservationStatusUpdate(crate::v21::messages::reservation_status_update::ReservationStatusUpdateRequest),
    ReserveNow(crate::v21::messages::reserve_now::ReserveNowRequest),
    Reset(crate::v21::messages::reset::ResetRequest),
    SecurityEventNotification(crate::v21::messages::security_event_notification::SecurityEventNotificationRequest),
    SendLocalList(crate::v21::messages::send_local_list::SendLocalListRequest),
    SetChargingProfile(crate::v21::messages::set_charging_profile::SetChargingProfileRequest),
    SetDERControl(crate::v21::messages::set_der_control::SetDERControlRequest),
    SetDefaultTariff(crate::v21::messages::set_default_tariff::SetDefaultTariffRequest),
    SetDisplayMessage(crate::v21::messages::set_display_message::SetDisplayMessageRequest),
    SetMonitoringBase(crate::v21::messages::set_monitoring_base::SetMonitoringBaseRequest),
    SetMonitoringLevel(crate::v21::messages::set_monitoring_level::SetMonitoringLevelRequest),
    SetNetworkProfile(crate::v21::messages::set_network_profile::SetNetworkProfileRequest),
    SetVariableMonitoring(crate::v21::messages::set_variable_monitoring::SetVariableMonitoringRequest),
    SetVariables(crate::v21::messages::set_variables::SetVariablesRequest),
    SignCertificate(crate::v21::messages::sign_certificate::SignCertificateRequest),
    StatusNotification(crate::v21::messages::status_notification::StatusNotificationRequest),
    TransactionEvent(crate::v21::messages::transaction_event::TransactionEventRequest),
    TriggerMessage(crate::v21::messages::trigger_message::TriggerMessageRequest),
    UnlockConnector(crate::v21::messages::unlock_connector::UnlockConnectorRequest),
    UnpublishFirmware(crate::v21::messages::unpublish_firmware::UnpublishFirmwareRequest),
    UpdateDynamicSchedule(crate::v21::messages::update_dynamic_schedule::UpdateDynamicScheduleRequest),
    UpdateFirmware(crate::v21::messages::update_firmware::UpdateFirmwareRequest),
    UsePriorityCharging(crate::v21::messages::use_priority_charging::UsePriorityChargingRequest),
    VatNumberValidation(crate::v21::messages::vat_number_validation::VatNumberValidationRequest),
}

impl Action {
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
}

#[derive(Debug, PartialEq, Serialize_tuple, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Call {
    pub(super) message_id: i32,
    pub unique_id: String,
    pub(crate) action: String,
    pub payload: Action,
}

impl Call {
    #[must_use]
    pub fn new(unique_id: String, payload: Action) -> Self {
        Self {
            message_id: 2,
            unique_id,
            action: payload.action_name().to_string(),
            payload,
        }
    }

    #[must_use]
    pub fn action_kind(&self) -> &'static str {
        self.payload.action_name()
    }
}

impl<'de> Deserialize<'de> for Call {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CallVisitor;

        impl<'de> Visitor<'de> for CallVisitor {
            type Value = Call;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("OCPP 2.1 CALL array [2, messageId, action, payload]")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let message_id: i32 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("missing message type"))?;
                if message_id != 2 {
                    return Err(de::Error::custom("CALL message type must be 2"));
                }
                let unique_id: String = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("missing messageId"))?;
                let action_name: String = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("missing action"))?;

                match action_name.as_str() {
                    "AFRRSignal" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::AFRRSignal(payload),
                        })
                    }
                    "AdjustPeriodicEventStream" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::AdjustPeriodicEventStream(payload),
                        })
                    }
                    "Authorize" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::Authorize(payload),
                        })
                    }
                    "BatterySwap" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::BatterySwap(payload),
                        })
                    }
                    "BootNotification" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::BootNotification(payload),
                        })
                    }
                    "CancelReservation" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::CancelReservation(payload),
                        })
                    }
                    "CertificateSigned" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::CertificateSigned(payload),
                        })
                    }
                    "ChangeAvailability" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::ChangeAvailability(payload),
                        })
                    }
                    "ChangeTransactionTariff" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::ChangeTransactionTariff(payload),
                        })
                    }
                    "ClearCache" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::ClearCache(payload),
                        })
                    }
                    "ClearChargingProfile" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::ClearChargingProfile(payload),
                        })
                    }
                    "ClearDERControl" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::ClearDERControl(payload),
                        })
                    }
                    "ClearDisplayMessage" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::ClearDisplayMessage(payload),
                        })
                    }
                    "ClearTariffs" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::ClearTariffs(payload),
                        })
                    }
                    "ClearVariableMonitoring" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::ClearVariableMonitoring(payload),
                        })
                    }
                    "ClearedChargingLimit" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::ClearedChargingLimit(payload),
                        })
                    }
                    "ClosePeriodicEventStream" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::ClosePeriodicEventStream(payload),
                        })
                    }
                    "CostUpdated" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::CostUpdated(payload),
                        })
                    }
                    "CustomerInformation" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::CustomerInformation(payload),
                        })
                    }
                    "DataTransfer" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::DataTransfer(payload),
                        })
                    }
                    "DeleteCertificate" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::DeleteCertificate(payload),
                        })
                    }
                    "FirmwareStatusNotification" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::FirmwareStatusNotification(payload),
                        })
                    }
                    "Get15118EVCertificate" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::Get15118EVCertificate(payload),
                        })
                    }
                    "GetBaseReport" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::GetBaseReport(payload),
                        })
                    }
                    "GetCertificateChainStatus" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::GetCertificateChainStatus(payload),
                        })
                    }
                    "GetCertificateStatus" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::GetCertificateStatus(payload),
                        })
                    }
                    "GetChargingProfiles" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::GetChargingProfiles(payload),
                        })
                    }
                    "GetCompositeSchedule" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::GetCompositeSchedule(payload),
                        })
                    }
                    "GetDERControl" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::GetDERControl(payload),
                        })
                    }
                    "GetDisplayMessages" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::GetDisplayMessages(payload),
                        })
                    }
                    "GetInstalledCertificateIds" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::GetInstalledCertificateIds(payload),
                        })
                    }
                    "GetLocalListVersion" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::GetLocalListVersion(payload),
                        })
                    }
                    "GetLog" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::GetLog(payload),
                        })
                    }
                    "GetMonitoringReport" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::GetMonitoringReport(payload),
                        })
                    }
                    "GetPeriodicEventStream" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::GetPeriodicEventStream(payload),
                        })
                    }
                    "GetReport" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::GetReport(payload),
                        })
                    }
                    "GetTariffs" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::GetTariffs(payload),
                        })
                    }
                    "GetTransactionStatus" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::GetTransactionStatus(payload),
                        })
                    }
                    "GetVariables" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::GetVariables(payload),
                        })
                    }
                    "Heartbeat" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::Heartbeat(payload),
                        })
                    }
                    "InstallCertificate" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::InstallCertificate(payload),
                        })
                    }
                    "LogStatusNotification" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::LogStatusNotification(payload),
                        })
                    }
                    "MeterValues" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::MeterValues(payload),
                        })
                    }
                    "NotifyAllowedEnergyTransfer" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::NotifyAllowedEnergyTransfer(payload),
                        })
                    }
                    "NotifyChargingLimit" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::NotifyChargingLimit(payload),
                        })
                    }
                    "NotifyCustomerInformation" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::NotifyCustomerInformation(payload),
                        })
                    }
                    "NotifyDERAlarm" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::NotifyDERAlarm(payload),
                        })
                    }
                    "NotifyDERStartStop" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::NotifyDERStartStop(payload),
                        })
                    }
                    "NotifyDisplayMessages" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::NotifyDisplayMessages(payload),
                        })
                    }
                    "NotifyEVChargingNeeds" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::NotifyEVChargingNeeds(payload),
                        })
                    }
                    "NotifyEVChargingSchedule" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::NotifyEVChargingSchedule(payload),
                        })
                    }
                    "NotifyEvent" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::NotifyEvent(payload),
                        })
                    }
                    "NotifyMonitoringReport" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::NotifyMonitoringReport(payload),
                        })
                    }
                    "NotifyPriorityCharging" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::NotifyPriorityCharging(payload),
                        })
                    }
                    "NotifyReport" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::NotifyReport(payload),
                        })
                    }
                    "NotifySettlement" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::NotifySettlement(payload),
                        })
                    }
                    "NotifyWebPaymentStarted" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::NotifyWebPaymentStarted(payload),
                        })
                    }
                    "OpenPeriodicEventStream" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::OpenPeriodicEventStream(payload),
                        })
                    }
                    "PublishFirmware" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::PublishFirmware(payload),
                        })
                    }
                    "PublishFirmwareStatusNotification" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::PublishFirmwareStatusNotification(payload),
                        })
                    }
                    "PullDynamicScheduleUpdate" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::PullDynamicScheduleUpdate(payload),
                        })
                    }
                    "ReportChargingProfiles" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::ReportChargingProfiles(payload),
                        })
                    }
                    "ReportDERControl" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::ReportDERControl(payload),
                        })
                    }
                    "RequestBatterySwap" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::RequestBatterySwap(payload),
                        })
                    }
                    "RequestStartTransaction" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::RequestStartTransaction(payload),
                        })
                    }
                    "RequestStopTransaction" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::RequestStopTransaction(payload),
                        })
                    }
                    "ReservationStatusUpdate" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::ReservationStatusUpdate(payload),
                        })
                    }
                    "ReserveNow" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::ReserveNow(payload),
                        })
                    }
                    "Reset" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::Reset(payload),
                        })
                    }
                    "SecurityEventNotification" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::SecurityEventNotification(payload),
                        })
                    }
                    "SendLocalList" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::SendLocalList(payload),
                        })
                    }
                    "SetChargingProfile" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::SetChargingProfile(payload),
                        })
                    }
                    "SetDERControl" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::SetDERControl(payload),
                        })
                    }
                    "SetDefaultTariff" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::SetDefaultTariff(payload),
                        })
                    }
                    "SetDisplayMessage" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::SetDisplayMessage(payload),
                        })
                    }
                    "SetMonitoringBase" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::SetMonitoringBase(payload),
                        })
                    }
                    "SetMonitoringLevel" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::SetMonitoringLevel(payload),
                        })
                    }
                    "SetNetworkProfile" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::SetNetworkProfile(payload),
                        })
                    }
                    "SetVariableMonitoring" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::SetVariableMonitoring(payload),
                        })
                    }
                    "SetVariables" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::SetVariables(payload),
                        })
                    }
                    "SignCertificate" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::SignCertificate(payload),
                        })
                    }
                    "StatusNotification" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::StatusNotification(payload),
                        })
                    }
                    "TransactionEvent" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::TransactionEvent(payload),
                        })
                    }
                    "TriggerMessage" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::TriggerMessage(payload),
                        })
                    }
                    "UnlockConnector" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::UnlockConnector(payload),
                        })
                    }
                    "UnpublishFirmware" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::UnpublishFirmware(payload),
                        })
                    }
                    "UpdateDynamicSchedule" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::UpdateDynamicSchedule(payload),
                        })
                    }
                    "UpdateFirmware" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::UpdateFirmware(payload),
                        })
                    }
                    "UsePriorityCharging" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::UsePriorityCharging(payload),
                        })
                    }
                    "VatNumberValidation" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                        Ok(Call {
                            message_id: 2,
                            unique_id,
                            action: action_name,
                            payload: Action::VatNumberValidation(payload),
                        })
                    }
                    other => Err(de::Error::custom(alloc::format!(
                        "unknown OCPP 2.1 action: {other}"
                    ))),
                }
            }
        }

        deserializer.deserialize_seq(CallVisitor)
    }
}
