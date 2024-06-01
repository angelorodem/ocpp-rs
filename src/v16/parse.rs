use super::call::Call;
use super::call_result::CallResult;
use super::call_error::CallError;

use serde::{de::Error, Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Message {
    Call(Call),
    CallResult(CallResult),
    CallError(CallError),
}

pub fn parse(data: &str) -> Result<Message> {
    
    let v: Vec<serde_json::Value> = serde_json::from_str(data)?;

    #[cfg(debug_assertions)]
    println!("Input: {:?}\n", v[3]);
    
    match v[0].as_i64() {
        Some(2) => {
            let call: Call = Call {
                unique_id: v[1].as_str().unwrap().to_string(),
                action: v[2].as_str().unwrap().to_string(),
                payload: serde_json::from_value(v[3].clone())?,
            };

            Ok(Message::Call(call))
        }
        Some(3) => {
            let call_result: CallResult = CallResult {
                unique_id: v[1].as_str().unwrap().to_string(),
                payload: serde_json::from_value(v[2].clone())?,
            };
            Ok(Message::CallResult(call_result))
        }
        Some(4) => {
            let call_error: CallError = CallError {
                unique_id: v[1].as_str().unwrap().to_string(),
                error_code: v[2].as_str().unwrap().to_string(),
                error_description: v[3].as_str().unwrap().to_string(),
                error_details: serde_json::from_value(v[4].clone())?,
            };
            Ok(Message::CallError(call_error))
        }
        _ => Err(serde_json::Error::custom("Invalid message type")),
        
    }

    
    
}