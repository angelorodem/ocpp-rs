use serde_json::Value;
use serde_tuple::*;


#[derive(Debug, PartialEq, Eq, Serialize_tuple, Deserialize_tuple, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CallError {
    pub unique_id: String,
    pub error_code: String,
    pub error_description: String,
    pub error_details: Vec<Value>
}