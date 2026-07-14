//! `MessageContentType`
use super::CustomDataType;
use crate::v21::enumerations::MessageFormatEnumType;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MessageContentType {
    pub format: MessageFormatEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub language: Option<String>,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
