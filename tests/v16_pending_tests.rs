//! OCPP 1.6 PendingCalls / action-name / probe tests.

use ocpp_rs::v16::call::{Action, Call, ClearCache, Heartbeat, StatusNotification};
use ocpp_rs::v16::call_result::{CallResultRaw, ClearCache as ClearCacheResult, EmptyResponse};
use ocpp_rs::v16::enums::{ChargePointErrorCode, ChargePointStatus, ClearCacheStatus};
use ocpp_rs::v16::parse::{self, Message, TypedMessage};
use ocpp_rs::v16::pending::{
    resolve_with_action_name, try_resolve_unique, PendingActionNames, PendingCalls,
};
use ocpp_rs::v16::typed_call_result::TypedCallResult;

#[test]
fn pending_action_names_roundtrip() {
    let mut names = PendingActionNames::new();
    names
        .send_call(Call::new("hb".into(), Action::Heartbeat(Heartbeat {})))
        .unwrap();
    match names
        .deserialize_typed(r#"[3, "hb", {"currentTime": "2024-01-01T00:00:00.000Z"}]"#)
        .unwrap()
    {
        TypedMessage::CallResult(TypedCallResult::Heartbeat(_)) => {}
        other => panic!("{other:?}"),
    }
}

#[test]
fn resolve_with_action_name_from_shared_store() {
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
}

#[test]
fn empty_object_is_ambiguous_without_pending() {
    let empty = CallResultRaw::new("e".into(), serde_json::json!({}));
    assert!(empty.probe_candidates().len() > 1);
    assert!(matches!(
        try_resolve_unique(&empty).unwrap_err(),
        ocpp_rs::errors::Error::AmbiguousCallResult(_)
    ));
}

#[test]
fn status_only_resolves_via_pending_clear_cache() {
    let mut pending = PendingCalls::new();
    pending.register("cc", Action::ClearCache(ClearCache {}));
    match pending
        .deserialize_typed(r#"[3, "cc", {"status": "Accepted"}]"#)
        .unwrap()
    {
        TypedMessage::CallResult(TypedCallResult::ClearCache(cr)) => {
            assert_eq!(cr.payload.status, ClearCacheStatus::Accepted);
            let _: ClearCacheResult = cr.payload;
        }
        other => panic!("{other:?}"),
    }
}

#[test]
fn unknown_pending_id_errors() {
    let mut pending = PendingCalls::new();
    let err = pending
        .deserialize_typed(r#"[3, "missing", {}]"#)
        .unwrap_err();
    assert!(matches!(
        err,
        ocpp_rs::errors::Error::UnknownPendingMessageId(_)
    ));
}

#[test]
fn status_notification_empty_via_pending() {
    let mut pending = PendingCalls::new();
    pending.register(
        "s1",
        Action::StatusNotification(StatusNotification {
            connector_id: 1,
            error_code: ChargePointErrorCode::NoError,
            status: ChargePointStatus::Available,
            ..Default::default()
        }),
    );
    match pending.deserialize_typed(r#"[3, "s1", {}]"#).unwrap() {
        TypedMessage::CallResult(TypedCallResult::StatusNotification(cr)) => {
            let _: EmptyResponse = cr.payload;
        }
        other => panic!("{other:?}"),
    }
}
