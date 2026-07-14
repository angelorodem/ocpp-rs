//! Schema-shaped round-trips for nested OCPP 1.6 types fixed in 0.3.

use ocpp_rs::v16::call::{
    Action, RemoteStartTransaction, SendLocalList, SetChargingProfile,
};
use ocpp_rs::v16::call_result::{self, CallResultRaw};
use ocpp_rs::v16::data_types::{
    AuthorizationData, ChargingProfile, ChargingSchedule, ChargingSchedulePeriod, IdTagInfo,
};
use ocpp_rs::v16::enums::{
    AuthorizationStatus, ChargingProfileKindType, ChargingProfilePurposeType, ChargingRateUnitType,
    GetCompositeScheduleStatus, UnitOfMeasure, UpdateType,
};
use ocpp_rs::v16::parse::{deserialize_to_message, serialize_message, Message};
use ocpp_rs::v16::pending::resolve_with_action_name;

#[test]
fn send_local_list_with_authorization_data() {
    let data = r#"[2, "ll1", "SendLocalList", {
        "listVersion": 2,
        "updateType": "Full",
        "localAuthorizationList": [
            {"idTag": "TAG1", "idTagInfo": {"status": "Accepted"}},
            {"idTag": "TAG2"}
        ]
    }]"#;
    let message = deserialize_to_message(data).expect("parse");
    match message {
        Message::Call(call) => match call.payload {
            Action::SendLocalList(SendLocalList {
                list_version,
                update_type,
                local_authorization_list,
            }) => {
                assert_eq!(list_version, 2);
                assert_eq!(update_type, UpdateType::Full);
                let list = local_authorization_list.expect("list present");
                assert_eq!(list.len(), 2);
                assert_eq!(list[0].id_tag, "TAG1");
                assert_eq!(
                    list[0]
                        .id_tag_info
                        .as_ref()
                        .map(|i| &i.status),
                    Some(&AuthorizationStatus::Accepted)
                );
                assert_eq!(list[1].id_tag, "TAG2");
                assert!(list[1].id_tag_info.is_none());
            }
            other => panic!("unexpected action: {other:?}"),
        },
        other => panic!("unexpected message: {other:?}"),
    }
}

#[test]
fn send_local_list_omits_optional_list() {
    let data = r#"[2, "ll2", "SendLocalList", {
        "listVersion": 1,
        "updateType": "Differential"
    }]"#;
    let message = deserialize_to_message(data).expect("parse");
    match message {
        Message::Call(call) => match call.payload {
            Action::SendLocalList(SendLocalList {
                local_authorization_list,
                ..
            }) => assert!(local_authorization_list.is_none()),
            other => panic!("unexpected action: {other:?}"),
        },
        other => panic!("unexpected message: {other:?}"),
    }
}

#[test]
fn set_charging_profile_nested_object() {
    let data = r#"[2, "scp1", "SetChargingProfile", {
        "connectorId": 1,
        "csChargingProfiles": {
            "chargingProfileId": 10,
            "stackLevel": 0,
            "chargingProfilePurpose": "TxDefaultProfile",
            "chargingProfileKind": "Absolute",
            "chargingSchedule": {
                "chargingRateUnit": "A",
                "chargingSchedulePeriod": [{"startPeriod": 0, "limit": 16.0}]
            }
        }
    }]"#;
    let message = deserialize_to_message(data).expect("parse");
    match message {
        Message::Call(call) => match call.payload {
            Action::SetChargingProfile(SetChargingProfile {
                connector_id,
                cs_charging_profiles,
            }) => {
                assert_eq!(connector_id, 1);
                assert_eq!(cs_charging_profiles.charging_profile_id, 10);
                assert_eq!(
                    cs_charging_profiles.charging_profile_purpose,
                    ChargingProfilePurposeType::TxDefaultProfile
                );
                assert_eq!(
                    cs_charging_profiles.charging_schedule.charging_rate_unit,
                    ChargingRateUnitType::A
                );
                assert_eq!(
                    cs_charging_profiles.charging_schedule.charging_schedule_period[0].limit,
                    16.0
                );
            }
            other => panic!("unexpected action: {other:?}"),
        },
        other => panic!("unexpected message: {other:?}"),
    }
}

#[test]
fn remote_start_with_charging_profile_roundtrip() {
    let profile = ChargingProfile {
        charging_profile_id: 1,
        transaction_id: None,
        stack_level: 0,
        charging_profile_purpose: ChargingProfilePurposeType::TxProfile,
        charging_profile_kind: ChargingProfileKindType::Relative,
        recurrency_kind: None,
        valid_from: None,
        valid_to: None,
        charging_schedule: ChargingSchedule {
            duration: None,
            start_schedule: None,
            charging_rate_unit: ChargingRateUnitType::W,
            charging_schedule_period: vec![ChargingSchedulePeriod {
                start_period: 0,
                limit: 11000.0,
                number_phases: Some(3),
            }],
            min_charging_rate: None,
        },
    };
    let call = ocpp_rs::v16::call::Call::new(
        "rs1".into(),
        Action::RemoteStartTransaction(RemoteStartTransaction {
            id_tag: "RFID".into(),
            connector_id: Some(1),
            charging_profile: Some(profile),
        }),
    );
    let wire = serialize_message(&Message::Call(call)).expect("serialize");
    let again = deserialize_to_message(&wire).expect("parse");
    match again {
        Message::Call(c) => match c.payload {
            Action::RemoteStartTransaction(RemoteStartTransaction {
                charging_profile: Some(p),
                ..
            }) => {
                assert_eq!(p.charging_profile_purpose, ChargingProfilePurposeType::TxProfile);
                assert_eq!(p.charging_schedule.charging_schedule_period[0].start_period, 0);
            }
            other => panic!("unexpected: {other:?}"),
        },
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn get_composite_schedule_response_with_schedule() {
    let raw = CallResultRaw::new(
        "gcs1".into(),
        serde_json::json!({
            "status": "Accepted",
            "connectorId": 1,
            "scheduleStart": "2024-06-01T10:00:00.000Z",
            "chargingSchedule": {
                "chargingRateUnit": "A",
                "chargingSchedulePeriod": [{"startPeriod": 0, "limit": 32.0}]
            }
        }),
    );
    let typed = resolve_with_action_name(raw, "GetCompositeSchedule").expect("resolve");
    match typed {
        ocpp_rs::v16::typed_call_result::TypedCallResult::GetCompositeSchedule(cr) => {
            assert_eq!(cr.payload.status, GetCompositeScheduleStatus::Accepted);
            let schedule = cr.payload.charging_schedule.expect("schedule");
            assert_eq!(schedule.charging_rate_unit, ChargingRateUnitType::A);
            assert_eq!(schedule.charging_schedule_period[0].limit, 32.0);
        }
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn unit_of_measure_accepts_celcius_alias() {
    let data = r#"[2, "mv1", "MeterValues", {
        "connectorId": 1,
        "meterValue": [{
            "timestamp": "2024-06-01T10:30:00.000Z",
            "sampledValue": [{"value": "22.5", "unit": "Celcius", "measurand": "Temperature"}]
        }]
    }]"#;
    let message = deserialize_to_message(data).expect("parse Celcius");
    match message {
        Message::Call(call) => match call.payload {
            Action::MeterValues(mv) => {
                assert_eq!(
                    mv.meter_value[0].sampled_value[0].unit,
                    Some(UnitOfMeasure::Celsius)
                );
            }
            other => panic!("unexpected: {other:?}"),
        },
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn send_local_list_authorization_data_roundtrip_serialize() {
    let list = vec![AuthorizationData {
        id_tag: "ABC".into(),
        id_tag_info: Some(IdTagInfo {
            expiry_date: None,
            parent_id_tag: None,
            status: AuthorizationStatus::Blocked,
        }),
    }];
    let call = ocpp_rs::v16::call::Call::new(
        "ll3".into(),
        Action::SendLocalList(SendLocalList {
            list_version: 3,
            update_type: UpdateType::Full,
            local_authorization_list: Some(list),
        }),
    );
    let wire = serialize_message(&Message::Call(call)).expect("serialize");
    assert!(wire.contains("\"idTag\":\"ABC\""));
    assert!(wire.contains("\"Blocked\""));
    let _ = deserialize_to_message(&wire).expect("roundtrip");
}

#[test]
fn unlock_connector_status_is_flat() {
    let payload = call_result::UnlockConnector {
        status: ocpp_rs::v16::enums::UnlockStatus::Unlocked,
    };
    let json = serde_json::to_value(&payload).expect("ser");
    assert_eq!(json, serde_json::json!({"status": "Unlocked"}));
}

#[test]
fn authorize_conf_requires_id_tag_info() {
    let err = serde_json::from_value::<call_result::Authorize>(serde_json::json!({}));
    assert!(err.is_err());
    let ok: call_result::Authorize = serde_json::from_value(serde_json::json!({
        "idTagInfo": {"status": "Accepted"}
    }))
    .expect("authorize");
    assert_eq!(ok.id_tag_info.status, AuthorizationStatus::Accepted);
}

#[test]
fn stop_transaction_conf_allows_empty() {
    let ok: call_result::StopTransaction =
        serde_json::from_value(serde_json::json!({})).expect("empty stop conf");
    assert!(ok.id_tag_info.is_none());
}

#[test]
fn call_error_accepts_nested_json_details() {
    use std::collections::BTreeMap;
    use ocpp_rs::v16::call_error::CallError;
    use ocpp_rs::v16::parse::{deserialize_to_message, serialize_message, Message};

    let mut details = BTreeMap::new();
    details.insert("nested".into(), serde_json::json!({"code": 42, "ok": true}));
    let err = CallError::new(
        "e1".into(),
        "FormationViolation".into(),
        "bad payload".into(),
        details,
    );
    let wire = serialize_message(&Message::CallError(err)).expect("serialize");
    let again = deserialize_to_message(&wire).expect("parse");
    match again {
        Message::CallError(e) => {
            assert_eq!(
                e.error_details.get("nested"),
                Some(&serde_json::json!({"code": 42, "ok": true}))
            );
        }
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn signed_firmware_status_security_variants() {
    let data = r#"[2, "sf1", "SignedFirmwareStatusNotification", {
        "status": "SignatureVerified"
    }]"#;
    let message = deserialize_to_message(data).expect("parse");
    match message {
        Message::Call(call) => match call.payload {
            Action::SignedFirmwareStatusNotification(n) => {
                assert_eq!(
                    n.status,
                    ocpp_rs::v16::enums::FirmwareStatus::SignatureVerified
                );
                assert!(n.request_id.is_none());
            }
            other => panic!("unexpected: {other:?}"),
        },
        other => panic!("unexpected: {other:?}"),
    }
}
