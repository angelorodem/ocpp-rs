use ocpp_rs::v16::{
    call::{self, Action, Call},
    call_error::CallError,
    call_result::{self, CallResultRaw},
    log_helper::MessageLogLine,
    parse::{self, deserialize_to_message, serialize_message, Message},
    pending::PendingCalls,
    response_trait::Response,
    typed_call_result::TypedCallResult,
};
use ocpp_rs::v16::data_types::IdTagInfo;
use ocpp_rs::v16::enums::AuthorizationStatus;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // CallResult with PendingCalls correlation (no untagged guessing)
    let mut pending = PendingCalls::new();
    pending.register(
        "253356461",
        Action::GetConfiguration(call::GetConfiguration {
            key: None,
        }),
    );
    let data = "[3, \"253356461\", {\"configurationKey\": [{\"key\": \"key1\", \"readonly\": true}, {\"key\": \"key2\", \"readonly\": false}]}]";
    println!("Parsing CallResult with pending: {data}");
    match pending.deserialize_typed(data)? {
        parse::TypedMessage::CallResult(TypedCallResult::GetConfiguration(cr)) => {
            println!("GetConfiguration response: {:?}", cr.payload);
        }
        other => println!("Unexpected typed message: {other:?}"),
    }

    let data = "[2, \"253356461\", \"StatusNotification\", {\"connectorId\":1,\"errorCode\":\"NoError\",\"status\":\"Available\",\"timestamp\":\"2024-06-01T19:52:45.000Z\"}]";
    println!("Parsing and handling message Call: {data}");
    parse_and_handle(data);

    let data = "[4, \"253356461\", \"GenericError\", \"Error in processing the request\", {\"detail1\":\"detail1\",\"detail2\":\"detail2\"}]";
    println!("Parsing and handling message CallError: {data}");
    parse_and_handle(data);

    println!("Sending new message to server!");
    send_new_message();

    readme_example_receive()?;
    readme_example_send()?;

    Ok(())
}

fn readme_example_receive() -> Result<(), Box<dyn std::error::Error>> {
    let incoming_text = "[2, \"19223201\", \"BootNotification\", { \"chargePointVendor\": \"VendorX\", \"chargePointModel\": \"SingleSocketCharger\" }]";
    let incoming_message = parse::deserialize_to_message(incoming_text);
    if let Ok(Message::Call(call)) = incoming_message {
        match call.payload {
            Action::BootNotification(_boot_notification) => {}
            _ => {}
        }
    }
    Ok(())
}

fn readme_example_send() -> Result<(), Box<dyn std::error::Error>> {
    let response = Message::CallResult(CallResultRaw::new(
        "1234".to_string(),
        serde_json::to_value(call_result::StartTransaction {
            transaction_id: 0,
            id_tag_info: IdTagInfo {
                status: AuthorizationStatus::Accepted,
                expiry_date: None,
                parent_id_tag: None,
            },
        })?,
    ));

    let json = parse::serialize_message(&response)?;
    println!("Sending to client: {json}");
    Ok(())
}

fn send_new_message() {
    let call = Call::new(
        "253356461".to_string(),
        Action::RemoteStartTransaction(call::RemoteStartTransaction {
            id_tag: "123456789".to_string(),
            connector_id: Some(1),
            charging_profile: None,
        }),
    );
    send_to_websocket(Message::Call(call));
}

fn parse_and_handle(data: &str) {
    println!("Parsing message: {data}");
    let message = deserialize_to_message(data).unwrap();
    println!("Message type: {:?}", message.as_ref());
    println!("Log message: {:?}", MessageLogLine::from_message(&message));

    match message {
        Message::Call(call) => receive_call(call),
        Message::CallResult(raw) => receive_call_result(raw),
        Message::CallError(call_error) => receive_call_error(call_error),
    }
}

fn receive_call(data: Call) {
    println!("Received Call type: {:?}", data.payload.as_ref());
    match data.payload {
        Action::StatusNotification(payload) => {
            println!("StatusNotification payload: {payload:?}");
            let response_message = payload
                .get_response(data.unique_id, call_result::EmptyResponse {})
                .expect("serialize response");
            send_to_websocket(response_message);
        }
        Action::Authorize(payload) => {
            println!("Authorize payload: {payload:?}");
        }
        _ => {}
    }
}

fn receive_call_result(data: CallResultRaw) {
    // Blind parse — correlate with PendingCalls in real code.
    println!(
        "CallResultRaw unique_id={} payload={}",
        data.unique_id, data.payload
    );
}

fn receive_call_error(data: CallError) {
    println!("CallError: {data:?}");
}

fn send_to_websocket(data: Message) {
    let json = serialize_message(&data).unwrap();
    println!("Sending to server: {json}");
}
