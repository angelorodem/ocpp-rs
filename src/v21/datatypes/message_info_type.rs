//! MessageInfoType
use super::ComponentType;
use super::CustomDataType;
use super::DateTimeWrapper;
use super::MessageContentType;
use crate::v21::enumerations::MessagePriorityEnumType;
use crate::v21::enumerations::MessageStateEnumType;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MessageInfoType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub display: Option<ComponentType>,
    pub id: i32,
    pub priority: MessagePriorityEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub state: Option<MessageStateEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::v21::utils::rfc3339_date_time_optional")]
    pub start_date_time: Option<DateTimeWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::v21::utils::rfc3339_date_time_optional")]
    pub end_date_time: Option<DateTimeWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub transaction_id: Option<String>,
    pub message: MessageContentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub message_extra: Option<Vec<MessageContentType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
