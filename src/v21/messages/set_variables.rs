//! OCPP 2.1 SetVariables request/response payloads.

use crate::v21::datatypes::ComponentType;
use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::StatusInfoType;
use crate::v21::datatypes::VariableType;
use crate::v21::enumerations::AttributeEnumType;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetVariableDataType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub attribute_type: Option<AttributeEnumType>,
    pub attribute_value: String,
    pub component: ComponentType,
    pub variable: VariableType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SetVariableStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "UnknownComponent")]
    UnknownComponent,
    #[serde(rename = "UnknownVariable")]
    UnknownVariable,
    #[serde(rename = "NotSupportedAttributeType")]
    NotSupportedAttributeType,
    #[serde(rename = "RebootRequired")]
    RebootRequired,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetVariableResultType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub attribute_type: Option<AttributeEnumType>,
    pub attribute_status: SetVariableStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub attribute_status_info: Option<StatusInfoType>,
    pub component: ComponentType,
    pub variable: VariableType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetVariablesRequest {
    pub set_variable_data: Vec<SetVariableDataType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetVariablesResponse {
    pub set_variable_result: Vec<SetVariableResultType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
