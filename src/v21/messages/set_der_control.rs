//! OCPP 2.1 `SetDERControl` request/response payloads.

use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::DERCurveType;
use crate::v21::datatypes::EnterServiceType;
use crate::v21::datatypes::FixedPFType;
use crate::v21::datatypes::FixedVarType;
use crate::v21::datatypes::FreqDroopType;
use crate::v21::datatypes::GradientType;
use crate::v21::datatypes::LimitMaxDischargeType;
use crate::v21::datatypes::StatusInfoType;
use crate::v21::enumerations::DERControlEnumType;
use crate::v21::enumerations::DERControlStatusEnumType;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetDERControlRequest {
    pub is_default: bool,
    pub control_id: String,
    pub control_type: DERControlEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub curve: Option<DERCurveType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub enter_service: Option<EnterServiceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub fixed_pf_absorb: Option<FixedPFType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub fixed_pf_inject: Option<FixedPFType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub fixed_var: Option<FixedVarType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub freq_droop: Option<FreqDroopType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub gradient: Option<GradientType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub limit_max_discharge: Option<LimitMaxDischargeType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetDERControlResponse {
    pub status: DERControlStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub status_info: Option<StatusInfoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub superseded_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
