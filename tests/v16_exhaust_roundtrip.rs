//! AUTO-generated exhaustive v16 CALL round-trips. Regenerate: python tools/gen_exhaust_tests.py
use ocpp_rs::v16::parse::{deserialize_to_message, serialize_message, Message};

fn assert_call_roundtrip(wire: &str) {
    let msg = deserialize_to_message(wire).unwrap_or_else(|e| panic!("parse {wire}: {e}"));
    assert!(matches!(msg, Message::Call(_)), "expected Call: {msg:?}");
    let again = serialize_message(&msg).expect("serialize");
    let back = deserialize_to_message(&again).expect("reparse");
    assert_eq!(msg, back);
}

#[test]
fn roundtrip_authorize() {
    assert_call_roundtrip(r#"[2,"t","Authorize",{"idTag":"x"}]"#);
}

#[test]
fn roundtrip_boot_notification() {
    assert_call_roundtrip(r#"[2,"t","BootNotification",{"chargePointVendor":"x","chargePointModel":"x"}]"#);
}

#[test]
fn roundtrip_cancel_reservation() {
    assert_call_roundtrip(r#"[2,"t","CancelReservation",{"reservationId":0}]"#);
}

#[test]
fn roundtrip_change_availability() {
    assert_call_roundtrip(r#"[2,"t","ChangeAvailability",{"connectorId":0,"type":"Inoperative"}]"#);
}

#[test]
fn roundtrip_change_configuration() {
    assert_call_roundtrip(r#"[2,"t","ChangeConfiguration",{"key":"x","value":"x"}]"#);
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
fn roundtrip_data_transfer() {
    assert_call_roundtrip(r#"[2,"t","DataTransfer",{"vendorId":"x"}]"#);
}

#[test]
fn roundtrip_diagnostics_status_notification() {
    assert_call_roundtrip(r#"[2,"t","DiagnosticsStatusNotification",{"status":"Idle"}]"#);
}

#[test]
fn roundtrip_firmware_status_notification() {
    assert_call_roundtrip(r#"[2,"t","FirmwareStatusNotification",{"status":"Downloaded"}]"#);
}

#[test]
fn roundtrip_get_composite_schedule() {
    assert_call_roundtrip(r#"[2,"t","GetCompositeSchedule",{"connectorId":0,"duration":0}]"#);
}

#[test]
fn roundtrip_get_configuration() {
    assert_call_roundtrip(r#"[2,"t","GetConfiguration",{}]"#);
}

#[test]
fn roundtrip_get_diagnostics() {
    assert_call_roundtrip(r#"[2,"t","GetDiagnostics",{"location":"x"}]"#);
}

#[test]
fn roundtrip_get_local_list_version() {
    assert_call_roundtrip(r#"[2,"t","GetLocalListVersion",{}]"#);
}

#[test]
fn roundtrip_heartbeat() {
    assert_call_roundtrip(r#"[2,"t","Heartbeat",{}]"#);
}

#[test]
fn roundtrip_meter_values() {
    assert_call_roundtrip(r#"[2,"t","MeterValues",{"connectorId":0,"meterValue":[]}]"#);
}

#[test]
fn roundtrip_remote_start_transaction() {
    assert_call_roundtrip(r#"[2,"t","RemoteStartTransaction",{"idTag":"x"}]"#);
}

#[test]
fn roundtrip_remote_stop_transaction() {
    assert_call_roundtrip(r#"[2,"t","RemoteStopTransaction",{"transactionId":0}]"#);
}

#[test]
fn roundtrip_reserve_now() {
    assert_call_roundtrip(r#"[2,"t","ReserveNow",{"connectorId":0,"expiryDate":"2024-01-01T00:00:00.000Z","idTag":"x","reservationId":0}]"#);
}

#[test]
fn roundtrip_reset() {
    assert_call_roundtrip(r#"[2,"t","Reset",{"type":"Hard"}]"#);
}

#[test]
fn roundtrip_send_local_list() {
    assert_call_roundtrip(r#"[2,"t","SendLocalList",{"listVersion":0,"updateType":"Differential"}]"#);
}

#[test]
fn roundtrip_set_charging_profile() {
    assert_call_roundtrip(r#"[2,"t","SetChargingProfile",{"connectorId":0,"csChargingProfiles":{"chargingProfileId":0,"stackLevel":0,"chargingProfilePurpose":"ChargePointMaxProfile","chargingProfileKind":"Absolute","chargingSchedule":{"chargingRateUnit":"A","chargingSchedulePeriod":[]}}}]"#);
}

#[test]
fn roundtrip_start_transaction() {
    assert_call_roundtrip(r#"[2,"t","StartTransaction",{"connectorId":0,"idTag":"x","meterStart":0,"timestamp":"2024-01-01T00:00:00.000Z"}]"#);
}

#[test]
fn roundtrip_status_notification() {
    assert_call_roundtrip(r#"[2,"t","StatusNotification",{"connectorId":0,"errorCode":"ConnectorLockFailure","status":"Available"}]"#);
}

#[test]
fn roundtrip_stop_transaction() {
    assert_call_roundtrip(r#"[2,"t","StopTransaction",{"meterStop":0,"timestamp":"2024-01-01T00:00:00.000Z","transactionId":0}]"#);
}

#[test]
fn roundtrip_trigger_message() {
    assert_call_roundtrip(r#"[2,"t","TriggerMessage",{"requestedMessage":"BootNotification"}]"#);
}

#[test]
fn roundtrip_unlock_connector() {
    assert_call_roundtrip(r#"[2,"t","UnlockConnector",{"connectorId":0}]"#);
}

#[test]
fn roundtrip_update_firmware() {
    assert_call_roundtrip(r#"[2,"t","UpdateFirmware",{"location":"x","retrieveDate":"2024-01-01T00:00:00.000Z"}]"#);
}

#[test]
fn roundtrip_security_get_log() {
    assert_call_roundtrip(r#"[2,"t","GetLog",{"logType":"DiagnosticsLog","requestId":1,"log":{"remoteLocation":"ftp://x"}}]"#);
}

#[test]
fn roundtrip_security_delete_certificate() {
    assert_call_roundtrip(r#"[2,"t","DeleteCertificate",{"certificateHashData":{"hashAlgorithm":"SHA256","issuerNameHash":"a","issuerKeyHash":"b","serialNumber":"1"}}]"#);
}

#[test]
fn roundtrip_security_install_certificate() {
    assert_call_roundtrip(r#"[2,"t","InstallCertificate",{"certificateType":"CentralSystemRootCertificate","certificate":"PEM"}]"#);
}

#[test]
fn roundtrip_security_get_installed_certificate_ids() {
    assert_call_roundtrip(r#"[2,"t","GetInstalledCertificateIds",{"certificateType":"ManufacturerRootCertificate"}]"#);
}

#[test]
fn roundtrip_security_signed_update_firmware() {
    assert_call_roundtrip(r#"[2,"t","SignedUpdateFirmware",{"requestId":1,"firmware":{"location":"https://x","retrieveDateTime":"2024-01-01T00:00:00.000Z","signingCertificate":"C","signature":"S"}}]"#);
}

#[test]
fn roundtrip_security_security_event_notification() {
    assert_call_roundtrip(r#"[2,"t","SecurityEventNotification",{"type":"FirmwareUpdated","timestamp":"2024-01-01T00:00:00.000Z"}]"#);
}

#[test]
fn roundtrip_security_extended_trigger_message() {
    assert_call_roundtrip(r#"[2,"t","ExtendedTriggerMessage",{"requestedMessage":"BootNotification"}]"#);
}

#[test]
fn roundtrip_security_certificate_signed() {
    assert_call_roundtrip(r#"[2,"t","CertificateSigned",{"certificateChain":"CERT"}]"#);
}

#[test]
fn roundtrip_security_sign_certificate() {
    assert_call_roundtrip(r#"[2,"t","SignCertificate",{"csr":"CSR"}]"#);
}

#[test]
fn roundtrip_security_log_status_notification() {
    assert_call_roundtrip(r#"[2,"t","LogStatusNotification",{"status":"Uploaded","requestId":1}]"#);
}

#[test]
fn roundtrip_security_signed_firmware_status_notification() {
    assert_call_roundtrip(r#"[2,"t","SignedFirmwareStatusNotification",{"status":"Downloaded"}]"#);
}
