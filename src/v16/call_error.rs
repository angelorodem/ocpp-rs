use serde_json::Value;
use serde_tuple::{Serialize_tuple, Deserialize_tuple};


#[derive(Debug, PartialEq, Eq, Serialize_tuple, Deserialize_tuple, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CallError {
    pub(super) message_id: i32,
    pub unique_id: String,
    pub error_code: String,
    pub error_description: String,
    pub error_details: Vec<Value>
}

impl CallError {
    pub fn new(unique_id: String, error_code: String, error_description: String, error_details: Vec<Value>) -> Self {
        Self {
            message_id: 4,
            unique_id,
            error_code,
            error_description,
            error_details
        }
    }
}