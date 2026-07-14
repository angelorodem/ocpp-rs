//! OCPP 2.1 ReportDERControl request/response payloads.

use alloc::string::String;
use alloc::vec::Vec;
use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::DERCurveType;
use crate::v21::datatypes::EnterServiceType;
use crate::v21::datatypes::FixedPFType;
use crate::v21::datatypes::FixedVarType;
use crate::v21::datatypes::FreqDroopType;
use crate::v21::datatypes::GradientType;
use crate::v21::datatypes::LimitMaxDischargeType;
use crate::v21::enumerations::DERControlEnumType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DERCurveGetType {
    pub curve: DERCurveType,
    pub id: String,
    pub curve_type: DERControlEnumType,
    pub is_default: bool,
    pub is_superseded: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EnterServiceGetType {
    pub enter_service: EnterServiceType,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FixedPFGetType {
    pub fixed_pf: FixedPFType,
    pub id: String,
    pub is_default: bool,
    pub is_superseded: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FixedVarGetType {
    pub fixed_var: FixedVarType,
    pub id: String,
    pub is_default: bool,
    pub is_superseded: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FreqDroopGetType {
    pub freq_droop: FreqDroopType,
    pub id: String,
    pub is_default: bool,
    pub is_superseded: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GradientGetType {
    pub gradient: GradientType,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LimitMaxDischargeGetType {
    pub id: String,
    pub is_default: bool,
    pub is_superseded: bool,
    pub limit_max_discharge: LimitMaxDischargeType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReportDERControlRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub curve: Option<Vec<DERCurveGetType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub enter_service: Option<Vec<EnterServiceGetType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub fixed_pf_absorb: Option<Vec<FixedPFGetType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub fixed_pf_inject: Option<Vec<FixedPFGetType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub fixed_var: Option<Vec<FixedVarGetType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub freq_droop: Option<Vec<FreqDroopGetType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub gradient: Option<Vec<GradientGetType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub limit_max_discharge: Option<Vec<LimitMaxDischargeGetType>>,
    pub request_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tbc: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReportDERControlResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
