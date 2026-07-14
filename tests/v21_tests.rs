//! OCPP 2.1 framing and CallResult correlation tests.

use chrono::{TimeZone, Utc};
use ocpp_rs::v21::call::{Action, Call};
use ocpp_rs::v21::call_error::CallError;
use ocpp_rs::v21::call_result_error::CallResultError;
use ocpp_rs::v21::datatypes::{CustomDataType, DateTimeWrapper};
use ocpp_rs::v21::messages::boot_notification::ChargingStationType;
use ocpp_rs::v21::messages::boot_notification::{
    BootNotificationRequest, BootNotificationResponse, BootReasonEnumType,
    RegistrationStatusEnumType,
};
use ocpp_rs::v21::messages::clear_cache::ClearCacheRequest;
use ocpp_rs::v21::messages::clear_cache::ClearCacheStatusEnumType;
use ocpp_rs::v21::messages::heartbeat::{HeartbeatRequest, HeartbeatResponse};
use ocpp_rs::v21::messages::status_notification::StatusNotificationResponse;
use ocpp_rs::v21::call_result::CallResultRaw;
use ocpp_rs::v21::parse::{self, Message, TypedMessage};
use ocpp_rs::v21::pending::PendingCalls;
use ocpp_rs::v21::response_trait::Response;
use ocpp_rs::v21::send::{Send, SendAction};
use ocpp_rs::v21::typed_call_result::TypedCallResult;
use std::collections::BTreeMap;

#[test]
fn boot_notification_call_roundtrip() {
    let call = Call::new(
        "19223201".into(),
        Action::BootNotification(BootNotificationRequest {
            reason: BootReasonEnumType::PowerUp,
            charging_station: ChargingStationType {
                model: "SingleSocketCharger".into(),
                vendor_name: "VendorX".into(),
                serial_number: None,
                modem: None,
                firmware_version: None,
                custom_data: None,
            },
            custom_data: None,
        }),
    );
    let json = parse::serialize_message(&Message::Call(call)).unwrap();
    let msg = parse::deserialize_to_message(&json).unwrap();
    match msg {
        Message::Call(c) => match c.payload {
            Action::BootNotification(req) => {
                assert_eq!(req.reason, BootReasonEnumType::PowerUp);
                assert_eq!(req.charging_station.vendor_name, "VendorX");
            }
            other => panic!("unexpected action: {other:?}"),
        },
        other => panic!("unexpected message: {other:?}"),
    }
}

#[test]
fn boot_notification_sample_from_part4() {
    let incoming = r#"[2, "19223201", "BootNotification", {"reason": "PowerUp", "chargingStation": {"model": "SingleSocketCharger", "vendorName": "VendorX"}}]"#;
    let msg = parse::deserialize_to_message(incoming).unwrap();
    assert!(matches!(
        msg,
        Message::Call(Call {
            payload: Action::BootNotification(_),
            ..
        })
    ));
}

#[test]
fn call_result_empty_is_not_ambiguous_with_pending() {
    // Same wire payload `{}` must resolve to different typed responses based on pending Action.
    let empty = r#"[3, "id-1", {}]"#;

    let mut pending = PendingCalls::new();
    pending.register(
        "id-1",
        Action::StatusNotification(ocpp_rs::v21::messages::status_notification::StatusNotificationRequest {
            timestamp: DateTimeWrapper::new(Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap()),
            connector_status:
                ocpp_rs::v21::messages::status_notification::ConnectorStatusEnumType::Available,
            evse_id: 1,
            connector_id: 1,
            custom_data: None,
        }),
    );
    let typed = pending.deserialize_typed(empty).unwrap();
    match typed {
        TypedMessage::CallResult(TypedCallResult::StatusNotification(cr)) => {
            assert!(cr.payload.custom_data.is_none());
            let _: StatusNotificationResponse = cr.payload;
        }
        other => panic!("expected StatusNotification response, got {other:?}"),
    }

    let mut pending = PendingCalls::new();
    pending.register(
        "id-1",
        Action::Heartbeat(HeartbeatRequest {
            custom_data: None,
        }),
    );
    // HeartbeatResponse requires currentTime — empty object must fail, not silently become another type
    let err = pending.deserialize_typed(empty).unwrap_err();
    assert!(matches!(err, ocpp_rs::errors::Error::SerdeJson(_)));
}

#[test]
fn clear_cache_status_only_resolves_via_pending() {
    let wire = r#"[3, "cc-1", {"status": "Accepted"}]"#;
    let mut pending = PendingCalls::new();
    pending.register("cc-1", Action::ClearCache(ClearCacheRequest { custom_data: None }));
    match pending.deserialize_typed(wire).unwrap() {
        TypedMessage::CallResult(TypedCallResult::ClearCache(cr)) => {
            assert_eq!(cr.payload.status, ClearCacheStatusEnumType::Accepted);
        }
        other => panic!("unexpected {other:?}"),
    }
}

#[test]
fn unknown_pending_message_id_errors() {
    let wire = r#"[3, "missing", {}]"#;
    let mut pending = PendingCalls::new();
    let err = pending.deserialize_typed(wire).unwrap_err();
    assert!(matches!(
        err,
        ocpp_rs::errors::Error::UnknownPendingMessageId(_)
    ));
}

#[test]
fn response_trait_builds_call_result() {
    let req = BootNotificationRequest {
        reason: BootReasonEnumType::PowerUp,
        charging_station: ChargingStationType {
            model: "M".into(),
            vendor_name: "V".into(),
            serial_number: None,
            modem: None,
            firmware_version: None,
            custom_data: None,
        },
        custom_data: None,
    };
    let resp = BootNotificationResponse {
        current_time: DateTimeWrapper::new(Utc.with_ymd_and_hms(2013, 2, 1, 20, 53, 32).unwrap()),
        interval: 300,
        status: RegistrationStatusEnumType::Accepted,
        status_info: None,
        custom_data: None,
    };
    let msg = req.get_response("19223201".into(), resp).expect("serialize response");
    let json = parse::serialize_message(&msg).unwrap();
    assert!(json.starts_with("[3,"));
    assert!(json.contains("Accepted"));
}

#[test]
fn call_error_and_call_result_error_and_send() {
    let err = CallError::new(
        "e1".into(),
        "NotSupported".into(),
        "nope".into(),
        BTreeMap::new(),
    );
    let json = parse::serialize_message(&Message::CallError(err)).unwrap();
    assert!(matches!(
        parse::deserialize_to_message(&json).unwrap(),
        Message::CallError(_)
    ));

    let err5 = CallResultError::new(
        "e2".into(),
        "GenericError".into(),
        "bad result".into(),
        BTreeMap::new(),
    );
    let json = parse::serialize_message(&Message::CallResultError(err5)).unwrap();
    assert!(matches!(
        parse::deserialize_to_message(&json).unwrap(),
        Message::CallResultError(_)
    ));

    // SEND NotifyPeriodicEventStream — minimal valid payload
    use ocpp_rs::v21::messages::notify_periodic_event_stream::{
        NotifyPeriodicEventStream, StreamDataElementType,
    };
    let send = Send::new(
        "s1".into(),
        SendAction::NotifyPeriodicEventStream(NotifyPeriodicEventStream {
            id: 1,
            pending: 0,
            basetime: DateTimeWrapper::new(Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap()),
            data: vec![StreamDataElementType {
                t: 0.0,
                v: "1".into(),
                custom_data: None,
            }],
            custom_data: None,
        }),
    );
    let json = parse::serialize_message(&Message::Send(send)).unwrap();
    assert!(matches!(
        parse::deserialize_to_message(&json).unwrap(),
        Message::Send(_)
    ));
}

#[test]
fn send_call_registers_for_later_result() {
    let mut pending = PendingCalls::new();
    let call = Call::new(
        "hb-1".into(),
        Action::Heartbeat(HeartbeatRequest { custom_data: None }),
    );
    let _ = pending.send_call(call).unwrap();
    assert_eq!(pending.len(), 1);

    let wire = r#"[3, "hb-1", {"currentTime": "2024-01-01T00:00:00.000Z"}]"#;
    match pending.deserialize_typed(wire).unwrap() {
        TypedMessage::CallResult(TypedCallResult::Heartbeat(cr)) => {
            let _: HeartbeatResponse = cr.payload;
        }
        other => panic!("unexpected {other:?}"),
    }
    assert!(pending.is_empty());
}

#[test]
fn custom_data_roundtrip() {
    let req = HeartbeatRequest {
        custom_data: Some(CustomDataType {
            vendor_id: "VendorX".into(),
        }),
    };
    let json = serde_json::to_string(&req).unwrap();
    let back: HeartbeatRequest = serde_json::from_str(&json).unwrap();
    assert_eq!(back.custom_data.unwrap().vendor_id, "VendorX");
}

#[test]
fn unsupported_message_type() {
    let err = parse::deserialize_to_message(r#"[9, "x", {}]"#).unwrap_err();
    assert!(matches!(
        err,
        ocpp_rs::errors::Error::InvalidMessageCallType
            | ocpp_rs::errors::Error::UnsupportedMessageType(_)
    ));
}

#[test]
fn pending_action_names_and_resolve_with_action_name() {
    use ocpp_rs::v21::pending::{resolve_with_action_name, PendingActionNames};

    let mut names = PendingActionNames::new();
    names
        .send_call(Call::new(
            "hb".into(),
            Action::Heartbeat(HeartbeatRequest { custom_data: None }),
        ))
        .unwrap();
    match names
        .deserialize_typed(r#"[3, "hb", {"currentTime": "2024-01-01T00:00:00.000Z"}]"#)
        .unwrap()
    {
        TypedMessage::CallResult(TypedCallResult::Heartbeat(_)) => {}
        other => panic!("{other:?}"),
    }

    // Simulate Redis: only the action string is available on another node
    let raw = match parse::deserialize_to_message(
        r#"[3, "n1", {"currentTime": "2024-06-01T12:00:00.000Z"}]"#,
    )
    .unwrap()
    {
        Message::CallResult(r) => r,
        other => panic!("{other:?}"),
    };
    let typed = resolve_with_action_name(raw, "Heartbeat").unwrap();
    assert!(matches!(typed, TypedCallResult::Heartbeat(_)));

    let err = resolve_with_action_name(
        CallResultRaw::new("x".into(), serde_json::json!({})),
        "NotARealAction",
    )
    .unwrap_err();
    assert!(matches!(err, ocpp_rs::errors::Error::UnknownActionName(_)));
}

#[test]
fn probe_and_try_resolve_unique_when_type_unknown() {
    use ocpp_rs::v21::pending::try_resolve_unique;

    let raw = CallResultRaw::new(
        "u".into(),
        serde_json::json!({"currentTime": "2024-01-01T00:00:00.000Z"}),
    );
    let unique = try_resolve_unique(&raw).unwrap();
    assert_eq!(unique.action_name(), "Heartbeat");

    let empty = CallResultRaw::new("e".into(), serde_json::json!({}));
    let candidates = empty.probe_candidates();
    assert!(
        candidates.len() > 1,
        "empty object should match multiple response schemas, got {}",
        candidates.len()
    );
    let err = try_resolve_unique(&empty).unwrap_err();
    assert!(matches!(err, ocpp_rs::errors::Error::AmbiguousCallResult(_)));
}
