use crate::v16::parse::Message;
use alloc::{
    format,
    string::{String, ToString},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// This module provides a structure for logging messages in a specific format.
///
/// core type is the type of the message, like `Call`, `CallResult`, or `CallError`.
/// payload type is the type of the payload, like `BootNotification`, `StartTransaction`, etc.
/// payload is the actual content of the message in JSON format.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct MessageLogLine {
    pub core_type: String,
    pub payload_type: String,
    pub payload: String,
}

impl MessageLogLine {
    /// Creates a `MessageLogLine` from a `Message`
    ///
    /// For `Call` messages: uses the action name as `payload_type`\
    /// For `CallResult` messages: uses the `ResultPayload` variant name as `payload_type`\
    /// For `CallError` messages: uses `error_code` and `error_description`
    #[must_use]
    pub fn from_message(message: &Message) -> Self {
        match message {
            Message::Call(call) => Self {
                core_type: "Call".to_string(),
                payload_type: call.action.clone(),
                payload: serde_json::to_string(&call.payload)
                    .unwrap_or_else(|_| json!({"error": "Serialization failed"}).to_string()),
            },
            Message::CallResult(call_result) => Self {
                core_type: "CallResult".to_string(),
                payload_type: call_result.payload.as_ref().to_string(),
                payload: serde_json::to_string(&call_result.payload)
                    .unwrap_or_else(|_| json!({"error": "Serialization failed"}).to_string()),
            },
            Message::CallError(call_error) => Self {
                core_type: "CallError".to_string(),
                payload_type: "Error".to_string(),
                payload: json!({
                    "error_code": call_error.error_code,
                    "error_description": call_error.error_description,
                    "error_details": call_error.error_details,
                })
                .to_string(),
            },
        }
    }
}
