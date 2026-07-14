//! Datetime serialize feature tests.
//!
//! Parse always accepts RFC3339 (including `%.3fZ`).
//! Serialize defaults to `%Y-%m-%dT%H:%M:%S%.3fZ`; enable `datetime_serialize_rfc3339`
//! for chrono RFC3339 millis emit.

use chrono::{TimeZone, Utc};
use ocpp_rs::datetime::DateTimeWrapper;
use ocpp_rs::v16::call_result::Heartbeat;
use ocpp_rs::v16::parse::{deserialize_to_message, serialize_message};
use ocpp_rs::v16::response_trait::Response;

fn status_call_json(ts: &str) -> String {
    format!(
        r#"[2, "dt", "StatusNotification", {{"connectorId":1,"errorCode":"NoError","status":"Available","timestamp":"{ts}"}}]"#
    )
}

#[test]
fn parses_strict_millis_z() {
    let data = status_call_json("2024-06-01T19:52:45.123Z");
    assert!(deserialize_to_message(&data).is_ok());
}

#[test]
fn parses_rfc3339_offset() {
    let data = status_call_json("2024-06-01T19:52:45.123+00:00");
    assert!(deserialize_to_message(&data).is_ok());
}

#[test]
fn parses_rfc3339_without_fractional_seconds() {
    let data = status_call_json("2024-06-01T19:52:45Z");
    assert!(deserialize_to_message(&data).is_ok());
}

#[test]
fn default_serialize_uses_three_fractional_digits() {
    let req = ocpp_rs::v16::call::Heartbeat {};
    let dt = DateTimeWrapper::new(Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
    let msg = req
        .get_response("1".into(), Heartbeat { current_time: dt })
        .expect("serialize response");
    let wire = serialize_message(&msg).expect("serialize");
    assert!(
        wire.contains("2024-01-01T00:00:00.000Z"),
        "expected %.3fZ in {wire}"
    );
}

#[cfg(feature = "datetime_serialize_rfc3339")]
#[test]
fn serialize_rfc3339_emits_millis_z() {
    let req = ocpp_rs::v16::call::Heartbeat {};
    let dt = DateTimeWrapper::new(Utc.with_ymd_and_hms(2024, 6, 15, 12, 30, 45).unwrap());
    let msg = req
        .get_response("1".into(), Heartbeat { current_time: dt })
        .expect("serialize response");
    let wire = serialize_message(&msg).expect("serialize");
    // For Utc + millis, RFC3339 opts match %.3fZ shape.
    assert!(wire.contains("2024-06-15T12:30:45.000Z"), "{wire}");
}
