use chrono::DateTime;
use ocpp_rs::v16::call::*;
use ocpp_rs::v16::call_result::CallResult;
use ocpp_rs::v16::call_result::EmptyResponse;
use ocpp_rs::v16::data_types::*;
use ocpp_rs::v16::enums::*;
use ocpp_rs::v16::parse;
use ocpp_rs::v16::parse::{Message, deserialize_to_message};
use ocpp_rs::v16::response_trait::Response;

#[test]
fn test_parse_boot_notification() {
    let data = "[2, \"19223201\", \"BootNotification\", { \"chargePointVendor\": \"VendorX\", \"chargePointModel\": \"SingleSocketCharger\" }]";
    let message = deserialize_to_message(data).unwrap();
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
fn test_parse_boot_notification_formatted() {
    let data = "[\n 2,\n \"000002202408090409141051003\",\n \"BootNotification\",\n {\n \"chargePointModel\": \"DC\",\n \"chargePointSerialNumber\": \"bbb\",\n \"chargePointVendor\": \"xxx\",\n \"firmwareVersion\": \"230906.0755\",\n \"iccid\": \"\",\n \"meterType\": \"DC\"\n }\n]\n";
    let message = deserialize_to_message(data).unwrap();
    println!("\nParsed: {:?}\n", message);

    let message_eq: Message = Message::Call(Call::new(
        "000002202408090409141051003".to_string(),
        Action::BootNotification(BootNotification {
            charge_point_vendor: "xxx".to_string(),
            charge_point_model: "DC".to_string(),
            charge_point_serial_number: Some("bbb".to_string()),
            firmware_version: Some("230906.0755".to_string()),
            iccid: Some("".to_string()),
            meter_type: Some("DC".to_string()),
            ..Default::default()
        }),
    ));

    assert_eq!(message, message_eq);
    match message {
        ocpp_rs::v16::parse::Message::Call(call) => {
            assert_eq!(call.unique_id, "000002202408090409141051003");
        }
        _ => panic!("Unexpected message type"),
    }
}

#[test]
fn test_parse_heartbeat() {
    let data = "[2, \"19223201\", \"Heartbeat\", {}]";
    let message = deserialize_to_message(data).unwrap();
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
    let message = deserialize_to_message(data).unwrap();
    println!("\nParsed: {:?}\n", message);

    let time = DateTimeWrapper::new(
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
    let message = deserialize_to_message(data).unwrap();
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
    let data = "[3, \"253356461\", {\"configurationKey\":[
        {\"key\":\"key1\", \"readonly\": false, \"value\": \"val1\" },
        {\"key\":\"key2\", \"readonly\": true, \"value\": \"val2\" }
    ]}]";
    let message = deserialize_to_message(data).unwrap();
    println!("\nParsed: {:?}\n", message);

    let auth = ocpp_rs::v16::call_result::ResultPayload::PossibleEmptyResponse(
        ocpp_rs::v16::call_result::EmptyResponses::GetConfiguration(
            ocpp_rs::v16::call_result::GetConfiguration {
                configuration_key: Some(vec![
                    KeyValue {
                        key: "key1".to_string(),
                        readonly: false,
                        value: Some("val1".to_string()),
                    },
                    KeyValue {
                        key: "key2".to_string(),
                        readonly: true,
                        value: Some("val2".to_string()),
                    },
                ]),
                unknown_key: None,
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
