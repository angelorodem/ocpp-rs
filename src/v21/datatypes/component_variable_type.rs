//! ComponentVariableType
use serde::{Deserialize, Serialize};
use super::ComponentType;
use super::CustomDataType;
use super::VariableType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ComponentVariableType {
    pub component: ComponentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub variable: Option<VariableType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
