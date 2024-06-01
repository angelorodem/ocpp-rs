use super::call::Call;
use super::call_error::CallError;
use super::call_result::CallResult;

use serde::{de::Error, Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Message {
    Call(Call),
    CallResult(CallResult),
    CallError(CallError),
}

pub fn to_message(data: &str) -> Result<Message> {

    let v: Vec<serde_json::Value> = serde_json::from_str(data)?;

    #[cfg(debug_assertions)]
    println!("Input payload: {:?}\n", v[3]);

    match v[0].as_i64() {
        Some(2) => {
            let call: Call = serde_json::from_str(data)?;
            Ok(Message::Call(call))
        }
        Some(3) => {
            let call_result: CallResult = serde_json::from_str(data)?;
            Ok(Message::CallResult(call_result))
        }
        Some(4) => {
            let call_error: CallError = serde_json::from_str(data)?;
            Ok(Message::CallError(call_error))
        }
        _ => Err(serde_json::Error::custom("Invalid message type")),
    }
}

// From Message to Json string
pub fn from_message(message: &Message) -> Result<String> {
    match message {
        Message::Call(call) => {
            if call.message_id != 2 {
                return Err(serde_json::Error::custom("Invalid call message type, expected 2"));
            }            
            let v = serde_json::to_value(call)?;
            Ok(v.to_string())
        }
        Message::CallResult(call_result) => {
            if call_result.message_id != 3 {
                return Err(serde_json::Error::custom("Invalid callResult message type, expected 3"));
            }
            let v = serde_json::to_value(call_result)?;
            Ok(v.to_string())
        }
        Message::CallError(call_error) => {
            if call_error.message_id != 4 {
                return Err(serde_json::Error::custom("Invalid callError message type, expected 4"));
            }
            let v = serde_json::to_value(call_error)?;
            Ok(v.to_string())
        }
    }
}
