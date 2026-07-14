//! PeriodicEventStreamParamsType
use serde::{Deserialize, Serialize};
use super::CustomDataType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PeriodicEventStreamParamsType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub interval: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub values: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
