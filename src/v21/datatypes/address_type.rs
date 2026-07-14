//! `AddressType`
use super::CustomDataType;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AddressType {
    pub name: String,
    pub address1: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub address2: Option<String>,
    pub city: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub postal_code: Option<String>,
    pub country: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
