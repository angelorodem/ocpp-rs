use ocpp_rs::v16::{
    call::{self, Action, Call},
    call_error::CallError,
    call_result::{self, CallResult, EmptyResponses, ResultPayload, StatusResponses},
    parse::{from_message, to_message, Message},
    response_trait::Response,
};

fn main() {
    // 1. Get the data through a WebSocket connection
    // Example call_result message coming from a charge point (GetConfiguration call result)
    let data = "[3, \"253356461\", {\"configurationKey\": [\"key1\", \"key2\"]}]";

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

    // Exapample of a server starting a remote transaction on a charge point
    println!("Sending new message to server!");
    send_new_message();
}

fn send_new_message() {
    // Example new call to start a transaction on a charge point sent from a server
    let call = Call::new(
        Some("253356461".to_string()), // You can omit for the lib to generate a unique id for you
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
    let message = to_message(data).unwrap();

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
    let json = from_message(&data).unwrap();
    println!("Sending to server: {}", json);
}
