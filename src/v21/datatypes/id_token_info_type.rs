//! `IdTokenInfoType`
use super::CustomDataType;
use super::DateTimeWrapper;
use super::IdTokenType;
use super::MessageContentType;
use crate::v21::enumerations::AuthorizationStatusEnumType;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IdTokenInfoType {
    pub status: AuthorizationStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::v21::utils::rfc3339_date_time_optional")]
    pub cache_expiry_date_time: Option<DateTimeWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub charging_priority: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub group_id_token: Option<IdTokenType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub language1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub language2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub evse_id: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub personal_message: Option<MessageContentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
