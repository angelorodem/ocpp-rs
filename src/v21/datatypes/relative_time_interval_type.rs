//! `RelativeTimeIntervalType`
use super::CustomDataType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RelativeTimeIntervalType {
    pub start: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
