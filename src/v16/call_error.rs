use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CallError {
    pub unique_id: String,
    pub error_code: String,
    pub error_description: String,
    pub error_details: Vec<Value>
}