use chrono::{DateTime, Timelike};
use ocpp_rs::v16::call::*;
use ocpp_rs::v16::call_result::CallResult;
use ocpp_rs::v16::call_result::EmptyResponse;
use ocpp_rs::v16::data_types::*;
use ocpp_rs::v16::enums::*;
use ocpp_rs::v16::parse;
use ocpp_rs::v16::parse::{Message, deserialize_to_message};
use ocpp_rs::v16::response_trait::Response;

// Helper function to create datetime with millisecond precision for consistent testing
fn now_with_millis() -> DateTimeWrapper {
    let now = chrono::Utc::now();
    // Truncate to millisecond precision to match serialization format
    let truncated = now
        .with_nanosecond((now.nanosecond() / 1_000_000) * 1_000_000)
        .unwrap();
    DateTimeWrapper::new(truncated)
}

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

#[test]
fn test_empty_payloads_deserialization() {
    // Test empty Heartbeat
    let data = "[2, \"12345\", \"Heartbeat\", {}]";
    let message = deserialize_to_message(data).unwrap();
    assert!(matches!(message, Message::Call(_)));

    // Test empty ClearCache
    let data = "[2, \"12346\", \"ClearCache\", {}]";
    let message = deserialize_to_message(data).unwrap();
    assert!(matches!(message, Message::Call(_)));

    // Test empty GetLocalListVersion
    let data = "[2, \"12347\", \"GetLocalListVersion\", {}]";
    let message = deserialize_to_message(data).unwrap();
    assert!(matches!(message, Message::Call(_)));
}

#[test]
fn test_malformed_json_handling() {
    // Missing closing bracket
    let data = "[2, \"12345\", \"Heartbeat\", {";
    assert!(deserialize_to_message(data).is_err());

    // Invalid message type
    let data = "[5, \"12345\", \"Heartbeat\", {}]";
    assert!(deserialize_to_message(data).is_err());

    // Missing required fields
    let data = "[2, \"12345\", \"BootNotification\", {}]";
    assert!(deserialize_to_message(data).is_err());

    // Invalid action name
    let data = "[2, \"12345\", \"InvalidAction\", {}]";
    assert!(deserialize_to_message(data).is_err());
}

#[test]
fn test_datetime_edge_cases() {
    // Test with milliseconds
    let data = "[2, \"12345\", \"StatusNotification\", {\"connectorId\":1,\"errorCode\":\"NoError\",\"status\":\"Available\",\"timestamp\":\"2024-06-01T19:52:45.123Z\"}]";
    let message = deserialize_to_message(data).unwrap();
    assert!(matches!(message, Message::Call(_)));

    // Test without milliseconds
    let data = "[2, \"12346\", \"StatusNotification\", {\"connectorId\":1,\"errorCode\":\"NoError\",\"status\":\"Available\",\"timestamp\":\"2024-06-01T19:52:45Z\"}]";
    let message = deserialize_to_message(data).unwrap();
    assert!(matches!(message, Message::Call(_)));

    // Test future date
    let data = "[2, \"12347\", \"StatusNotification\", {\"connectorId\":1,\"errorCode\":\"NoError\",\"status\":\"Available\",\"timestamp\":\"2030-12-31T23:59:59Z\"}]";
    let message = deserialize_to_message(data).unwrap();
    assert!(matches!(message, Message::Call(_)));
}

#[test]
fn test_call_error_scenarios() {
    // Test CallError parsing
    let data = "[4, \"12345\", \"GenericError\", \"Generic error description\", {}]";
    let message = deserialize_to_message(data).unwrap();
    match message {
        Message::CallError(call_error) => {
            assert_eq!(call_error.unique_id, "12345");
            assert_eq!(call_error.error_code, "GenericError");
            assert_eq!(call_error.error_description, "Generic error description");
        }
        _ => panic!("Expected CallError"),
    }
}

#[test]
fn test_complex_boot_notification_with_all_fields() {
    let data = r#"[2, "boot_001", "BootNotification", {
        "chargePointVendor": "ACME Corp",
        "chargePointModel": "FastCharger Pro",
        "chargeBoxSerialNumber": "CB123456789",
        "chargePointSerialNumber": "CP987654321",
        "firmwareVersion": "v2.1.3",
        "iccid": "89014103211118510720",
        "imsi": "310260000000000",
        "meterSerialNumber": "MT555666777",
        "meterType": "AC_METER_PRO"
    }]"#;

    let message = deserialize_to_message(data).unwrap();
    match message {
        Message::Call(call) => {
            assert_eq!(call.unique_id, "boot_001");
            if let Action::BootNotification(boot) = call.payload {
                assert_eq!(boot.charge_point_vendor, "ACME Corp");
                assert_eq!(boot.charge_point_model, "FastCharger Pro");
                assert_eq!(
                    boot.charge_box_serial_number,
                    Some("CB123456789".to_string())
                );
                assert_eq!(boot.firmware_version, Some("v2.1.3".to_string()));
                assert_eq!(boot.iccid, Some("89014103211118510720".to_string()));
                assert_eq!(boot.meter_type, Some("AC_METER_PRO".to_string()));
            } else {
                panic!("Expected BootNotification");
            }
        }
        _ => panic!("Expected Call"),
    }
}

#[test]
fn test_meter_values_with_multiple_sampled_values() {
    let data = r#"[2, "meter_001", "MeterValues", {
        "connectorId": 1,
        "transactionId": 123,
        "meterValue": [{
            "timestamp": "2024-06-01T10:30:00Z",
            "sampledValue": [
                {
                    "value": "12.5",
                    "context": "Sample.Periodic",
                    "format": "Raw",
                    "measurand": "Energy.Active.Import.Register",
                    "phase": "L1",
                    "location": "Outlet",
                    "unit": "kWh"
                },
                {
                    "value": "230.5",
                    "measurand": "Voltage",
                    "unit": "V"
                }
            ]
        }]
    }]"#;

    let message = deserialize_to_message(data).unwrap();
    match message {
        Message::Call(call) => {
            if let Action::MeterValues(meter_values) = call.payload {
                assert_eq!(meter_values.connector_id, 1);
                assert_eq!(meter_values.transaction_id, Some(123));
                assert_eq!(meter_values.meter_value.len(), 1);

                let meter_value = &meter_values.meter_value[0];
                assert_eq!(meter_value.sampled_value.len(), 2);

                let first_sample = &meter_value.sampled_value[0];
                assert_eq!(first_sample.value, "12.5");
                assert_eq!(
                    first_sample.measurand,
                    Some(Measurand::EnergyActiveImportRegister)
                );
            } else {
                panic!("Expected MeterValues");
            }
        }
        _ => panic!("Expected Call"),
    }
}

#[test]
fn test_response_generation_for_all_call_types() {
    // Test BootNotification response
    let boot = BootNotification {
        charge_point_vendor: "Test".to_string(),
        charge_point_model: "Model".to_string(),
        ..Default::default()
    };

    let response_payload = ocpp_rs::v16::call_result::BootNotification {
        current_time: now_with_millis(),
        interval: 60,
        status: ParsedGenericStatus::Accepted,
    };

    let response = boot.get_response("test_id".to_string(), response_payload);
    assert!(matches!(response, parse::Message::CallResult(_)));

    // Test Heartbeat response
    let heartbeat = Heartbeat {};
    let heartbeat_response = ocpp_rs::v16::call_result::Heartbeat {
        current_time: now_with_millis(),
    };

    let response = heartbeat.get_response("test_id".to_string(), heartbeat_response);
    assert!(matches!(response, parse::Message::CallResult(_)));

    // Test Authorize response
    let authorize = Authorize {
        id_tag: "RFID123".to_string(),
    };

    let auth_response = ocpp_rs::v16::call_result::GenericIdTagInfo {
        id_tag_info: Some(IdTagInfo {
            status: ParsedGenericStatus::Accepted,
            expiry_date: None,
            parent_id_tag: None,
        }),
    };

    let response = authorize.get_response("test_id".to_string(), auth_response);
    assert!(matches!(response, parse::Message::CallResult(_)));
}

#[test]
fn test_optional_fields_handling() {
    // Test StatusNotification with minimal fields
    let data = "[2, \"status_001\", \"StatusNotification\", {\"connectorId\":1,\"errorCode\":\"NoError\",\"status\":\"Available\"}]";
    let message = deserialize_to_message(data).unwrap();

    match message {
        Message::Call(call) => {
            if let Action::StatusNotification(status) = call.payload {
                assert_eq!(status.connector_id, 1);
                assert_eq!(status.error_code, ChargePointErrorCode::NoError);
                assert_eq!(status.status, ChargePointStatus::Available);
                assert!(status.timestamp.is_none());
                assert!(status.info.is_none());
                assert!(status.vendor_id.is_none());
            } else {
                panic!("Expected StatusNotification");
            }
        }
        _ => panic!("Expected Call"),
    }
}

#[test]
fn test_start_stop_transaction_flow() {
    // Test StartTransaction
    let start_data = r#"[2, "start_001", "StartTransaction", {
        "connectorId": 1,
        "idTag": "RFID123456",
        "meterStart": 1500,
        "timestamp": "2024-06-01T10:00:00Z",
        "reservationId": 42
    }]"#;

    let start_message = deserialize_to_message(start_data).unwrap();
    match start_message {
        Message::Call(call) => {
            if let Action::StartTransaction(start_tx) = call.payload {
                assert_eq!(start_tx.connector_id, 1);
                assert_eq!(start_tx.id_tag, "RFID123456");
                assert_eq!(start_tx.meter_start, 1500);
                assert_eq!(start_tx.reservation_id, Some(42));

                // Test response generation
                let response_payload = ocpp_rs::v16::call_result::StartTransaction {
                    transaction_id: 789,
                    id_tag_info: IdTagInfo {
                        status: ParsedGenericStatus::Accepted,
                        expiry_date: None,
                        parent_id_tag: None,
                    },
                };

                let response = start_tx.get_response(call.unique_id, response_payload);
                assert!(matches!(response, parse::Message::CallResult(_)));
            } else {
                panic!("Expected StartTransaction");
            }
        }
        _ => panic!("Expected Call"),
    }

    // Test StopTransaction
    let stop_data = r#"[2, "stop_001", "StopTransaction", {
        "meterStop": 2500,
        "timestamp": "2024-06-01T11:00:00Z",
        "transactionId": 789,
        "reason": "Local",
        "idTag": "RFID123456"
    }]"#;

    let stop_message = deserialize_to_message(stop_data).unwrap();
    match stop_message {
        Message::Call(call) => {
            if let Action::StopTransaction(stop_tx) = call.payload {
                assert_eq!(stop_tx.meter_stop, 2500);
                assert_eq!(stop_tx.transaction_id, 789);
                assert_eq!(stop_tx.reason, Some(Reason::Local));
                assert_eq!(stop_tx.id_tag, Some("RFID123456".to_string()));
            } else {
                panic!("Expected StopTransaction");
            }
        }
        _ => panic!("Expected Call"),
    }
}

#[test]
fn test_configuration_management() {
    // Test GetConfiguration call
    let get_config_data = r#"[2, "config_001", "GetConfiguration", {
        "key": ["HeartbeatInterval", "MeterValueSampleInterval"]
    }]"#;

    let message = deserialize_to_message(get_config_data).unwrap();
    match message {
        Message::Call(call) => {
            if let Action::GetConfiguration(get_config) = call.payload {
                assert!(get_config.key.is_some());
                let keys = get_config.key.unwrap();
                assert_eq!(keys.len(), 2);
                assert!(keys.contains(&"HeartbeatInterval".to_string()));
            } else {
                panic!("Expected GetConfiguration");
            }
        }
        _ => panic!("Expected Call"),
    }

    // Test ChangeConfiguration call
    let change_config_data = r#"[2, "config_002", "ChangeConfiguration", {
        "key": "HeartbeatInterval",
        "value": "30"
    }]"#;

    let message = deserialize_to_message(change_config_data).unwrap();
    match message {
        Message::Call(call) => {
            if let Action::ChangeConfiguration(change_config) = call.payload {
                assert_eq!(change_config.key, "HeartbeatInterval");
                assert_eq!(change_config.value, "30");
            } else {
                panic!("Expected ChangeConfiguration");
            }
        }
        _ => panic!("Expected Call"),
    }
}

#[test]
fn test_call_result_ambiguous_deserialization() {
    // Test empty response that could be interpreted as EmptyResponse
    let empty_response_data = "[3, \"test_001\", {}]";
    let message = deserialize_to_message(empty_response_data).unwrap();

    match message {
        Message::CallResult(call_result) => {
            assert_eq!(call_result.unique_id, "test_001");
            // Should deserialize as EmptyResponse in the PossibleEmptyResponse enum
            if let ocpp_rs::v16::call_result::ResultPayload::PossibleEmptyResponse(empty_resp) =
                call_result.payload
            {
                assert!(empty_resp.is_empty());
            } else {
                panic!("Expected PossibleEmptyResponse");
            }
        }
        _ => panic!("Expected CallResult"),
    }
}

#[test]
fn test_data_transfer_scenarios() {
    // Test DataTransfer call with all fields
    let data_transfer_data = r#"[2, "dt_001", "DataTransfer", {
        "vendorId": "ACME",
        "messageId": "CustomMessage",
        "data": "{\"customField\": \"customValue\"}"
    }]"#;

    let message = deserialize_to_message(data_transfer_data).unwrap();
    match message {
        Message::Call(call) => {
            if let Action::DataTransfer(dt) = call.payload {
                assert_eq!(dt.vendor_id, "ACME");
                assert_eq!(dt.message_id, Some("CustomMessage".to_string()));
                assert!(dt.data.is_some());
            } else {
                panic!("Expected DataTransfer");
            }
        }
        _ => panic!("Expected Call"),
    }

    // Test DataTransfer with minimal fields
    let minimal_dt_data = r#"[2, "dt_002", "DataTransfer", {
        "vendorId": "ACME"
    }]"#;

    let message = deserialize_to_message(minimal_dt_data).unwrap();
    match message {
        Message::Call(call) => {
            if let Action::DataTransfer(dt) = call.payload {
                assert_eq!(dt.vendor_id, "ACME");
                assert!(dt.message_id.is_none());
                assert!(dt.data.is_none());
            } else {
                panic!("Expected DataTransfer");
            }
        }
        _ => panic!("Expected Call"),
    }
}

#[test]
fn test_reset_and_unlock_operations() {
    // Test Reset (Hard)
    let reset_data = r#"[2, "reset_001", "Reset", {
        "type": "Hard"
    }]"#;

    let message = deserialize_to_message(reset_data).unwrap();
    match message {
        Message::Call(call) => {
            if let Action::Reset(reset) = call.payload {
                assert_eq!(reset.reset_type, ResetType::Hard);
            } else {
                panic!("Expected Reset");
            }
        }
        _ => panic!("Expected Call"),
    }

    // Test UnlockConnector
    let unlock_data = r#"[2, "unlock_001", "UnlockConnector", {
        "connectorId": 1
    }]"#;

    let message = deserialize_to_message(unlock_data).unwrap();
    match message {
        Message::Call(call) => {
            if let Action::UnlockConnector(unlock) = call.payload {
                assert_eq!(unlock.connector_id, 1);
            } else {
                panic!("Expected UnlockConnector");
            }
        }
        _ => panic!("Expected Call"),
    }
}

#[test]
fn test_long_unique_ids() {
    // Test with very long unique ID (edge case for memory management)
    let long_id = "a".repeat(1000);
    let data = format!("[2, \"{}\", \"Heartbeat\", {{}}]", long_id);

    let message = deserialize_to_message(&data).unwrap();
    match message {
        Message::Call(call) => {
            assert_eq!(call.unique_id, long_id);
        }
        _ => panic!("Expected Call"),
    }
}

#[test]
fn test_large_meter_values_payload() {
    // Test with many sampled values to ensure performance doesn't degrade
    let mut sampled_values = Vec::new();
    for i in 0..100 {
        sampled_values.push(format!(
            "{{\"value\": \"{}.{}\", \"measurand\": \"Energy.Active.Import.Register\"}}",
            i,
            i % 10
        ));
    }

    let data = format!(
        "[2, \"large_meter\", \"MeterValues\", {{\"connectorId\": 1, \"meterValue\": [{{\"timestamp\": \"2024-06-01T10:00:00Z\", \"sampledValue\": [{}]}}]}}]",
        sampled_values.join(",")
    );

    let message = deserialize_to_message(&data).unwrap();
    match message {
        Message::Call(call) => {
            if let Action::MeterValues(meter_values) = call.payload {
                assert_eq!(meter_values.meter_value[0].sampled_value.len(), 100);
            } else {
                panic!("Expected MeterValues");
            }
        }
        _ => panic!("Expected Call"),
    }
}

#[test]
fn test_serialization_roundtrip_all_message_types() {
    // Test that all message types can be serialized and deserialized without loss

    // Test Call messages
    let boot_call = Message::Call(Call::new(
        "test_001".to_string(),
        Action::BootNotification(BootNotification {
            charge_point_vendor: "Test Vendor".to_string(),
            charge_point_model: "Test Model".to_string(),
            firmware_version: Some("1.0.0".to_string()),
            ..Default::default()
        }),
    ));

    let serialized = parse::serialize_message(&boot_call).unwrap();
    let deserialized = deserialize_to_message(&serialized).unwrap();
    assert_eq!(boot_call, deserialized);

    // Test CallResult messages
    let boot_result = Message::CallResult(CallResult::new(
        "test_001".to_string(),
        ocpp_rs::v16::call_result::ResultPayload::BootNotification(
            ocpp_rs::v16::call_result::BootNotification {
                current_time: now_with_millis(),
                interval: 300,
                status: ParsedGenericStatus::Accepted,
            },
        ),
    ));

    let serialized = parse::serialize_message(&boot_result).unwrap();
    let deserialized = deserialize_to_message(&serialized).unwrap();
    assert_eq!(boot_result, deserialized);

    // Test CallError messages
    let error_msg = Message::CallError(ocpp_rs::v16::call_error::CallError::new(
        "test_001".to_string(),
        "InternalError".to_string(),
        "Test error".to_string(),
        Default::default(),
    ));

    let serialized = parse::serialize_message(&error_msg).unwrap();
    let deserialized = deserialize_to_message(&serialized).unwrap();
    assert_eq!(error_msg, deserialized);
}

#[test]
fn test_datetime_timezone_handling() {
    // Test various timezone scenarios that might occur in production
    let test_cases = vec![
        "2024-01-01T00:00:00.000Z",
        "2024-12-31T23:59:59.999Z",
        "2024-06-15T12:30:45.123Z",
        "2024-02-29T06:00:00.000Z", // Leap year
        "2024-03-10T02:30:00.000Z", // DST transition
    ];

    for datetime_str in test_cases {
        let data = format!(
            "[2, \"dt_test\", \"StatusNotification\", {{\"connectorId\":1,\"errorCode\":\"NoError\",\"status\":\"Available\",\"timestamp\":\"{}\"}}]",
            datetime_str
        );

        let message = deserialize_to_message(&data).unwrap();
        let serialized = parse::serialize_message(&message).unwrap();
        let roundtrip = deserialize_to_message(&serialized).unwrap();
        assert_eq!(message, roundtrip);
    }
}

#[test]
fn test_malformed_datetime_handling() {
    // Test clearly invalid datetime formats
    let invalid_datetimes = vec![
        "invalid-date",
        "2024-13-01T00:00:00.000Z", // Invalid month
        "2024-01-32T00:00:00.000Z", // Invalid day
        "2024-01-01T25:00:00.000Z", // Invalid hour
        "2024-01-01T00:60:00.000Z", // Invalid minute
        "not-a-date-at-all",
        "",
        "null",
        "2024-01-01",          // Too short
        "2024-01-01T00:00:00", // Missing timezone
    ];

    for invalid_dt in invalid_datetimes {
        let data = format!(
            "[2, \"dt_test\", \"StatusNotification\", {{\"connectorId\":1,\"errorCode\":\"NoError\",\"status\":\"Available\",\"timestamp\":\"{}\"}}]",
            invalid_dt
        );

        let result = deserialize_to_message(&data);
        assert!(
            result.is_err(),
            "Expected failure for datetime: {}",
            invalid_dt
        );
    }
}

#[test]
fn test_unicode_and_special_characters() {
    let special_strings = vec![
        "Normal ASCII",
        "Café with accents",
        "中文字符",
        "🔌⚡️🚗", // Emojis
        "\"Quoted string\"",
        "String with\nnewlines\tand\ttabs",
        "String with \\backslashes\\",
        "Mixed: café ⚡️ 中文 \"quotes\"",
    ];

    for special_str in special_strings {
        let data = format!(
            "[2, \"unicode_test\", \"BootNotification\", {{\"chargePointVendor\":\"{}\",\"chargePointModel\":\"Test\"}}]",
            special_str
                .replace('"', "\\\"")
                .replace('\n', "\\n")
                .replace('\t', "\\t")
        );

        if let Ok(message) = deserialize_to_message(&data) {
            // If parsing succeeds, roundtrip should work
            let serialized = parse::serialize_message(&message).unwrap();
            let roundtrip = deserialize_to_message(&serialized).unwrap();
            assert_eq!(message, roundtrip);
        }
    }
}

#[test]
fn test_extreme_numeric_values() {
    let test_cases = vec![
        (i32::MIN, "minimum i32"),
        (i32::MAX, "maximum i32"),
        (0, "zero"),
        (-1, "negative one"),
        (1, "positive one"),
    ];

    for (value, description) in test_cases {
        let data = format!(
            "[2, \"numeric_test\", \"StartTransaction\", {{\"connectorId\":1,\"idTag\":\"TEST\",\"meterStart\":1000,\"timestamp\":\"2024-01-01T00:00:00Z\",\"reservationId\":{}}}]",
            value
        );

        let result = deserialize_to_message(&data);
        match value {
            v if v < 0 => {
                // Negative reservation IDs should be handled gracefully
                // This depends on your business logic - adjust as needed
            }
            _ => {
                assert!(
                    result.is_ok(),
                    "Failed to parse {} value: {}",
                    description,
                    value
                );
                if let Ok(message) = result {
                    let serialized = parse::serialize_message(&message).unwrap();
                    let roundtrip = deserialize_to_message(&serialized).unwrap();
                    assert_eq!(message, roundtrip);
                }
            }
        }
    }
}

#[test]
fn test_concurrent_message_processing() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    let messages = vec![
        "[2, \"1\", \"Heartbeat\", {}]",
        "[2, \"2\", \"BootNotification\", {\"chargePointVendor\":\"V1\",\"chargePointModel\":\"M1\"}]",
        "[2, \"3\", \"StatusNotification\", {\"connectorId\":1,\"errorCode\":\"NoError\",\"status\":\"Available\"}]",
        "[2, \"4\", \"Authorize\", {\"idTag\":\"TEST123\"}]",
        "[2, \"5\", \"StartTransaction\", {\"connectorId\":1,\"idTag\":\"TEST\",\"meterStart\":1000,\"timestamp\":\"2024-01-01T00:00:00Z\"}]",
    ];

    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for (i, msg) in messages.into_iter().enumerate() {
        let results_clone = Arc::clone(&results);
        let msg_owned = msg.to_string();

        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let parsed = deserialize_to_message(&msg_owned).unwrap();
                let serialized = parse::serialize_message(&parsed).unwrap();
                let roundtrip = deserialize_to_message(&serialized).unwrap();
                assert_eq!(parsed, roundtrip);
            }

            results_clone.lock().unwrap().push(i);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_results = results.lock().unwrap();
    assert_eq!(final_results.len(), 5);
}

#[test]
fn test_memory_pressure_large_payloads() {
    // Test with very large meter values to ensure no memory issues
    let mut sampled_values = Vec::new();
    for i in 0..1000 {
        sampled_values.push(format!(
            "{{\"value\":\"{}.{}\",\"measurand\":\"Energy.Active.Import.Register\",\"unit\":\"kWh\"}}",
            i, i % 10
        ));
    }

    let data = format!(
        "[2, \"memory_test\", \"MeterValues\", {{\"connectorId\":1,\"meterValue\":[{{\"timestamp\":\"2024-01-01T00:00:00Z\",\"sampledValue\":[{}]}}]}}]",
        sampled_values.join(",")
    );

    let start_time = std::time::Instant::now();
    let message = deserialize_to_message(&data).unwrap();
    let parse_time = start_time.elapsed();

    let start_time = std::time::Instant::now();
    let serialized = parse::serialize_message(&message).unwrap();
    let serialize_time = start_time.elapsed();

    let start_time = std::time::Instant::now();
    let roundtrip = deserialize_to_message(&serialized).unwrap();
    let roundtrip_time = start_time.elapsed();

    assert_eq!(message, roundtrip);

    // Performance assertions (adjust thresholds based on requirements)
    assert!(
        parse_time.as_millis() < 1000,
        "Parsing took too long: {:?}",
        parse_time
    );
    assert!(
        serialize_time.as_millis() < 1000,
        "Serialization took too long: {:?}",
        serialize_time
    );
    assert!(
        roundtrip_time.as_millis() < 1000,
        "Roundtrip took too long: {:?}",
        roundtrip_time
    );
}

#[test]
fn test_all_enum_variants_serialization() {
    // Test all error codes
    let error_codes = vec![
        ChargePointErrorCode::ConnectorLockFailure,
        ChargePointErrorCode::EVCommunicationError,
        ChargePointErrorCode::GroundFailure,
        ChargePointErrorCode::HighTemperature,
        ChargePointErrorCode::InternalError,
        ChargePointErrorCode::LocalListConflict,
        ChargePointErrorCode::NoError,
        ChargePointErrorCode::OtherError,
        ChargePointErrorCode::OverCurrentFailure,
        ChargePointErrorCode::OverVoltage,
        ChargePointErrorCode::PowerMeterFailure,
        ChargePointErrorCode::PowerSwitchFailure,
        ChargePointErrorCode::ReaderFailure,
        ChargePointErrorCode::ResetFailure,
        ChargePointErrorCode::UnderVoltage,
        ChargePointErrorCode::WeakSignal,
    ];

    for error_code in error_codes {
        let status_notification = StatusNotification {
            connector_id: 1,
            error_code,
            status: ChargePointStatus::Available,
            timestamp: Some(now_with_millis()),
            ..Default::default()
        };

        let call = Call::new(
            "enum_test".to_string(),
            Action::StatusNotification(status_notification),
        );
        let message = Message::Call(call);

        let serialized = parse::serialize_message(&message).unwrap();
        let deserialized = deserialize_to_message(&serialized).unwrap();
        assert_eq!(message, deserialized);
    }

    // Test all charge point statuses
    let statuses = vec![
        ChargePointStatus::Available,
        ChargePointStatus::Preparing,
        ChargePointStatus::Charging,
        ChargePointStatus::SuspendedEVSE,
        ChargePointStatus::SuspendedEV,
        ChargePointStatus::Finishing,
        ChargePointStatus::Reserved,
        ChargePointStatus::Unavailable,
        ChargePointStatus::Faulted,
    ];

    for status in statuses {
        let status_notification = StatusNotification {
            connector_id: 1,
            error_code: ChargePointErrorCode::NoError,
            status,
            timestamp: Some(now_with_millis()),
            ..Default::default()
        };

        let call = Call::new(
            "status_test".to_string(),
            Action::StatusNotification(status_notification),
        );
        let message = Message::Call(call);

        let serialized = parse::serialize_message(&message).unwrap();
        let deserialized = deserialize_to_message(&serialized).unwrap();
        assert_eq!(message, deserialized);
    }
}

#[test]
fn test_production_workflow_simulation() {
    // Simulate a complete charging session workflow
    let mut messages = Vec::new();

    // 1. Boot notification
    let boot = Message::Call(Call::new(
        "001".to_string(),
        Action::BootNotification(BootNotification {
            charge_point_vendor: "ACME Corp".to_string(),
            charge_point_model: "FastCharger v2".to_string(),
            firmware_version: Some("2.1.0".to_string()),
            ..Default::default()
        }),
    ));
    messages.push(boot);

    // 2. Status notification - Available
    let status_available = Message::Call(Call::new(
        "002".to_string(),
        Action::StatusNotification(StatusNotification {
            connector_id: 1,
            error_code: ChargePointErrorCode::NoError,
            status: ChargePointStatus::Available,
            timestamp: Some(now_with_millis()),
            ..Default::default()
        }),
    ));
    messages.push(status_available);

    // 3. Authorize
    let authorize = Message::Call(Call::new(
        "003".to_string(),
        Action::Authorize(Authorize {
            id_tag: "RFID123456789".to_string(),
        }),
    ));
    messages.push(authorize);

    // 4. Start transaction
    let start_tx = Message::Call(Call::new(
        "004".to_string(),
        Action::StartTransaction(StartTransaction {
            connector_id: 1,
            id_tag: "RFID123456789".to_string(),
            meter_start: 12500,
            timestamp: now_with_millis(),
            reservation_id: None,
        }),
    ));
    messages.push(start_tx);

    // 5. Status notification - Charging
    let status_charging = Message::Call(Call::new(
        "005".to_string(),
        Action::StatusNotification(StatusNotification {
            connector_id: 1,
            error_code: ChargePointErrorCode::NoError,
            status: ChargePointStatus::Charging,
            timestamp: Some(now_with_millis()),
            ..Default::default()
        }),
    ));
    messages.push(status_charging);

    // 6. Meter values during charging
    let meter_values = Message::Call(Call::new(
        "006".to_string(),
        Action::MeterValues(MeterValues {
            connector_id: 1,
            transaction_id: Some(12345),
            meter_value: vec![MeterValue {
                timestamp: now_with_millis(),
                sampled_value: vec![SampledValue {
                    value: "15.5".to_string(),
                    context: Some(ReadingContext::SamplePeriodic),
                    format: Some(ValueFormat::Raw),
                    measurand: Some(Measurand::EnergyActiveImportRegister),
                    unit: Some(UnitOfMeasure::KWh),
                    phase: None,
                    location: None,
                }],
            }],
        }),
    ));
    messages.push(meter_values);

    // 7. Stop transaction
    let stop_tx = Message::Call(Call::new(
        "007".to_string(),
        Action::StopTransaction(StopTransaction {
            meter_stop: 25000,
            timestamp: now_with_millis(),
            transaction_id: 12345,
            reason: Some(Reason::Local),
            id_tag: Some("RFID123456789".to_string()),
            transaction_data: None,
        }),
    ));
    messages.push(stop_tx);

    // 8. Status notification - Finishing
    let status_finishing = Message::Call(Call::new(
        "008".to_string(),
        Action::StatusNotification(StatusNotification {
            connector_id: 1,
            error_code: ChargePointErrorCode::NoError,
            status: ChargePointStatus::Finishing,
            timestamp: Some(now_with_millis()),
            ..Default::default()
        }),
    ));
    messages.push(status_finishing);

    // 9. Status notification - Available
    let status_available_final = Message::Call(Call::new(
        "009".to_string(),
        Action::StatusNotification(StatusNotification {
            connector_id: 1,
            error_code: ChargePointErrorCode::NoError,
            status: ChargePointStatus::Available,
            timestamp: Some(now_with_millis()),
            ..Default::default()
        }),
    ));
    messages.push(status_available_final);

    // Test that all messages in the workflow can be processed
    for message in messages {
        let serialized = parse::serialize_message(&message).unwrap();
        let deserialized = deserialize_to_message(&serialized).unwrap();
        assert_eq!(message, deserialized);
    }
}

#[test]
fn test_error_recovery_scenarios() {
    // Test various error scenarios that might occur in production

    // Incomplete JSON
    let incomplete_json = "[2, \"test\", \"Heartbeat\"";
    assert!(deserialize_to_message(incomplete_json).is_err());

    // Wrong message type
    let wrong_type = "[5, \"test\", \"Heartbeat\", {}]";
    assert!(deserialize_to_message(wrong_type).is_err());

    // Missing required fields
    let missing_fields = "[2, \"test\", \"BootNotification\", {}]";
    assert!(deserialize_to_message(missing_fields).is_err());

    // Invalid enum values
    let invalid_enum = "[2, \"test\", \"StatusNotification\", {\"connectorId\":1,\"errorCode\":\"InvalidErrorCode\",\"status\":\"Available\"}]";
    assert!(deserialize_to_message(invalid_enum).is_err());

    // Null values in required fields
    let null_required = "[2, \"test\", \"BootNotification\", {\"chargePointVendor\":null,\"chargePointModel\":\"Test\"}]";
    assert!(deserialize_to_message(null_required).is_err());
}

#[test]
fn test_message_id_consistency() {
    // Ensure message IDs are correct for each message type
    let call = Call::new("test".to_string(), Action::Heartbeat(Heartbeat {}));
    let call_msg = Message::Call(call);
    let serialized = parse::serialize_message(&call_msg).unwrap();
    assert!(serialized.starts_with("[2,"));

    let call_result = Message::CallResult(CallResult::new(
        "test".to_string(),
        ocpp_rs::v16::call_result::ResultPayload::PossibleEmptyResponse(
            ocpp_rs::v16::call_result::EmptyResponses::EmptyResponse(
                ocpp_rs::v16::call_result::EmptyResponse {},
            ),
        ),
    ));
    let serialized = parse::serialize_message(&call_result).unwrap();
    assert!(serialized.starts_with("[3,"));

    let call_error = Message::CallError(ocpp_rs::v16::call_error::CallError::new(
        "test".to_string(),
        "GenericError".to_string(),
        "Test error".to_string(),
        Default::default(),
    ));
    let serialized = parse::serialize_message(&call_error).unwrap();
    assert!(serialized.starts_with("[4,"));
}

#[test]
fn test_unique_id_edge_cases() {
    let long_id = "a".repeat(100);
    let edge_case_ids = vec![
        "",            // Empty string
        "0",           // Single character
        &long_id,      // Very long ID
        "123-456-789", // With hyphens
        "ID_WITH_UNDERSCORES",
        "id with spaces", // With spaces
        "🔌⚡️",           // Unicode
    ];

    for unique_id in edge_case_ids {
        let call = Call::new(unique_id.to_string(), Action::Heartbeat(Heartbeat {}));
        let message = Message::Call(call);

        if let Ok(serialized) = parse::serialize_message(&message) {
            if let Ok(deserialized) = deserialize_to_message(&serialized) {
                assert_eq!(message, deserialized);
            }
        }
    }
}

#[test]
fn test_response_trait_coverage() {
    // Test response generation for various call types

    // Test ClearCache response
    let clear_cache = ClearCache {};
    let response = clear_cache.get_response(
        "test_clear".to_string(),
        ocpp_rs::v16::call_result::GenericStatusResponse {
            status: ParsedGenericStatus::Accepted,
        },
    );
    assert!(matches!(response, parse::Message::CallResult(_)));

    // Test ChangeConfiguration response
    let change_config = ChangeConfiguration {
        key: "HeartbeatInterval".to_string(),
        value: "60".to_string(),
    };
    let response = change_config.get_response(
        "test_config".to_string(),
        ocpp_rs::v16::call_result::GenericStatusResponse {
            status: ParsedGenericStatus::Accepted,
        },
    );
    assert!(matches!(response, parse::Message::CallResult(_)));

    // Test GetLocalListVersion response
    let get_version = GetLocalListVersion {};
    let response = get_version.get_response(
        "test_version".to_string(),
        ocpp_rs::v16::call_result::GetLocalListVersion { list_version: 42 },
    );
    assert!(matches!(response, parse::Message::CallResult(_)));
}

#[test]
fn test_optional_field_combinations() {
    // Test various combinations of optional fields
    let boot_variants = vec![
        BootNotification {
            charge_point_vendor: "Vendor".to_string(),
            charge_point_model: "Model".to_string(),
            ..Default::default()
        },
        BootNotification {
            charge_point_vendor: "Vendor".to_string(),
            charge_point_model: "Model".to_string(),
            firmware_version: Some("1.0".to_string()),
            ..Default::default()
        },
        BootNotification {
            charge_point_vendor: "Vendor".to_string(),
            charge_point_model: "Model".to_string(),
            charge_box_serial_number: Some("CB123".to_string()),
            charge_point_serial_number: Some("CP456".to_string()),
            firmware_version: Some("1.0".to_string()),
            iccid: Some("89012345678901234567".to_string()),
            imsi: Some("310120123456789".to_string()),
            meter_serial_number: Some("MT789".to_string()),
            meter_type: Some("AC".to_string()),
        },
    ];

    for (i, boot) in boot_variants.into_iter().enumerate() {
        let call = Call::new(format!("boot_{}", i), Action::BootNotification(boot));
        let message = Message::Call(call);

        let serialized = parse::serialize_message(&message).unwrap();
        let deserialized = deserialize_to_message(&serialized).unwrap();
        assert_eq!(message, deserialized);
    }
}

#[test]
fn test_backwards_compatibility() {
    // Test parsing messages that might come from older implementations
    let legacy_messages = vec![
        // Minimal boot notification (older format)
        "[2, \"legacy_001\", \"BootNotification\", {\"chargePointVendor\":\"Old\",\"chargePointModel\":\"Legacy\"}]",
        // Status notification without timestamp (should still work)
        "[2, \"legacy_002\", \"StatusNotification\", {\"connectorId\":1,\"errorCode\":\"NoError\",\"status\":\"Available\"}]",
        // Authorize with basic structure
        "[2, \"legacy_003\", \"Authorize\", {\"idTag\":\"BASIC123\"}]",
    ];

    for legacy_msg in legacy_messages {
        let parsed = deserialize_to_message(legacy_msg).unwrap();
        let serialized = parse::serialize_message(&parsed).unwrap();
        let roundtrip = deserialize_to_message(&serialized).unwrap();
        assert_eq!(parsed, roundtrip);
    }
}
