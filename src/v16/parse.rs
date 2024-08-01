use crate::errors::{Error, Result};

use super::call::{Call};
use super::call_error::CallError;
use super::call_result::CallResult;

use serde::{Deserialize, Serialize};


#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Message {
    Call(Call),
    CallResult(CallResult),
    CallError(CallError),
}

pub fn to_message(data: &str) -> Result<Message> {

    let call_type = get_call_type(data)?;

    match call_type {
        2 => {
            let call: Call = serde_json::from_str(data)?;
            Ok(Message::Call(call))
        }
        3 => {
            let call_result: CallResult = serde_json::from_str(data)?;
            Ok(Message::CallResult(call_result))
        }
        4 => {
            let call_error: CallError = serde_json::from_str(data)?;
            Ok(Message::CallError(call_error))
        }
        _ => Err(Error::custom("Invalid message type")),
    }
}

/// Check the initial characters of the message to determine the message type
fn get_call_type(buf: &str) -> Result<u8> {
    for c in buf.chars().enumerate() {
        if c.0 > 4 {
            break;
        }
        if c.1.is_numeric() {
            return Ok(c.1.to_digit(10).unwrap() as u8);
        }
    }
    Err(Error::custom("Invalid message type"))
}

// From Message to Json string
pub fn from_message(message: &Message) -> Result<String> {
    match message {
        Message::Call(call) => {
            if call.message_id != 2 {
                return Err(Error::custom("Invalid call message type, expected 2"));
            }            
            let v = serde_json::to_value(call)?;
            Ok(v.to_string())
        }
        Message::CallResult(call_result) => {
            if call_result.message_id != 3 {
                return Err(Error::custom("Invalid callResult message type, expected 3"));
            }
            let v = serde_json::to_value(call_result)?;
            Ok(v.to_string())
        }
        Message::CallError(call_error) => {
            if call_error.message_id != 4 {
                return Err(Error::custom("Invalid callError message type, expected 4"));
            }
            let v = serde_json::to_value(call_error)?;
            Ok(v.to_string())
        }
    }
}
