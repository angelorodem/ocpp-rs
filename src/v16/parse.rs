use crate::errors::{CallTypeMismatch, Error, Result};

use super::call::Call;
use super::call_error::CallError;
use super::call_result::CallResult;

use serde::{Deserialize, Serialize};


#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Message {
    Call(Call),
    CallResult(CallResult),
    CallError(CallError),
}

/// Parses a string into a Message
/// Message is a container for `Call`, `CallResult`, and `CallError`
/// The message type is determined by the `message_id` field
/// 
/// # Errors
/// Will return Err if the message type is not 2, 3, or 4
/// and if the JSON deserialization fails
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
        _ => Err(Error::InvalidMessageCallType),
    }
}

/// Check the initial characters of the message to determine the message type
fn get_call_type(buf: &str) -> Result<u8> {
    for c in buf.chars().enumerate().skip(1) {
        if c.0 > 4 {
            break;
        }
        if c.1.is_numeric() {
            let value = c.1.to_digit(10).unwrap();
            if !(2..=4).contains(&value) {
                break;
            }
            #[allow(clippy::cast_possible_truncation)]
            return Ok(value as u8);
        }
    }
    Err(Error::InvalidMessageCallType)
}

/// Convert message into a string
/// # Errors
/// 
/// Will return Err if the JSON serialization fails
/// or if the message type is not 2, 3, or 4
pub fn from_message(message: &Message) -> Result<String> {
    match message {
        Message::Call(call) => {
            if call.message_id != 2 {
                return Err(Error::CallTypeMismatch(CallTypeMismatch{expected: 2, found: call.message_id}));
            }            
            let v = serde_json::to_value(call)?;
            Ok(v.to_string())
        }
        Message::CallResult(call_result) => {
            if call_result.message_id != 3 {
                return Err(Error::CallTypeMismatch(CallTypeMismatch{expected: 3, found: call_result.message_id}));
            }
            let v = serde_json::to_value(call_result)?;
            Ok(v.to_string())
        }
        Message::CallError(call_error) => {
            if call_error.message_id != 4 {
                return Err(Error::CallTypeMismatch(CallTypeMismatch{expected: 4, found: call_error.message_id}));
            }
            let v = serde_json::to_value(call_error)?;
            Ok(v.to_string())
        }
    }
}
