//! `AdditionalInfoType`
use super::CustomDataType;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AdditionalInfoType {
    pub additional_id_token: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
