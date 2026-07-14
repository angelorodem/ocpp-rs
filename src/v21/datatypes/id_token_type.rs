//! `IdTokenType`
use super::AdditionalInfoType;
use super::CustomDataType;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IdTokenType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub additional_info: Option<Vec<AdditionalInfoType>>,
    pub id_token: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
