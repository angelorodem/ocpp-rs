//! DERCurveType
use alloc::vec::Vec;
use crate::v21::enumerations::DERUnitEnumType;
use serde::{Deserialize, Serialize};
use super::CustomDataType;
use super::DERCurvePointsType;
use super::DateTimeWrapper;
use super::HysteresisType;
use super::ReactivePowerParamsType;
use super::VoltageParamsType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DERCurveType {
    pub curve_data: Vec<DERCurvePointsType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub hysteresis: Option<HysteresisType>,
    pub priority: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub reactive_power_params: Option<ReactivePowerParamsType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub voltage_params: Option<VoltageParamsType>,
    pub y_unit: DERUnitEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub response_time: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::v21::utils::rfc3339_date_time_optional")]
    pub start_time: Option<DateTimeWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
