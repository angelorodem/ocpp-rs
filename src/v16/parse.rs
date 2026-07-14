//! Message container and parse/serialize for OCPP 1.6 OCPP-J.

use crate::errors::{CallTypeMismatch, Error, Result};
use alloc::string::{String, ToString};
use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use super::call::Call;
use super::call_error::CallError;
use super::call_result::CallResultRaw;
use super::typed_call_result::TypedCallResult;

/// Blind parse result. CALLRESULT payloads stay untyped until resolved via [`crate::v16::pending`].
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, AsRefStr)]
#[serde(untagged)]
pub enum Message {
    Call(Call),
    CallResult(CallResultRaw),
    CallError(CallError),
}

/// Fully typed message after CALLRESULT resolution.
#[derive(Debug, PartialEq, Clone)]
pub enum TypedMessage {
    Call(Call),
    CallResult(TypedCallResult),
    CallError(CallError),
}

/// Parses a JSON string into a [`Message`].
///
/// Always enforces MessageId / UniqueId length ≤ 36.
/// With feature `schema_validate`, also enforces Part 3 / 1.6 schema bounds on CALL payloads.
///
/// # Errors
/// Message type not in 2..=4, JSON failure, or constraint violation.
pub fn deserialize_to_message(data: &str) -> Result<Message> {
    let call_type = get_call_type(data)?;

    match call_type {
        2 => {
            let call: Call = serde_json::from_str(data).map_err(Error::SerdeJson)?;
            crate::validate::check_message_id_len(&call.unique_id)?;
            #[cfg(feature = "schema_validate")]
            {
                let payload = serde_json::to_value(&call.payload).map_err(Error::SerdeJson)?;
                super::validate_gen::validate_action_payload(call.action_kind(), &payload)?;
            }
            Ok(Message::Call(call))
        }
        3 => {
            let call_result: CallResultRaw =
                serde_json::from_str(data).map_err(Error::SerdeJson)?;
            crate::validate::check_message_id_len(&call_result.unique_id)?;
            Ok(Message::CallResult(call_result))
        }
        4 => {
            let call_error: CallError = serde_json::from_str(data).map_err(Error::SerdeJson)?;
            crate::validate::check_message_id_len(&call_error.unique_id)?;
            Ok(Message::CallError(call_error))
        }
        _ => Err(Error::InvalidMessageCallType),
    }
}

fn get_call_type(buf: &str) -> Result<u8> {
    for c in buf.chars().enumerate().skip(1) {
        if c.0 > 6 || c.1 == ',' {
            break;
        }
        if c.1.is_numeric() {
            let value =
                c.1.to_digit(10)
                    .ok_or(Error::InvalidMessageCallTypeParsing)?;
            if !(2..=4).contains(&value) {
                break;
            }
            #[allow(clippy::cast_possible_truncation)]
            return Ok(value as u8);
        }
    }
    Err(Error::InvalidMessageCallType)
}

/// # Errors
/// Serialization failure or wrong stored message type id.
pub fn serialize_message(message: &Message) -> Result<String> {
    match message {
        Message::Call(call) => {
            if call.message_id != 2 {
                return Err(Error::CallTypeMismatch(CallTypeMismatch {
                    expected: 2,
                    found: call.message_id,
                }));
            }
            let v = serde_json::to_value(call).map_err(Error::SerdeJson)?;
            Ok(v.to_string())
        }
        Message::CallResult(call_result) => {
            if call_result.message_id != 3 {
                return Err(Error::CallTypeMismatch(CallTypeMismatch {
                    expected: 3,
                    found: call_result.message_id,
                }));
            }
            let v = serde_json::to_value(call_result).map_err(Error::SerdeJson)?;
            Ok(v.to_string())
        }
        Message::CallError(call_error) => {
            if call_error.message_id != 4 {
                return Err(Error::CallTypeMismatch(CallTypeMismatch {
                    expected: 4,
                    found: call_error.message_id,
                }));
            }
            let v = serde_json::to_value(call_error).map_err(Error::SerdeJson)?;
            Ok(v.to_string())
        }
    }
}

/// # Errors
/// [`Error::SerdeJson`]
pub fn serialize_typed_call_result(typed: &TypedCallResult) -> Result<String> {
    Ok(typed.to_value()?.to_string())
}
