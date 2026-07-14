//! Structured logging helper for OCPP 1.6 messages.

use crate::v16::parse::Message;
use alloc::string::{String, ToString};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct MessageLogLine {
    pub core_type: String,
    pub payload_type: String,
    pub payload: String,
}

impl MessageLogLine {
    /// For CALLRESULT, `payload_type` is `"CallResultRaw"` until resolved via pending correlation.
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
                payload_type: "CallResultRaw".to_string(),
                payload: alloc::string::ToString::to_string(&call_result.payload),
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
