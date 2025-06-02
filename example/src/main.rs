use ocpp_rs::v16::{
    call::{self, Action, Call},
    call_error::CallError,
    call_result::{self, CallResult, EmptyResponses, ResultPayload, StatusResponses},
    parse::{deserialize_to_message, serialize_message, Message},
    response_trait::Response,
};
use ocpp_rs::v16::data_types::IdTagInfo;
use ocpp_rs::v16::parse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Get the data through a WebSocket connection
    // Example call_result message coming from a charge point (GetConfiguration call result)
    let data = "[3, \"253356461\", {\"configurationKey\": [{\"key\": \"key1\", \"readonly\": true}, {\"key\": \"key2\", \"readonly\": false}]}]";

    // Parse the message and handle it
    println!("Parsing and handling message CallResult: {}", data);
    parse_and_handle(data);

    // Example call message coming from a server (StatusNotification call)
    let data = "[2, \"253356461\", \"StatusNotification\", {\"connectorId\":1,\"errorCode\":\"NoError\",\"status\":\"Available\",\"timestamp\":\"2024-06-01T19:52:45Z\"}]";

    // Parse the message and handle it
    println!("Parsing and handling message Call: {}", data);
    parse_and_handle(data);

    // Example call_error message coming from a charge point (RemoteStartTransaction call error)
    let data = "[4, \"253356461\", \"GenericError\", \"Error in processing the request\", {\"detail1\":\"detail1\",\"detail2\":\"detail2\"}]";

    // Parse the message and handle it
    println!("Parsing and handling message CallError: {}", data);
    parse_and_handle(data);

    // Example of a server starting a remote transaction on a charge point
    println!("Sending new message to server!");
    send_new_message();

    // From the README.md example
    readme_example_receive()?;
    readme_example_send()?;

    Ok(())
}

fn readme_example_receive() -> Result<(), Box<dyn std::error::Error>> {
    let incoming_text = "[2, \"19223201\", \"BootNotification\", { \"chargePointVendor\": \"VendorX\", \"chargePointModel\": \"SingleSocketCharger\" }]";
    let incoming_message = parse::deserialize_to_message(incoming_text);
    if let Ok(Message::Call(call)) = incoming_message {
        match call.payload {
            Action::BootNotification(_boot_notification) => {
                // Do something with boot_notification
            }
            _ => {
                // Handle other actions
            }
        }
    }
    Ok(())
}

fn readme_example_send() -> Result<(), Box<dyn std::error::Error>> {
    let response = Message::CallResult(CallResult::new(
        "1234".to_string(),
        ResultPayload::StartTransaction(call_result::StartTransaction {
            transaction_id: 0,
            id_tag_info: IdTagInfo {
                status: ocpp_rs::v16::enums::ParsedGenericStatus::Accepted,
                expiry_date: None,
                parent_id_tag: None,
            },
        }),
    ));
    
    let json = parse::serialize_message(&response)?;
    println!("Sending to client: {}", json);
    Ok(())
}

fn send_new_message() {
    // Example new call to start a transaction on a charge point sent from a server
    let call = Call::new(
        "253356461".to_string(),
        Action::RemoteStartTransaction(call::RemoteStartTransaction {
            id_tag: "123456789".to_string(),
            connector_id: Some(1),
            charging_profile: None,
        }),
    );

    // Package it in a message
    let message = Message::Call(call);

    // Send the message to the server
    send_to_websocket(message);
}

fn parse_and_handle(data: &str) {
    // 2. Parse the message
    println!("Parsing message: {}", data);
    let message = deserialize_to_message(data).unwrap();

    println!("Message type: {:?}", message.as_ref());

    // 3. Match the message payload to the appropriate call type
    // if you do not intend to handle all call types, you can use an if-let
    match message {
        Message::Call(call) => {
            receive_call(call);
        }
        Message::CallResult(call_result) => {
            receive_call_result(call_result);
        }
        Message::CallError(call_error) => {
            receive_call_error(call_error);
        }
    }
}

// Simulates a charger device receiving a call message
// from a server
fn receive_call(data: Call) {
    println!("Received Call type: {:?}", data.payload.as_ref());
    match data.payload {
        Action::StatusNotification(payload) => {
            // Handle the payload
            println!("StatusNotification payload: {:?}", payload);

            // 4. Respond to the server if handling a call message
            // This is a dummy response, the response should be sent through the WebSocket connection
            let response_message =
                payload.get_response(data.unique_id, call_result::EmptyResponse {});
            send_to_websocket(response_message);
        }
        Action::Authorize(payload) => {
            // Handle the payload
            println!("Authorize payload: {:?}", payload);
        }
        _ => {}
    }
}

// Simulates a server receiving a call result message
// for a request it sent
fn receive_call_result(data: CallResult) {
    match data.payload {
        ResultPayload::BootNotification(payload) => {
            // Handle the payload
            println!("BootNotification payload: {:?}", payload);
        }
        ResultPayload::PossibleEmptyResponse(payload) => {
            // This payload needs extra attention, as responses can be empty
            // When empty the payload is ambiguous, fitting multiple models
            // This happens because CallResult has no type information
            // When ambiguous, the smallest model (`EmptyResponse`) is chosen.
            // Always use the `unique_id` to identify the response to a sent request
            handle_empty_responses(payload);
        }
        ResultPayload::PossibleStatusResponse(payload) => {
            // This payload needs extra attention, as responses can have only a status field
            // When so, the payload is ambiguous, fitting multiple models
            // This happens because CallResult has no type information
            // When ambiguous, the smallest model (`StatusResponse`) is chosen.
            // Always use the `unique_id` to identify the response to a sent request
            handle_status_responses(payload);
        }
        _ => {}
    }
}

// Handle all payloads that might contain only a status field
fn handle_empty_responses(data: EmptyResponses) {
    // Handle the payload
    match data {
        EmptyResponses::EmptyResponse(payload) => {
            println!("EmptyResponse: {:?}", payload);
        }
        EmptyResponses::GetDiagnostics(payload) => {
            println!("GetDiagnostics: {:?}", payload);
        }
        _ => {}
    }
}

// Handle all payloads that might contain only a status field
fn handle_status_responses(data: StatusResponses) {
    // Handle the payload
    match data {
        StatusResponses::StatusResponse(payload) => {
            println!("StatusResponse: {:?}", payload);
        }
        StatusResponses::GetCompositeSchedule(payload) => {
            println!("GetCompositeSchedule: {:?}", payload);
        }
        _ => {}
    }
}

fn receive_call_error(data: CallError) {
    println!("CallError: {:?}", data);
}

fn send_to_websocket(data: Message) {
    // Send the data through a WebSocket connection
    let json = serialize_message(&data).unwrap();
    println!("Sending to server: {}", json);
}
