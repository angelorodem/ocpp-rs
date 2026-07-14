//! OCPP 2.1 `NotifyReport` request/response payloads.

use crate::v21::datatypes::ComponentType;
use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::DateTimeWrapper;
use crate::v21::datatypes::VariableType;
use crate::v21::enumerations::AttributeEnumType;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DataEnumType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "decimal")]
    Decimal,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "dateTime")]
    DateTime,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "OptionList")]
    OptionList,
    #[serde(rename = "SequenceList")]
    SequenceList,
    #[serde(rename = "MemberList")]
    MemberList,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MutabilityEnumType {
    #[serde(rename = "ReadOnly")]
    ReadOnly,
    #[serde(rename = "WriteOnly")]
    WriteOnly,
    #[serde(rename = "ReadWrite")]
    ReadWrite,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VariableAttributeType {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub type_: Option<AttributeEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub mutability: Option<MutabilityEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub persistent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub constant: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VariableCharacteristicsType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub unit: Option<String>,
    pub data_type: DataEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub min_limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_elements: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub values_list: Option<String>,
    pub supports_monitoring: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReportDataType {
    pub component: ComponentType,
    pub variable: VariableType,
    pub variable_attribute: Vec<VariableAttributeType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub variable_characteristics: Option<VariableCharacteristicsType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NotifyReportRequest {
    pub request_id: i32,
    #[serde(with = "crate::v21::utils::rfc3339_date_time")]
    pub generated_at: DateTimeWrapper,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub report_data: Option<Vec<ReportDataType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tbc: Option<bool>,
    pub seq_no: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NotifyReportResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
