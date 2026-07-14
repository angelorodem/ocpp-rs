//! `ConstantStreamDataType`
use super::CustomDataType;
use super::PeriodicEventStreamParamsType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ConstantStreamDataType {
    pub id: i32,
    pub params: PeriodicEventStreamParamsType,
    pub variable_monitoring_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
