use crate::errors::{CallTypeMismatch, Error, Result};

use super::call::Call;
use super::call_error::CallError;
use super::call_result::CallResult;

use alloc::string::String;
use serde::{Deserialize, Serialize};
use crate::alloc::string::ToString;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
/// Message is a container for `Call`, `CallResult`, and `CallError`   
/// The message type is determined by the `message_id` field 
/// These enum variants contains all OCPP messages
pub enum Message {
    /// Used to Send a commands (mostly used by the server)
    Call(Call),
    /// Used to send a response to a command (mostly used by the client)
    CallResult(CallResult),
    /// Used to send an error response to a command (mostly used by the client)
    CallError(CallError),
}

/// Parses a JSON string into a Message struct   
/// Message is a container for `Call`, `CallResult`, and `CallError`   
/// The message type is determined by the `message_id` field   
/// 
/// # Errors
/// Will return Err if the message type is not 2, 3, or 4   
/// and if the JSON deserialization fails   
pub fn deserialize_to_message(data: &str) -> Result<Message> {

    let call_type = get_call_type(data)?;

    match call_type {
        2 => {
            let call: Call = serde_json::from_str(data).map_err(Error::SerdeJson)?;
            Ok(Message::Call(call))
        }
        3 => {
            let call_result: CallResult = serde_json::from_str(data).map_err(Error::SerdeJson)?;
            Ok(Message::CallResult(call_result))
        }
        4 => {
            let call_error: CallError = serde_json::from_str(data).map_err(Error::SerdeJson)?;
            Ok(Message::CallError(call_error))
        }
        _ => Err(Error::InvalidMessageCallType),
    }
}

/// Check the initial characters of the message to determine the message type    
fn get_call_type(buf: &str) -> Result<u8> {
    for c in buf.chars().enumerate().skip(1) {
        if c.0 > 6 || c.1 == ',' {
            break;
        }
        if c.1.is_numeric() {
            let value = c.1.to_digit(10).ok_or(Error::InvalidMessageCallTypeParsing)?;
            if !(2..=4).contains(&value) {
                break;
            }
            #[allow(clippy::cast_possible_truncation)]
            return Ok(value as u8);
        }
    }
    Err(Error::InvalidMessageCallType)
}

/// Convert message into a string by serializing it into JSON    
/// # Errors    
///     
/// Will return Err if the JSON serialization fails    
/// or if the message type is not 2, 3, or 4    
pub fn serialize_message(message: &Message) -> Result<String> {
    match message {
        Message::Call(call) => {
            if call.message_id != 2 {
                return Err(Error::CallTypeMismatch(CallTypeMismatch{expected: 2, found: call.message_id}));
            }            
            let v = serde_json::to_value(call).map_err(Error::SerdeJson)?;
            Ok(v.to_string())
        }
        Message::CallResult(call_result) => {
            if call_result.message_id != 3 {
                return Err(Error::CallTypeMismatch(CallTypeMismatch{expected: 3, found: call_result.message_id}));
            }
            let v = serde_json::to_value(call_result).map_err(Error::SerdeJson)?;
            Ok(v.to_string())
        }
        Message::CallError(call_error) => {
            if call_error.message_id != 4 {
                return Err(Error::CallTypeMismatch(CallTypeMismatch{expected: 4, found: call_error.message_id}));
            }
            let v = serde_json::to_value(call_error).map_err(Error::SerdeJson)?;
            Ok(v.to_string())
        }
    }
}
