use chrono::DateTime;
use ocpp_rs::v16::call::*;
use ocpp_rs::v16::call_result::CallResult;
use ocpp_rs::v16::call_result::EmptyResponse;
use ocpp_rs::v16::data_types::*;
use ocpp_rs::v16::enums::*;
use ocpp_rs::v16::parse;
use ocpp_rs::v16::parse::{to_message, Message};
use ocpp_rs::v16::response_trait::Response;

#[test]
fn test_parse_boot_notification() {
    let data = "[2, \"19223201\", \"BootNotification\", { \"chargePointVendor\": \"VendorX\", \"chargePointModel\": \"SingleSocketCharger\" }]";
    let message = to_message(data).unwrap();
    println!("\nParsed: {:?}\n", message);

    let message_eq: Message = Message::Call(Call::new(
        "19223201".to_string(),
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
        "19223201".to_string(),
        Action::Heartbeat(Heartbeat {}),
    ));

    assert_eq!(message, message_eq);
    match message {
        ocpp_rs::v16::parse::Message::Call(call) => {
            assert_eq!(call.unique_id, "19223201");
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
        "253356461".to_string(),
        Action::StatusNotification(action),
    ));

    assert_eq!(message, message_eq);
    match &message {
        ocpp_rs::v16::parse::Message::Call(call) => {
            assert_eq!(call.unique_id, "253356461");
        }
        _ => panic!("Unexpected message type"),
    }

    if let Message::Call(ca) = message {
        if let Action::StatusNotification(sn) = ca.payload {
            let response = sn.get_response(ca.unique_id, EmptyResponse {});
            assert_eq!(
                response,
                parse::Message::CallResult(CallResult::new(
                    "253356461".to_string(),
                    ocpp_rs::v16::call_result::ResultPayload::PossibleEmptyResponse(
                        ocpp_rs::v16::call_result::EmptyResponses::EmptyResponse(EmptyResponse {})
                    )
                ))
            );
        }
    } else {
        panic!("Unexpected message type");
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
        status: ParsedGenericStatus::Accepted,
    };

    let auth = ocpp_rs::v16::call_result::ResultPayload::PossibleEmptyResponse(
        ocpp_rs::v16::call_result::EmptyResponses::GenericIdTagInfoResponse(
            ocpp_rs::v16::call_result::GenericIdTagInfo {
                id_tag_info: Some(id_tag_info),
            },
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

#[test]
fn test_get_configuration_call_result() {
    let data = "[3, \"253356461\", {\"configurationKey\": [\"key1\", \"key2\"]}]";
    let message = to_message(data).unwrap();
    println!("\nParsed: {:?}\n", message);


    let auth = ocpp_rs::v16::call_result::ResultPayload::PossibleEmptyResponse(
        ocpp_rs::v16::call_result::EmptyResponses::GetConfiguration(
            ocpp_rs::v16::call_result::GetConfiguration { configuration_key: Some(vec!["key1".to_string(), "key2".to_string()]), unknown_key: None }
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
