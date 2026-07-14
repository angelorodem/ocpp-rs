//! Runnable demos for ocpp_rs 0.4 (OCPP 1.6 + 2.1).
//!
//! ```bash
//! cd example && cargo run
//! ```

use ocpp_rs::v16::{
    call::{Action as V16Action, Call as V16Call, Heartbeat as V16Heartbeat},
    call_error::CallError as V16CallError,
    call_result::{self as v16_call_result, CallResultRaw},
    data_types::IdTagInfo,
    enums::AuthorizationStatus,
    log_helper::MessageLogLine,
    parse::{self as v16_parse, Message as V16Message, TypedMessage as V16TypedMessage},
    pending::PendingCalls as V16Pending,
    response_trait::Response,
    rpc_error_code::RpcErrorCode as V16RpcErrorCode,
    typed_call_result::TypedCallResult as V16TypedCallResult,
};
use ocpp_rs::v21::{
    call::{Action as V21Action, Call as V21Call},
    call_error::CallError as V21CallError,
    messages::boot_notification::{BootNotificationRequest, BootReasonEnumType, ChargingStationType},
    messages::heartbeat::HeartbeatRequest,
    parse::{self as v21_parse, Message as V21Message, TypedMessage as V21TypedMessage},
    pending::PendingCalls as V21Pending,
    typed_call_result::TypedCallResult as V21TypedCallResult,
    version::{allows_action, allows_message_type, NegotiatedVersion},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== OCPP 1.6 ===");
    v16_pending_heartbeat()?;
    v16_status_notification_and_response()?;
    v16_call_error()?;
    v16_boot_notification()?;
    v16_build_callresult()?;

    println!("\n=== OCPP 2.1 ===");
    v21_pending_heartbeat()?;
    v21_boot_notification()?;
    v21_call_error_and_version_gate()?;

    Ok(())
}

fn v16_pending_heartbeat() -> Result<(), Box<dyn std::error::Error>> {
    let mut pending = V16Pending::new();
    let wire = pending.send_call(V16Call::new(
        "1".into(),
        V16Action::Heartbeat(V16Heartbeat {}),
    ))?;
    println!("outbound CALL: {wire}");

    let typed = pending.deserialize_typed(
        r#"[3, "1", {"currentTime": "2024-01-01T00:00:00.000Z"}]"#,
    )?;
    assert!(matches!(
        typed,
        V16TypedMessage::CallResult(V16TypedCallResult::Heartbeat(_))
    ));
    println!("correlated Heartbeat CALLRESULT: {typed:?}");
    Ok(())
}

fn v16_status_notification_and_response() -> Result<(), Box<dyn std::error::Error>> {
    let data = r#"[2, "253356461", "StatusNotification", {"connectorId":1,"errorCode":"NoError","status":"Available","timestamp":"2024-06-01T19:52:45.000Z"}]"#;
    let message = v16_parse::deserialize_to_message(data)?;
    println!("log: {:?}", MessageLogLine::from_message(&message));

    let V16Message::Call(call) = message else {
        return Err("expected CALL".into());
    };
    assert_eq!(call.action_kind(), "StatusNotification");

    let V16Action::StatusNotification(payload) = call.payload else {
        return Err("expected StatusNotification".into());
    };
    let response = payload.get_response(call.unique_id, v16_call_result::EmptyResponse {})?;
    println!("StatusNotification → empty CALLRESULT: {}", v16_parse::serialize_message(&response)?);
    Ok(())
}

fn v16_call_error() -> Result<(), Box<dyn std::error::Error>> {
    let data = r#"[4, "253356461", "GenericError", "Error in processing the request", {"detail1":"detail1"}]"#;
    let V16Message::CallError(err) = v16_parse::deserialize_to_message(data)? else {
        return Err("expected CALLERROR".into());
    };
    assert_eq!(err.error_code, V16RpcErrorCode::GenericError);
    println!("CALLERROR: {err:?}");

    // Building an error with the typed RPC enum:
    let built = V16CallError::formation_violation("id".into(), "bad frame".into());
    println!("built FormationViolation: {}", v16_parse::serialize_message(&V16Message::CallError(built))?);
    Ok(())
}

fn v16_boot_notification() -> Result<(), Box<dyn std::error::Error>> {
    let incoming = r#"[2, "19223201", "BootNotification", {"chargePointVendor":"VendorX","chargePointModel":"SingleSocketCharger"}]"#;
    let V16Message::Call(call) = v16_parse::deserialize_to_message(incoming)? else {
        return Err("expected CALL".into());
    };
    match call.payload {
        V16Action::BootNotification(boot) => {
            println!("BootNotification vendor={}", boot.charge_point_vendor);
        }
        other => return Err(format!("unexpected action: {other:?}").into()),
    }
    Ok(())
}

fn v16_build_callresult() -> Result<(), Box<dyn std::error::Error>> {
    let response = V16Message::CallResult(CallResultRaw::new(
        "1234".into(),
        serde_json::to_value(v16_call_result::StartTransaction {
            transaction_id: 0,
            id_tag_info: IdTagInfo {
                status: AuthorizationStatus::Accepted,
                expiry_date: None,
                parent_id_tag: None,
            },
        })?,
    ));
    println!("StartTransaction CALLRESULT: {}", v16_parse::serialize_message(&response)?);
    Ok(())
}

fn v21_pending_heartbeat() -> Result<(), Box<dyn std::error::Error>> {
    let mut pending = V21Pending::new();
    let call = V21Call::new(
        "1".into(),
        V21Action::Heartbeat(HeartbeatRequest { custom_data: None }),
    );
    assert_eq!(call.action_kind(), "Heartbeat");
    let wire = pending.send_call(call)?;
    println!("outbound CALL: {wire}");

    let typed = pending.deserialize_typed(
        r#"[3, "1", {"currentTime": "2024-01-01T00:00:00.000Z"}]"#,
    )?;
    assert!(matches!(
        typed,
        V21TypedMessage::CallResult(V21TypedCallResult::Heartbeat(_))
    ));
    println!("correlated Heartbeat CALLRESULT: {typed:?}");
    Ok(())
}

fn v21_boot_notification() -> Result<(), Box<dyn std::error::Error>> {
    let call = V21Call::new(
        "19223201".into(),
        V21Action::BootNotification(BootNotificationRequest {
            reason: BootReasonEnumType::PowerUp,
            charging_station: ChargingStationType {
                model: "SingleSocketCharger".into(),
                vendor_name: "VendorX".into(),
                serial_number: None,
                firmware_version: None,
                modem: None,
                custom_data: None,
            },
            custom_data: None,
        }),
    );
    let wire = v21_parse::serialize_message(&V21Message::Call(call))?;
    println!("BootNotification CALL: {wire}");

    let parsed = v21_parse::deserialize_to_message(&wire)?;
    assert!(matches!(
        parsed,
        V21Message::Call(ref c) if c.action_kind() == "BootNotification"
    ));
    Ok(())
}

fn v21_call_error_and_version_gate() -> Result<(), Box<dyn std::error::Error>> {
    let err = V21CallError::not_supported("42".into(), "action not implemented".into());
    println!(
        "CALLERROR: {}",
        v21_parse::serialize_message(&V21Message::CallError(err))?
    );

    let v201 = NegotiatedVersion::Ocpp201;
    assert!(!allows_message_type(v201, 6)); // SEND is 2.1-only
    assert!(!allows_action(v201, "SetDERControl"));
    assert!(allows_action(v201, "BootNotification"));
    println!(
        "ocpp2.0.1 gate: SEND={}, SetDERControl={}, BootNotification={}",
        allows_message_type(v201, 6),
        allows_action(v201, "SetDERControl"),
        allows_action(v201, "BootNotification"),
    );
    Ok(())
}
