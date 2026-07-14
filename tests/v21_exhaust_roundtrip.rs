//! AUTO-generated exhaustive v21 CALL round-trips. Regenerate: python tools/gen_exhaust_tests.py
use ocpp_rs::v21::parse::{deserialize_to_message, serialize_message, Message};

fn assert_call_roundtrip(wire: &str) {
    let msg = deserialize_to_message(wire).unwrap_or_else(|e| panic!("parse {wire}: {e}"));
    assert!(matches!(msg, Message::Call(_) | Message::Send(_)), "expected Call/Send: {msg:?}");
    let again = serialize_message(&msg).expect("serialize");
    let back = deserialize_to_message(&again).expect("reparse");
    assert_eq!(msg, back);
}

#[test]
fn roundtrip_afrr_signal() {
    assert_call_roundtrip(r#"[2,"t","AFRRSignal",{"timestamp":"2024-01-01T00:00:00.000Z","signal":0}]"#);
}

#[test]
fn roundtrip_adjust_periodic_event_stream() {
    assert_call_roundtrip(r#"[2,"t","AdjustPeriodicEventStream",{"id":0,"params":{}}]"#);
}

#[test]
fn roundtrip_authorize() {
    assert_call_roundtrip(r#"[2,"t","Authorize",{"idToken":{"idToken":"x","type":"x"}}]"#);
}

#[test]
fn roundtrip_battery_swap() {
    assert_call_roundtrip(r#"[2,"t","BatterySwap",{"batteryData":[{"evseId":0,"serialNumber":"x","soC":0.0,"soH":0.0}],"eventType":"BatteryIn","idToken":{"idToken":"x","type":"x"},"requestId":0}]"#);
}

#[test]
fn roundtrip_boot_notification() {
    assert_call_roundtrip(r#"[2,"t","BootNotification",{"chargingStation":{"model":"x","vendorName":"x"},"reason":"ApplicationReset"}]"#);
}

#[test]
fn roundtrip_cancel_reservation() {
    assert_call_roundtrip(r#"[2,"t","CancelReservation",{"reservationId":0}]"#);
}

#[test]
fn roundtrip_certificate_signed() {
    assert_call_roundtrip(r#"[2,"t","CertificateSigned",{"certificateChain":"x"}]"#);
}

#[test]
fn roundtrip_change_availability() {
    assert_call_roundtrip(r#"[2,"t","ChangeAvailability",{"operationalStatus":"Inoperative"}]"#);
}

#[test]
fn roundtrip_change_transaction_tariff() {
    assert_call_roundtrip(r#"[2,"t","ChangeTransactionTariff",{"tariff":{"tariffId":"x","currency":"x"},"transactionId":"x"}]"#);
}

#[test]
fn roundtrip_clear_cache() {
    assert_call_roundtrip(r#"[2,"t","ClearCache",{}]"#);
}

#[test]
fn roundtrip_clear_charging_profile() {
    assert_call_roundtrip(r#"[2,"t","ClearChargingProfile",{}]"#);
}

#[test]
fn roundtrip_clear_der_control() {
    assert_call_roundtrip(r#"[2,"t","ClearDERControl",{"isDefault":false}]"#);
}

#[test]
fn roundtrip_clear_display_message() {
    assert_call_roundtrip(r#"[2,"t","ClearDisplayMessage",{"id":0}]"#);
}

#[test]
fn roundtrip_clear_tariffs() {
    assert_call_roundtrip(r#"[2,"t","ClearTariffs",{}]"#);
}

#[test]
fn roundtrip_clear_variable_monitoring() {
    assert_call_roundtrip(r#"[2,"t","ClearVariableMonitoring",{"id":[0]}]"#);
}

#[test]
fn roundtrip_cleared_charging_limit() {
    assert_call_roundtrip(r#"[2,"t","ClearedChargingLimit",{"chargingLimitSource":"x"}]"#);
}

#[test]
fn roundtrip_close_periodic_event_stream() {
    assert_call_roundtrip(r#"[2,"t","ClosePeriodicEventStream",{"id":0}]"#);
}

#[test]
fn roundtrip_cost_updated() {
    assert_call_roundtrip(r#"[2,"t","CostUpdated",{"totalCost":0.0,"transactionId":"x"}]"#);
}

#[test]
fn roundtrip_customer_information() {
    assert_call_roundtrip(r#"[2,"t","CustomerInformation",{"requestId":0,"report":false,"clear":false}]"#);
}

#[test]
fn roundtrip_data_transfer() {
    assert_call_roundtrip(r#"[2,"t","DataTransfer",{"vendorId":"x"}]"#);
}

#[test]
fn roundtrip_delete_certificate() {
    assert_call_roundtrip(r#"[2,"t","DeleteCertificate",{"certificateHashData":{"hashAlgorithm":"SHA256","issuerNameHash":"x","issuerKeyHash":"x","serialNumber":"x"}}]"#);
}

#[test]
fn roundtrip_firmware_status_notification() {
    assert_call_roundtrip(r#"[2,"t","FirmwareStatusNotification",{"status":"Downloaded"}]"#);
}

#[test]
fn roundtrip_get15118_ev_certificate() {
    assert_call_roundtrip(r#"[2,"t","Get15118EVCertificate",{"iso15118SchemaVersion":"x","action":"Install","exiRequest":"x"}]"#);
}

#[test]
fn roundtrip_get_base_report() {
    assert_call_roundtrip(r#"[2,"t","GetBaseReport",{"requestId":0,"reportBase":"ConfigurationInventory"}]"#);
}

#[test]
fn roundtrip_get_certificate_chain_status() {
    assert_call_roundtrip(r#"[2,"t","GetCertificateChainStatus",{"certificateStatusRequests":[{"certificateHashData":{"hashAlgorithm":"SHA256","issuerNameHash":"x","issuerKeyHash":"x","serialNumber":"x"},"source":"CRL","urls":["x"]}]}]"#);
}

#[test]
fn roundtrip_get_certificate_status() {
    assert_call_roundtrip(r#"[2,"t","GetCertificateStatus",{"ocspRequestData":{"hashAlgorithm":"SHA256","issuerNameHash":"x","issuerKeyHash":"x","serialNumber":"x","responderURL":"x"}}]"#);
}

#[test]
fn roundtrip_get_charging_profiles() {
    assert_call_roundtrip(r#"[2,"t","GetChargingProfiles",{"requestId":0,"chargingProfile":{}}]"#);
}

#[test]
fn roundtrip_get_composite_schedule() {
    assert_call_roundtrip(r#"[2,"t","GetCompositeSchedule",{"duration":0,"evseId":0}]"#);
}

#[test]
fn roundtrip_get_der_control() {
    assert_call_roundtrip(r#"[2,"t","GetDERControl",{"requestId":0}]"#);
}

#[test]
fn roundtrip_get_display_messages() {
    assert_call_roundtrip(r#"[2,"t","GetDisplayMessages",{"requestId":0}]"#);
}

#[test]
fn roundtrip_get_installed_certificate_ids() {
    assert_call_roundtrip(r#"[2,"t","GetInstalledCertificateIds",{}]"#);
}

#[test]
fn roundtrip_get_local_list_version() {
    assert_call_roundtrip(r#"[2,"t","GetLocalListVersion",{}]"#);
}

#[test]
fn roundtrip_get_log() {
    assert_call_roundtrip(r#"[2,"t","GetLog",{"log":{"remoteLocation":"x"},"logType":"DiagnosticsLog","requestId":0}]"#);
}

#[test]
fn roundtrip_get_monitoring_report() {
    assert_call_roundtrip(r#"[2,"t","GetMonitoringReport",{"requestId":0}]"#);
}

#[test]
fn roundtrip_get_periodic_event_stream() {
    assert_call_roundtrip(r#"[2,"t","GetPeriodicEventStream",{}]"#);
}

#[test]
fn roundtrip_get_report() {
    assert_call_roundtrip(r#"[2,"t","GetReport",{"requestId":0}]"#);
}

#[test]
fn roundtrip_get_tariffs() {
    assert_call_roundtrip(r#"[2,"t","GetTariffs",{"evseId":0}]"#);
}

#[test]
fn roundtrip_get_transaction_status() {
    assert_call_roundtrip(r#"[2,"t","GetTransactionStatus",{}]"#);
}

#[test]
fn roundtrip_get_variables() {
    assert_call_roundtrip(r#"[2,"t","GetVariables",{"getVariableData":[{"component":{"name":"x"},"variable":{"name":"x"}}]}]"#);
}

#[test]
fn roundtrip_heartbeat() {
    assert_call_roundtrip(r#"[2,"t","Heartbeat",{}]"#);
}

#[test]
fn roundtrip_install_certificate() {
    assert_call_roundtrip(r#"[2,"t","InstallCertificate",{"certificateType":"V2GRootCertificate","certificate":"x"}]"#);
}

#[test]
fn roundtrip_log_status_notification() {
    assert_call_roundtrip(r#"[2,"t","LogStatusNotification",{"status":"BadMessage"}]"#);
}

#[test]
fn roundtrip_meter_values() {
    assert_call_roundtrip(r#"[2,"t","MeterValues",{"evseId":0,"meterValue":[{"sampledValue":[{"value":0.0}],"timestamp":"2024-01-01T00:00:00.000Z"}]}]"#);
}

#[test]
fn roundtrip_notify_allowed_energy_transfer() {
    assert_call_roundtrip(r#"[2,"t","NotifyAllowedEnergyTransfer",{"transactionId":"x","allowedEnergyTransfer":["AC_single_phase"]}]"#);
}

#[test]
fn roundtrip_notify_charging_limit() {
    assert_call_roundtrip(r#"[2,"t","NotifyChargingLimit",{"chargingLimit":{"chargingLimitSource":"x"}}]"#);
}

#[test]
fn roundtrip_notify_customer_information() {
    assert_call_roundtrip(r#"[2,"t","NotifyCustomerInformation",{"data":"x","seqNo":0,"generatedAt":"2024-01-01T00:00:00.000Z","requestId":0}]"#);
}

#[test]
fn roundtrip_notify_der_alarm() {
    assert_call_roundtrip(r#"[2,"t","NotifyDERAlarm",{"controlType":"EnterService","timestamp":"2024-01-01T00:00:00.000Z"}]"#);
}

#[test]
fn roundtrip_notify_der_start_stop() {
    assert_call_roundtrip(r#"[2,"t","NotifyDERStartStop",{"controlId":"x","started":false,"timestamp":"2024-01-01T00:00:00.000Z"}]"#);
}

#[test]
fn roundtrip_notify_display_messages() {
    assert_call_roundtrip(r#"[2,"t","NotifyDisplayMessages",{"requestId":0}]"#);
}

#[test]
fn roundtrip_notify_ev_charging_needs() {
    assert_call_roundtrip(r#"[2,"t","NotifyEVChargingNeeds",{"evseId":1,"chargingNeeds":{"requestedEnergyTransfer":"AC_single_phase"}}]"#);
}

#[test]
fn roundtrip_notify_ev_charging_schedule() {
    assert_call_roundtrip(r#"[2,"t","NotifyEVChargingSchedule",{"timeBase":"2024-01-01T00:00:00.000Z","chargingSchedule":{"id":0,"chargingRateUnit":"W","chargingSchedulePeriod":[{"startPeriod":0}]},"evseId":1}]"#);
}

#[test]
fn roundtrip_notify_event() {
    assert_call_roundtrip(r#"[2,"t","NotifyEvent",{"generatedAt":"2024-01-01T00:00:00.000Z","seqNo":0,"eventData":[{"eventId":0,"timestamp":"2024-01-01T00:00:00.000Z","trigger":"Alerting","actualValue":"x","component":{"name":"x"},"eventNotificationType":"HardWiredNotification","variable":{"name":"x"}}]}]"#);
}

#[test]
fn roundtrip_notify_monitoring_report() {
    assert_call_roundtrip(r#"[2,"t","NotifyMonitoringReport",{"requestId":0,"seqNo":0,"generatedAt":"2024-01-01T00:00:00.000Z"}]"#);
}

#[test]
fn roundtrip_notify_priority_charging() {
    assert_call_roundtrip(r#"[2,"t","NotifyPriorityCharging",{"transactionId":"x","activated":false}]"#);
}

#[test]
fn roundtrip_notify_report() {
    assert_call_roundtrip(r#"[2,"t","NotifyReport",{"requestId":0,"generatedAt":"2024-01-01T00:00:00.000Z","seqNo":0}]"#);
}

#[test]
fn roundtrip_notify_settlement() {
    assert_call_roundtrip(r#"[2,"t","NotifySettlement",{"pspRef":"x","status":"Settled","settlementAmount":0.0,"settlementTime":"2024-01-01T00:00:00.000Z"}]"#);
}

#[test]
fn roundtrip_notify_web_payment_started() {
    assert_call_roundtrip(r#"[2,"t","NotifyWebPaymentStarted",{"evseId":0,"timeout":0}]"#);
}

#[test]
fn roundtrip_open_periodic_event_stream() {
    assert_call_roundtrip(r#"[2,"t","OpenPeriodicEventStream",{"constantStreamData":{"id":0,"params":{},"variableMonitoringId":0}}]"#);
}

#[test]
fn roundtrip_publish_firmware() {
    assert_call_roundtrip(r#"[2,"t","PublishFirmware",{"location":"x","checksum":"x","requestId":0}]"#);
}

#[test]
fn roundtrip_publish_firmware_status_notification() {
    assert_call_roundtrip(r#"[2,"t","PublishFirmwareStatusNotification",{"status":"Idle"}]"#);
}

#[test]
fn roundtrip_pull_dynamic_schedule_update() {
    assert_call_roundtrip(r#"[2,"t","PullDynamicScheduleUpdate",{"chargingProfileId":0}]"#);
}

#[test]
fn roundtrip_report_charging_profiles() {
    assert_call_roundtrip(r#"[2,"t","ReportChargingProfiles",{"requestId":0,"chargingLimitSource":"x","chargingProfile":[{"id":0,"stackLevel":0,"chargingProfilePurpose":"ChargingStationExternalConstraints","chargingProfileKind":"Absolute","chargingSchedule":[{"id":0,"chargingRateUnit":"W","chargingSchedulePeriod":[{"startPeriod":0}]}]}],"evseId":0}]"#);
}

#[test]
fn roundtrip_report_der_control() {
    assert_call_roundtrip(r#"[2,"t","ReportDERControl",{"requestId":0}]"#);
}

#[test]
fn roundtrip_request_battery_swap() {
    assert_call_roundtrip(r#"[2,"t","RequestBatterySwap",{"idToken":{"idToken":"x","type":"x"},"requestId":0}]"#);
}

#[test]
fn roundtrip_request_start_transaction() {
    assert_call_roundtrip(r#"[2,"t","RequestStartTransaction",{"idToken":{"idToken":"x","type":"x"},"remoteStartId":0}]"#);
}

#[test]
fn roundtrip_request_stop_transaction() {
    assert_call_roundtrip(r#"[2,"t","RequestStopTransaction",{"transactionId":"x"}]"#);
}

#[test]
fn roundtrip_reservation_status_update() {
    assert_call_roundtrip(r#"[2,"t","ReservationStatusUpdate",{"reservationId":0,"reservationUpdateStatus":"Expired"}]"#);
}

#[test]
fn roundtrip_reserve_now() {
    assert_call_roundtrip(r#"[2,"t","ReserveNow",{"id":0,"expiryDateTime":"2024-01-01T00:00:00.000Z","idToken":{"idToken":"x","type":"x"}}]"#);
}

#[test]
fn roundtrip_reset() {
    assert_call_roundtrip(r#"[2,"t","Reset",{"type":"Immediate"}]"#);
}

#[test]
fn roundtrip_security_event_notification() {
    assert_call_roundtrip(r#"[2,"t","SecurityEventNotification",{"type":"x","timestamp":"2024-01-01T00:00:00.000Z"}]"#);
}

#[test]
fn roundtrip_send_local_list() {
    assert_call_roundtrip(r#"[2,"t","SendLocalList",{"versionNumber":0,"updateType":"Differential"}]"#);
}

#[test]
fn roundtrip_set_charging_profile() {
    assert_call_roundtrip(r#"[2,"t","SetChargingProfile",{"evseId":0,"chargingProfile":{"id":0,"stackLevel":0,"chargingProfilePurpose":"ChargingStationExternalConstraints","chargingProfileKind":"Absolute","chargingSchedule":[{"id":0,"chargingRateUnit":"W","chargingSchedulePeriod":[{"startPeriod":0}]}]}}]"#);
}

#[test]
fn roundtrip_set_der_control() {
    assert_call_roundtrip(r#"[2,"t","SetDERControl",{"isDefault":false,"controlId":"x","controlType":"EnterService"}]"#);
}

#[test]
fn roundtrip_set_default_tariff() {
    assert_call_roundtrip(r#"[2,"t","SetDefaultTariff",{"evseId":0,"tariff":{"tariffId":"x","currency":"x"}}]"#);
}

#[test]
fn roundtrip_set_display_message() {
    assert_call_roundtrip(r#"[2,"t","SetDisplayMessage",{"message":{"id":0,"priority":"AlwaysFront","message":{"format":"ASCII","content":"x"}}}]"#);
}

#[test]
fn roundtrip_set_monitoring_base() {
    assert_call_roundtrip(r#"[2,"t","SetMonitoringBase",{"monitoringBase":"All"}]"#);
}

#[test]
fn roundtrip_set_monitoring_level() {
    assert_call_roundtrip(r#"[2,"t","SetMonitoringLevel",{"severity":0}]"#);
}

#[test]
fn roundtrip_set_network_profile() {
    assert_call_roundtrip(r#"[2,"t","SetNetworkProfile",{"configurationSlot":0,"connectionData":{"ocppInterface":"Wired0","ocppTransport":"SOAP","messageTimeout":0,"ocppCsmsUrl":"x","securityProfile":0}}]"#);
}

#[test]
fn roundtrip_set_variable_monitoring() {
    assert_call_roundtrip(r#"[2,"t","SetVariableMonitoring",{"setMonitoringData":[{"value":0.0,"type":"UpperThreshold","severity":0,"component":{"name":"x"},"variable":{"name":"x"}}]}]"#);
}

#[test]
fn roundtrip_set_variables() {
    assert_call_roundtrip(r#"[2,"t","SetVariables",{"setVariableData":[{"attributeValue":"x","component":{"name":"x"},"variable":{"name":"x"}}]}]"#);
}

#[test]
fn roundtrip_sign_certificate() {
    assert_call_roundtrip(r#"[2,"t","SignCertificate",{"csr":"x"}]"#);
}

#[test]
fn roundtrip_status_notification() {
    assert_call_roundtrip(r#"[2,"t","StatusNotification",{"timestamp":"2024-01-01T00:00:00.000Z","connectorStatus":"Available","evseId":0,"connectorId":0}]"#);
}

#[test]
fn roundtrip_transaction_event() {
    assert_call_roundtrip(r#"[2,"t","TransactionEvent",{"eventType":"Ended","timestamp":"2024-01-01T00:00:00.000Z","triggerReason":"AbnormalCondition","seqNo":0,"transactionInfo":{"transactionId":"x"}}]"#);
}

#[test]
fn roundtrip_trigger_message() {
    assert_call_roundtrip(r#"[2,"t","TriggerMessage",{"requestedMessage":"BootNotification"}]"#);
}

#[test]
fn roundtrip_unlock_connector() {
    assert_call_roundtrip(r#"[2,"t","UnlockConnector",{"evseId":0,"connectorId":0}]"#);
}

#[test]
fn roundtrip_unpublish_firmware() {
    assert_call_roundtrip(r#"[2,"t","UnpublishFirmware",{"checksum":"x"}]"#);
}

#[test]
fn roundtrip_update_dynamic_schedule() {
    assert_call_roundtrip(r#"[2,"t","UpdateDynamicSchedule",{"chargingProfileId":0,"scheduleUpdate":{}}]"#);
}

#[test]
fn roundtrip_update_firmware() {
    assert_call_roundtrip(r#"[2,"t","UpdateFirmware",{"requestId":0,"firmware":{"location":"x","retrieveDateTime":"2024-01-01T00:00:00.000Z"}}]"#);
}

#[test]
fn roundtrip_use_priority_charging() {
    assert_call_roundtrip(r#"[2,"t","UsePriorityCharging",{"transactionId":"x","activate":false}]"#);
}

#[test]
fn roundtrip_vat_number_validation() {
    assert_call_roundtrip(r#"[2,"t","VatNumberValidation",{"vatNumber":"x"}]"#);
}

#[test]
fn roundtrip_notify_periodic_event_stream_send() {
    assert_call_roundtrip(r#"[6,"t","NotifyPeriodicEventStream",{"data":[{"t":0.0,"v":"x"}],"id":0,"pending":0,"basetime":"2024-01-01T00:00:00.000Z"}]"#);
}
