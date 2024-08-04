use chrono::DateTime;
use ocpp_rs::v16::call::*;
use ocpp_rs::v16::call_result::CallResult;
use ocpp_rs::v16::data_types::*;
use ocpp_rs::v16::enums::*;
use ocpp_rs::v16::parse::{to_message, Message};

#[test]
fn test_parse_boot_notification() {
    let data = "[2, \"19223201\", \"BootNotification\", { \"chargePointVendor\": \"VendorX\", \"chargePointModel\": \"SingleSocketCharger\" }]";
    let message = to_message(data).unwrap();
    println!("\nParsed: {:?}\n", message);

    let message_eq: Message = Message::Call(Call::new(
        Some("19223201".to_string()),
        "BootNotification".to_string(),
        Action::BootNotification(BootNotification {
            charge_point_vendor: "VendorX".to_string(),
            charge_point_model: "SingleSocketCharger".to_string(),
            ..Default::default()
        }),
    ));

    assert_eq!(message, message_eq);
    match message {
        ocpp_rs::v16::parse::Message::Call(call) => {
            assert_eq!(call.unique_id, "19223201");
            assert_eq!(call.action, "BootNotification");
        }
        _ => panic!("Unexpected message type"),
    }
}

#[test]
fn test_parse_heartbeat() {
    let data = "[2, \"19223201\", \"Heartbeat\", {}]";
    let message = to_message(data).unwrap();
    println!("\nParsed: {:?}\n", message);

    let message_eq: Message = Message::Call(Call::new(
        Some("19223201".to_string()),
        "Heartbeat".to_string(),
        Action::Heartbeat(Heartbeat {}),
    ));

    assert_eq!(message, message_eq);
    match message {
        ocpp_rs::v16::parse::Message::Call(call) => {
            assert_eq!(call.unique_id, "19223201");
            assert_eq!(call.action, "Heartbeat");
        }
        _ => panic!("Unexpected message type"),
    }
}

#[test]
fn test_status_notification() {
    let data = "[2, \"253356461\", \"StatusNotification\", {\"connectorId\":1,\"errorCode\":\"NoError\",\"status\":\"Available\",\"timestamp\":\"2024-06-01T19:52:45Z\"}]";
    let message = to_message(data).unwrap();
    println!("\nParsed: {:?}\n", message);

    let time = DateTimeWrapper(
        DateTime::parse_from_rfc3339("2024-06-01T19:52:45Z")
            .unwrap()
            .with_timezone(&chrono::Utc),
    );

    let action = StatusNotification {
        connector_id: 1,
        error_code: ChargePointErrorCode::NoError,
        status: ChargePointStatus::Available,
        timestamp: Some(time),
        ..Default::default()
    };

    assert_eq!(
        serde_json::to_value(action.timestamp.unwrap()).unwrap(),
        "2024-06-01T19:52:45Z"
    );

    let message_eq: Message = Message::Call(Call::new(
        Some("253356461".to_string()),
        "StatusNotification".to_string(),
        Action::StatusNotification(action),
    ));

    assert_eq!(message, message_eq);
    match message {
        ocpp_rs::v16::parse::Message::Call(call) => {
            assert_eq!(call.unique_id, "253356461");
            assert_eq!(call.action, "StatusNotification");
        }
        _ => panic!("Unexpected message type"),
    }
}

#[test]
fn test_authorization_call_result() {
    let data = "[3, \"253356461\", {\"idTagInfo\":{\"status\":\"Accepted\"}}]";
    let message = to_message(data).unwrap();
    println!("\nParsed: {:?}\n", message);

    let id_tag_info = IdTagInfo {
        expiry_date: None,
        parent_id_tag: None,
        status: GenericStatus::Accepted,
    };

    let auth = ocpp_rs::v16::call_result::ResultPayload::PossibleEmptyResponse(
        ocpp_rs::v16::call_result::EmptyResponses::PossibleIdTagInfoResponse(
            ocpp_rs::v16::call_result::IdTagInfoResponses::Authorize(
                ocpp_rs::v16::call_result::Authorize { id_tag_info },
            ),
        ),
    );

    let message_eq: Message = Message::CallResult(CallResult::new("253356461".to_string(), auth));

    assert_eq!(message, message_eq);
    match message {
        ocpp_rs::v16::parse::Message::CallResult(call) => {
            assert_eq!(call.unique_id, "253356461");
        }
        _ => panic!("Unexpected message type"),
    }
}
