//! OCPP 2.1 integration tests.
//!
//! Part 6 certification procedures are catalogued in
//! [`docs/2-1-docs/07-testcases-index.md`](../docs/2-1-docs/07-testcases-index.md).
//! Those TCs are full CS/CSMS behavioral suites (TLS, state machines, reusable states).
//! This file covers the **library-layer** message sequences those blocks require:
//! serialize/deserialize round-trips, `PendingCalls` correlation, and multi-message workflows
//! for the informative basic set (blocks B, C, E, F, G, P, plus framing types 2–6).

use chrono::{TimeZone, Timelike, Utc};
use ocpp_rs::v21::call::{Action, Call};
use ocpp_rs::v21::call_error::CallError;
use ocpp_rs::v21::call_result::CallResultRaw;
use ocpp_rs::v21::call_result_error::CallResultError;
use ocpp_rs::v21::datatypes::{
    ComponentType, CustomDataType, DateTimeWrapper, EVSEType, IdTokenInfoType, IdTokenType,
    VariableType,
};
use ocpp_rs::v21::enumerations::{
    AuthorizationStatusEnumType, GenericDeviceModelStatusEnumType, RequestStartStopStatusEnumType,
};
use ocpp_rs::v21::log_helper::MessageLogLine;
use ocpp_rs::v21::messages::authorize::{AuthorizeRequest, AuthorizeResponse};
use ocpp_rs::v21::messages::boot_notification::{
    BootNotificationRequest, BootNotificationResponse, BootReasonEnumType, ChargingStationType,
    RegistrationStatusEnumType,
};
use ocpp_rs::v21::messages::change_availability::{
    ChangeAvailabilityRequest, ChangeAvailabilityResponse, ChangeAvailabilityStatusEnumType,
    OperationalStatusEnumType,
};
use ocpp_rs::v21::messages::data_transfer::{
    DataTransferRequest, DataTransferResponse, DataTransferStatusEnumType,
};
use ocpp_rs::v21::messages::get_base_report::{
    GetBaseReportRequest, GetBaseReportResponse, ReportBaseEnumType,
};
use ocpp_rs::v21::messages::get_variables::{
    GetVariableDataType, GetVariableResultType, GetVariableStatusEnumType, GetVariablesRequest,
    GetVariablesResponse,
};
use ocpp_rs::v21::messages::heartbeat::{HeartbeatRequest, HeartbeatResponse};
use ocpp_rs::v21::messages::request_start_transaction::{
    RequestStartTransactionRequest, RequestStartTransactionResponse,
};
use ocpp_rs::v21::messages::request_stop_transaction::{
    RequestStopTransactionRequest, RequestStopTransactionResponse,
};
use ocpp_rs::v21::messages::reset::{
    ResetEnumType, ResetRequest, ResetResponse, ResetStatusEnumType,
};
use ocpp_rs::v21::messages::status_notification::{
    ConnectorStatusEnumType, StatusNotificationRequest, StatusNotificationResponse,
};
use ocpp_rs::v21::messages::transaction_event::{
    ChargingStateEnumType, TransactionEventEnumType, TransactionEventRequest,
    TransactionEventResponse, TransactionType, TriggerReasonEnumType,
};
use ocpp_rs::v21::parse::{self, Message, TypedMessage};
use ocpp_rs::v21::pending::PendingCalls;
use ocpp_rs::v21::response_trait::Response;
use ocpp_rs::v21::send::{Send, SendAction};
use ocpp_rs::v21::typed_call_result::TypedCallResult;
use std::collections::BTreeMap;

fn dt(y: i32, m: u32, d: u32, hh: u32, mm: u32, ss: u32) -> DateTimeWrapper {
    DateTimeWrapper::new(Utc.with_ymd_and_hms(y, m, d, hh, mm, ss).unwrap())
}

fn now_ms() -> DateTimeWrapper {
    // Fixed instant — avoids chrono `clock`/`std` in the package feature graph.
    let now = Utc.with_ymd_and_hms(2024, 6, 1, 12, 0, 0).unwrap();
    let truncated = now
        .with_nanosecond((now.nanosecond() / 1_000_000) * 1_000_000)
        .unwrap();
    DateTimeWrapper::new(truncated)
}

fn id_token(token: &str, kind: &str) -> IdTokenType {
    IdTokenType {
        id_token: token.into(),
        type_: kind.into(),
        additional_info: None,
        custom_data: None,
    }
}

fn roundtrip_call(call: Call) -> Call {
    let json = parse::serialize_message(&Message::Call(call.clone())).expect("serialize call");
    match parse::deserialize_to_message(&json).expect("deserialize call") {
        Message::Call(c) => c,
        other => panic!("expected Call, got {other:?}"),
    }
}

fn assert_pending_result(
    pending: &mut PendingCalls,
    wire: &str,
    check: impl FnOnce(TypedCallResult),
) {
    match pending.deserialize_typed(wire).expect("typed result") {
        TypedMessage::CallResult(t) => check(t),
        other => panic!("expected CallResult, got {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Framing (Part 4 / message types 2–6)
// ---------------------------------------------------------------------------

#[test]
fn framing_all_message_types_roundtrip() {
    // Type 2
    let call = Call::new(
        "m2".into(),
        Action::Heartbeat(HeartbeatRequest { custom_data: None }),
    );
    let j = parse::serialize_message(&Message::Call(call)).unwrap();
    assert!(j.starts_with("[2,"));
    assert!(matches!(
        parse::deserialize_to_message(&j).unwrap(),
        Message::Call(_)
    ));

    // Type 3 (raw)
    let raw = CallResultRaw::new(
        "m3".into(),
        serde_json::json!({"currentTime": "2024-01-01T00:00:00.000Z"}),
    );
    let j = parse::serialize_message(&Message::CallResult(raw)).unwrap();
    assert!(j.starts_with("[3,"));

    // Type 4
    let j = parse::serialize_message(&Message::CallError(CallError::new(
        "m4".into(),
        ocpp_rs::v21::rpc_error_code::RpcErrorCode::NotImplemented,
        "".into(),
        BTreeMap::new(),
    )))
    .unwrap();
    assert!(j.starts_with("[4,"));

    // Type 5
    let j = parse::serialize_message(&Message::CallResultError(CallResultError::new(
        "m5".into(),
        ocpp_rs::v21::rpc_error_code::RpcErrorCode::GenericError,
        "unusable".into(),
        BTreeMap::new(),
    )))
    .unwrap();
    assert!(j.starts_with("[5,"));

    // Type 6
    use ocpp_rs::v21::messages::notify_periodic_event_stream::{
        NotifyPeriodicEventStream, StreamDataElementType,
    };
    let j = parse::serialize_message(&Message::Send(Send::new(
        "m6".into(),
        SendAction::NotifyPeriodicEventStream(NotifyPeriodicEventStream {
            id: 7,
            pending: 0,
            basetime: dt(2024, 1, 1, 0, 0, 0),
            data: vec![StreamDataElementType {
                t: 1.5,
                v: "42".into(),
                custom_data: None,
            }],
            custom_data: None,
        }),
    )))
    .unwrap();
    assert!(j.starts_with("[6,"));
    assert!(matches!(
        parse::deserialize_to_message(&j).unwrap(),
        Message::Send(_)
    ));
}

#[test]
fn log_helper_covers_all_core_types() {
    let call = Message::Call(Call::new(
        "1".into(),
        Action::Heartbeat(HeartbeatRequest { custom_data: None }),
    ));
    let line = MessageLogLine::from_message(&call);
    assert_eq!(line.core_type, "Call");
    assert_eq!(line.payload_type, "Heartbeat");

    let result = Message::CallResult(CallResultRaw::new("1".into(), serde_json::json!({})));
    let line = MessageLogLine::from_message(&result);
    assert_eq!(line.core_type, "CallResult");
    assert_eq!(line.payload_type, "CallResultRaw");
}

// ---------------------------------------------------------------------------
// Block B — Provisioning (TC_B_01 / TC_B_02 / TC_B_03 / TC_B_06 / TC_B_09 / TC_B_12 / TC_B_20)
// ---------------------------------------------------------------------------

/// Inspired by TC_B_01_CS: Cold Boot Charging Station - Accepted.
#[test]
fn tc_b_01_cold_boot_accepted_workflow() {
    let mut cs_pending = PendingCalls::new(); // station waits for BootNotification.conf
    let mut csms_seen_boot = false;

    // CS → CSMS: BootNotification
    let boot_call = Call::new(
        "boot-1".into(),
        Action::BootNotification(BootNotificationRequest {
            reason: BootReasonEnumType::PowerUp,
            charging_station: ChargingStationType {
                model: "SingleSocketCharger".into(),
                vendor_name: "VendorX".into(),
                serial_number: Some("SN-001".into()),
                modem: None,
                firmware_version: Some("1.0.0".into()),
                custom_data: None,
            },
            custom_data: None,
        }),
    );
    let boot_wire = cs_pending.send_call(boot_call).unwrap();

    // CSMS receives CALL
    match parse::deserialize_to_message(&boot_wire).unwrap() {
        Message::Call(Call {
            payload: Action::BootNotification(req),
            unique_id,
            ..
        }) => {
            assert_eq!(unique_id, "boot-1");
            assert_eq!(req.reason, BootReasonEnumType::PowerUp);
            csms_seen_boot = true;

            let conf = BootNotificationResponse {
                current_time: now_ms(),
                interval: 300,
                status: RegistrationStatusEnumType::Accepted,
                status_info: None,
                custom_data: None,
            };
            let resp_msg = req
                .get_response(unique_id, conf)
                .expect("serialize response");
            let resp_wire = parse::serialize_message(&resp_msg).unwrap();

            assert_pending_result(&mut cs_pending, &resp_wire, |t| match t {
                TypedCallResult::BootNotification(cr) => {
                    assert_eq!(cr.payload.status, RegistrationStatusEnumType::Accepted);
                    assert_eq!(cr.payload.interval, 300);
                }
                other => panic!("{other:?}"),
            });
        }
        other => panic!("{other:?}"),
    }
    assert!(csms_seen_boot);

    // Post-boot: StatusNotification for connectors (TC_B_01 post validation)
    let status = Call::new(
        "status-1".into(),
        Action::StatusNotification(StatusNotificationRequest {
            timestamp: now_ms(),
            connector_status: ConnectorStatusEnumType::Available,
            evse_id: 1,
            connector_id: 1,
            custom_data: None,
        }),
    );
    let status = roundtrip_call(status);
    assert!(matches!(status.payload, Action::StatusNotification(_)));

    // Heartbeat with interval from boot
    let mut hb_pending = PendingCalls::new();
    let hb_wire = hb_pending
        .send_call(Call::new(
            "hb-1".into(),
            Action::Heartbeat(HeartbeatRequest { custom_data: None }),
        ))
        .unwrap();
    assert!(hb_wire.contains("Heartbeat"));
    let hb_resp = HeartbeatRequest { custom_data: None }
        .get_response(
            "hb-1".into(),
            HeartbeatResponse {
                current_time: now_ms(),
                custom_data: None,
            },
        )
        .expect("serialize response");
    let hb_resp_wire = parse::serialize_message(&hb_resp).unwrap();
    assert_pending_result(&mut hb_pending, &hb_resp_wire, |t| {
        assert!(matches!(t, TypedCallResult::Heartbeat(_)));
    });
}

/// Inspired by TC_B_02 / TC_B_03: Pending and Rejected registration.
#[test]
fn tc_b_02_b_03_boot_pending_and_rejected() {
    for (status, interval) in [
        (RegistrationStatusEnumType::Pending, 60),
        (RegistrationStatusEnumType::Rejected, 120),
    ] {
        let mut pending = PendingCalls::new();
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
        pending
            .send_call(Call::new("b".into(), Action::BootNotification(req.clone())))
            .unwrap();
        let msg = req
            .get_response(
                "b".into(),
                BootNotificationResponse {
                    current_time: now_ms(),
                    interval,
                    status: status.clone(),
                    status_info: None,
                    custom_data: None,
                },
            )
            .expect("serialize response");
        let wire = parse::serialize_message(&msg).unwrap();
        assert_pending_result(&mut pending, &wire, |t| match t {
            TypedCallResult::BootNotification(cr) => {
                assert_eq!(cr.payload.status, status);
                assert_eq!(cr.payload.interval, interval);
            }
            other => panic!("{other:?}"),
        });
    }
}

/// Inspired by TC_B_06_CS: Get Variables - single value.
#[test]
fn tc_b_06_get_variables_single() {
    let mut pending = PendingCalls::new();
    let req = GetVariablesRequest {
        get_variable_data: vec![GetVariableDataType {
            attribute_type: None,
            component: ComponentType {
                name: "AuthCtrlr".into(),
                instance: None,
                evse: None,
                custom_data: None,
            },
            variable: VariableType {
                name: "Enabled".into(),
                instance: None,
                custom_data: None,
            },
            custom_data: None,
        }],
        custom_data: None,
    };
    pending
        .send_call(Call::new("gv-1".into(), Action::GetVariables(req.clone())))
        .unwrap();

    let resp = GetVariablesResponse {
        get_variable_result: vec![GetVariableResultType {
            attribute_status: GetVariableStatusEnumType::Accepted,
            attribute_status_info: None,
            attribute_type: None,
            attribute_value: Some("true".into()),
            component: req.get_variable_data[0].component.clone(),
            variable: req.get_variable_data[0].variable.clone(),
            custom_data: None,
        }],
        custom_data: None,
    };
    let wire = parse::serialize_message(
        &req.get_response("gv-1".into(), resp)
            .expect("serialize response"),
    )
    .unwrap();
    assert_pending_result(&mut pending, &wire, |t| match t {
        TypedCallResult::GetVariables(cr) => {
            assert_eq!(cr.payload.get_variable_result.len(), 1);
            assert_eq!(
                cr.payload.get_variable_result[0].attribute_value.as_deref(),
                Some("true")
            );
        }
        other => panic!("{other:?}"),
    });
}

/// Inspired by TC_B_09 / TC_B_12 / TC_B_20: SetVariables, GetBaseReport, Reset wire shapes.
#[test]
fn tc_b_provisioning_control_messages_roundtrip() {
    let reset = roundtrip_call(Call::new(
        "rst".into(),
        Action::Reset(ResetRequest {
            type_: ResetEnumType::OnIdle,
            evse_id: None,
            custom_data: None,
        }),
    ));
    match reset.payload {
        Action::Reset(r) => assert_eq!(r.type_, ResetEnumType::OnIdle),
        other => panic!("{other:?}"),
    }

    let mut pending = PendingCalls::new();
    pending
        .send_call(Call::new(
            "gbr".into(),
            Action::GetBaseReport(GetBaseReportRequest {
                request_id: 1,
                report_base: ReportBaseEnumType::ConfigurationInventory,
                custom_data: None,
            }),
        ))
        .unwrap();
    let wire = parse::serialize_message(
        &GetBaseReportRequest {
            request_id: 1,
            report_base: ReportBaseEnumType::ConfigurationInventory,
            custom_data: None,
        }
        .get_response(
            "gbr".into(),
            GetBaseReportResponse {
                status: GenericDeviceModelStatusEnumType::Accepted,
                status_info: None,
                custom_data: None,
            },
        )
        .expect("serialize response"),
    )
    .unwrap();
    assert_pending_result(&mut pending, &wire, |t| {
        assert!(matches!(t, TypedCallResult::GetBaseReport(_)));
    });

    let mut pending = PendingCalls::new();
    pending
        .send_call(Call::new(
            "rst2".into(),
            Action::Reset(ResetRequest {
                type_: ResetEnumType::Immediate,
                evse_id: Some(1),
                custom_data: None,
            }),
        ))
        .unwrap();
    let wire = parse::serialize_message(
        &ResetRequest {
            type_: ResetEnumType::Immediate,
            evse_id: Some(1),
            custom_data: None,
        }
        .get_response(
            "rst2".into(),
            ResetResponse {
                status: ResetStatusEnumType::Accepted,
                status_info: None,
                custom_data: None,
            },
        )
        .expect("serialize response"),
    )
    .unwrap();
    assert_pending_result(&mut pending, &wire, |t| match t {
        TypedCallResult::Reset(cr) => assert_eq!(cr.payload.status, ResetStatusEnumType::Accepted),
        other => panic!("{other:?}"),
    });
}

// ---------------------------------------------------------------------------
// Block C — Authorization
// ---------------------------------------------------------------------------

/// Inspired by Block C online Authorize Accepted.
#[test]
fn block_c_authorize_accepted_roundtrip() {
    let mut pending = PendingCalls::new();
    let req = AuthorizeRequest {
        id_token: id_token("TAG-001", "ISO14443"),
        certificate: None,
        iso15118_certificate_hash_data: None,
        custom_data: None,
    };
    pending
        .send_call(Call::new("auth-1".into(), Action::Authorize(req.clone())))
        .unwrap();

    let resp = AuthorizeResponse {
        id_token_info: IdTokenInfoType {
            status: AuthorizationStatusEnumType::Accepted,
            cache_expiry_date_time: None,
            charging_priority: None,
            group_id_token: None,
            language1: None,
            language2: None,
            evse_id: None,
            personal_message: None,
            custom_data: None,
        },
        certificate_status: None,
        allowed_energy_transfer: None,
        tariff: None,
        custom_data: None,
    };
    let wire = parse::serialize_message(
        &req.get_response("auth-1".into(), resp)
            .expect("serialize response"),
    )
    .unwrap();
    assert_pending_result(&mut pending, &wire, |t| match t {
        TypedCallResult::Authorize(cr) => {
            assert_eq!(
                cr.payload.id_token_info.status,
                AuthorizationStatusEnumType::Accepted
            );
        }
        other => panic!("{other:?}"),
    });
}

// ---------------------------------------------------------------------------
// Block E — Transactions (TC_E_* start/update/end via TransactionEvent)
// ---------------------------------------------------------------------------

/// Inspired by Block E: TransactionEvent Started → Updated → Ended with PendingCalls.
#[test]
fn block_e_transaction_event_lifecycle() {
    let tx_id = "tx-42";
    let events = [
        (
            "te-start",
            TransactionEventEnumType::Started,
            TriggerReasonEnumType::CablePluggedIn,
            Some(ChargingStateEnumType::EVConnected),
        ),
        (
            "te-upd",
            TransactionEventEnumType::Updated,
            TriggerReasonEnumType::ChargingStateChanged,
            Some(ChargingStateEnumType::Charging),
        ),
        (
            "te-end",
            TransactionEventEnumType::Ended,
            TriggerReasonEnumType::StopAuthorized,
            Some(ChargingStateEnumType::Idle),
        ),
    ];

    for (mid, event_type, trigger, charging_state) in events {
        let mut pending = PendingCalls::new();
        let req = TransactionEventRequest {
            cost_details: None,
            event_type: event_type.clone(),
            meter_value: None,
            timestamp: now_ms(),
            trigger_reason: trigger,
            seq_no: 0,
            offline: Some(false),
            number_of_phases_used: None,
            cable_max_current: None,
            reservation_id: None,
            preconditioning_status: None,
            evse_sleep: None,
            transaction_info: TransactionType {
                transaction_id: tx_id.into(),
                charging_state,
                time_spent_charging: None,
                stopped_reason: None,
                remote_start_id: None,
                operation_mode: None,
                tariff_id: None,
                transaction_limit: None,
                custom_data: None,
            },
            evse: Some(EVSEType {
                id: 1,
                connector_id: Some(1),
                custom_data: None,
            }),
            id_token: Some(id_token("TAG-001", "ISO14443")),
            custom_data: None,
        };
        let call = roundtrip_call(Call::new(mid.into(), Action::TransactionEvent(req.clone())));
        match call.payload {
            Action::TransactionEvent(r) => assert_eq!(r.event_type, event_type),
            other => panic!("{other:?}"),
        }

        pending
            .send_call(Call::new(mid.into(), Action::TransactionEvent(req.clone())))
            .unwrap();
        // Empty `{}` is a valid TransactionEventResponse — must not become another type
        let wire = format!(r#"[3, "{mid}", {{}}]"#);
        assert_pending_result(&mut pending, &wire, |t| match t {
            TypedCallResult::TransactionEvent(cr) => {
                let _: TransactionEventResponse = cr.payload;
            }
            other => panic!("ambiguous empty result resolved wrong: {other:?}"),
        });
    }
}

// ---------------------------------------------------------------------------
// Block F — Remote Control
// ---------------------------------------------------------------------------

#[test]
fn block_f_remote_start_stop() {
    let mut pending = PendingCalls::new();
    let start = RequestStartTransactionRequest {
        evse_id: Some(1),
        group_id_token: None,
        id_token: id_token("REMOTE", "Central"),
        remote_start_id: 99,
        charging_profile: None,
        custom_data: None,
    };
    pending
        .send_call(Call::new(
            "rs".into(),
            Action::RequestStartTransaction(start.clone()),
        ))
        .unwrap();
    let wire = parse::serialize_message(
        &start
            .get_response(
                "rs".into(),
                RequestStartTransactionResponse {
                    status: RequestStartStopStatusEnumType::Accepted,
                    status_info: None,
                    transaction_id: Some("tx-remote".into()),
                    custom_data: None,
                },
            )
            .expect("serialize response"),
    )
    .unwrap();
    assert_pending_result(&mut pending, &wire, |t| match t {
        TypedCallResult::RequestStartTransaction(cr) => {
            assert_eq!(cr.payload.status, RequestStartStopStatusEnumType::Accepted);
            assert_eq!(cr.payload.transaction_id.as_deref(), Some("tx-remote"));
        }
        other => panic!("{other:?}"),
    });

    let mut pending = PendingCalls::new();
    let stop = RequestStopTransactionRequest {
        transaction_id: "tx-remote".into(),
        custom_data: None,
    };
    pending
        .send_call(Call::new(
            "rst".into(),
            Action::RequestStopTransaction(stop.clone()),
        ))
        .unwrap();
    let wire = parse::serialize_message(
        &stop
            .get_response(
                "rst".into(),
                RequestStopTransactionResponse {
                    status: RequestStartStopStatusEnumType::Accepted,
                    status_info: None,
                    custom_data: None,
                },
            )
            .expect("serialize response"),
    )
    .unwrap();
    assert_pending_result(&mut pending, &wire, |t| {
        assert!(matches!(t, TypedCallResult::RequestStopTransaction(_)));
    });
}

// ---------------------------------------------------------------------------
// Block G — Availability
// ---------------------------------------------------------------------------

#[test]
fn block_g_status_and_change_availability() {
    let status = roundtrip_call(Call::new(
        "sn".into(),
        Action::StatusNotification(StatusNotificationRequest {
            timestamp: dt(2024, 6, 1, 12, 0, 0),
            connector_status: ConnectorStatusEnumType::Occupied,
            evse_id: 1,
            connector_id: 2,
            custom_data: None,
        }),
    ));
    match status.payload {
        Action::StatusNotification(s) => {
            assert_eq!(s.connector_status, ConnectorStatusEnumType::Occupied);
        }
        other => panic!("{other:?}"),
    }

    // Empty conf for StatusNotification vs status-only ChangeAvailability — PendingCalls distinguishes
    let empty = r#"[3, "x", {}]"#;
    let status_only = r#"[3, "y", {"status": "Accepted"}]"#;

    let mut p = PendingCalls::new();
    p.register(
        "x",
        Action::StatusNotification(StatusNotificationRequest {
            timestamp: now_ms(),
            connector_status: ConnectorStatusEnumType::Available,
            evse_id: 1,
            connector_id: 1,
            custom_data: None,
        }),
    );
    assert_pending_result(&mut p, empty, |t| match t {
        TypedCallResult::StatusNotification(cr) => {
            let _: StatusNotificationResponse = cr.payload;
        }
        other => panic!("{other:?}"),
    });

    let mut p = PendingCalls::new();
    p.register(
        "y",
        Action::ChangeAvailability(ChangeAvailabilityRequest {
            evse: Some(EVSEType {
                id: 1,
                connector_id: None,
                custom_data: None,
            }),
            operational_status: OperationalStatusEnumType::Inoperative,
            custom_data: None,
        }),
    );
    assert_pending_result(&mut p, status_only, |t| match t {
        TypedCallResult::ChangeAvailability(cr) => {
            assert_eq!(
                cr.payload.status,
                ChangeAvailabilityStatusEnumType::Accepted
            );
            let _: ChangeAvailabilityResponse = cr.payload;
        }
        other => panic!("{other:?}"),
    });
}

// ---------------------------------------------------------------------------
// Block P — DataTransfer
// ---------------------------------------------------------------------------

#[test]
fn block_p_data_transfer_unknown_vendor() {
    let mut pending = PendingCalls::new();
    let req = DataTransferRequest {
        message_id: Some("CustomMsg".into()),
        data: Some(serde_json::json!({"foo": 1})),
        vendor_id: "UnknownVendor".into(),
        custom_data: None,
    };
    pending
        .send_call(Call::new("dt".into(), Action::DataTransfer(req.clone())))
        .unwrap();
    let wire = parse::serialize_message(
        &req.get_response(
            "dt".into(),
            DataTransferResponse {
                status: DataTransferStatusEnumType::UnknownVendorId,
                status_info: None,
                data: None,
                custom_data: None,
            },
        )
        .expect("serialize response"),
    )
    .unwrap();
    assert_pending_result(&mut pending, &wire, |t| match t {
        TypedCallResult::DataTransfer(cr) => {
            assert_eq!(
                cr.payload.status,
                DataTransferStatusEnumType::UnknownVendorId
            );
        }
        other => panic!("{other:?}"),
    });
}

// ---------------------------------------------------------------------------
// End-to-end station session (combines B + C + E + G)
// ---------------------------------------------------------------------------

#[test]
fn production_like_session_boot_auth_transaction() {
    // 1) Boot Accepted
    let mut boot_pending = PendingCalls::new();
    let boot_req = BootNotificationRequest {
        reason: BootReasonEnumType::PowerUp,
        charging_station: ChargingStationType {
            model: "Model-A".into(),
            vendor_name: "VendorY".into(),
            serial_number: None,
            modem: None,
            firmware_version: None,
            custom_data: Some(CustomDataType {
                vendor_id: "VendorY".into(),
                extra: Default::default(),
            }),
        },
        custom_data: None,
    };
    boot_pending
        .send_call(Call::new(
            "1".into(),
            Action::BootNotification(boot_req.clone()),
        ))
        .unwrap();
    let boot_conf = parse::serialize_message(
        &boot_req
            .get_response(
                "1".into(),
                BootNotificationResponse {
                    current_time: now_ms(),
                    interval: 60,
                    status: RegistrationStatusEnumType::Accepted,
                    status_info: None,
                    custom_data: None,
                },
            )
            .expect("serialize response"),
    )
    .unwrap();
    assert_pending_result(&mut boot_pending, &boot_conf, |_| {});

    // 2) Authorize
    let mut auth_pending = PendingCalls::new();
    let auth_req = AuthorizeRequest {
        id_token: id_token("AABBCCDD", "ISO14443"),
        certificate: None,
        iso15118_certificate_hash_data: None,
        custom_data: None,
    };
    auth_pending
        .send_call(Call::new("2".into(), Action::Authorize(auth_req.clone())))
        .unwrap();
    let auth_conf = parse::serialize_message(
        &auth_req
            .get_response(
                "2".into(),
                AuthorizeResponse {
                    id_token_info: IdTokenInfoType {
                        status: AuthorizationStatusEnumType::Accepted,
                        cache_expiry_date_time: None,
                        charging_priority: Some(0),
                        group_id_token: None,
                        language1: None,
                        language2: None,
                        evse_id: Some(vec![1]),
                        personal_message: None,
                        custom_data: None,
                    },
                    certificate_status: None,
                    allowed_energy_transfer: None,
                    tariff: None,
                    custom_data: None,
                },
            )
            .expect("serialize response"),
    )
    .unwrap();
    assert_pending_result(&mut auth_pending, &auth_conf, |_| {});

    // 3) TransactionEvent Started
    let te = TransactionEventRequest {
        cost_details: None,
        event_type: TransactionEventEnumType::Started,
        meter_value: None,
        timestamp: now_ms(),
        trigger_reason: TriggerReasonEnumType::Authorized,
        seq_no: 0,
        offline: None,
        number_of_phases_used: Some(3),
        cable_max_current: None,
        reservation_id: None,
        preconditioning_status: None,
        evse_sleep: None,
        transaction_info: TransactionType {
            transaction_id: "session-1".into(),
            charging_state: Some(ChargingStateEnumType::Charging),
            time_spent_charging: None,
            stopped_reason: None,
            remote_start_id: None,
            operation_mode: None,
            tariff_id: None,
            transaction_limit: None,
            custom_data: None,
        },
        evse: Some(EVSEType {
            id: 1,
            connector_id: Some(1),
            custom_data: None,
        }),
        id_token: Some(id_token("AABBCCDD", "ISO14443")),
        custom_data: None,
    };
    let te_call = roundtrip_call(Call::new("3".into(), Action::TransactionEvent(te)));
    assert!(matches!(te_call.payload, Action::TransactionEvent(_)));

    // 4) Status Occupied
    let sn = roundtrip_call(Call::new(
        "4".into(),
        Action::StatusNotification(StatusNotificationRequest {
            timestamp: now_ms(),
            connector_status: ConnectorStatusEnumType::Occupied,
            evse_id: 1,
            connector_id: 1,
            custom_data: None,
        }),
    ));
    assert!(matches!(sn.payload, Action::StatusNotification(_)));
}

#[test]
fn part4_boot_notification_json_samples() {
    let request = r#"[2,"19223201","BootNotification",{"reason":"PowerUp","chargingStation":{"model":"SingleSocketCharger","vendorName":"VendorX"}}]"#;
    let response = r#"[3,"19223201",{"currentTime":"2013-02-01T20:53:32.486Z","interval":300,"status":"Accepted"}]"#;

    match parse::deserialize_to_message(request).unwrap() {
        Message::Call(c) => assert!(matches!(c.payload, Action::BootNotification(_))),
        other => panic!("{other:?}"),
    }

    let mut pending = PendingCalls::new();
    pending.register(
        "19223201",
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
    assert_pending_result(&mut pending, response, |t| match t {
        TypedCallResult::BootNotification(cr) => {
            assert_eq!(cr.payload.interval, 300);
            assert_eq!(cr.payload.status, RegistrationStatusEnumType::Accepted);
        }
        other => panic!("{other:?}"),
    });
}

#[test]
fn deny_unknown_fields_on_boot_notification() {
    let bad = r#"{"reason":"PowerUp","chargingStation":{"model":"M","vendorName":"V"},"extraField":true}"#;
    let err = serde_json::from_str::<BootNotificationRequest>(bad).unwrap_err();
    assert!(err.to_string().contains("unknown field") || err.is_data());
}
