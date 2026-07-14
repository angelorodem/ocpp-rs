//! SignedMeterValueType
use alloc::string::String;
use serde::{Deserialize, Serialize};
use super::CustomDataType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SignedMeterValueType {
    pub signed_meter_data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub signing_method: Option<String>,
    pub encoding_method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub public_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
