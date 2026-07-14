//! SampledValueType
use super::CustomDataType;
use super::SignedMeterValueType;
use super::UnitOfMeasureType;
use crate::v21::enumerations::LocationEnumType;
use crate::v21::enumerations::MeasurandEnumType;
use crate::v21::enumerations::PhaseEnumType;
use crate::v21::enumerations::ReadingContextEnumType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SampledValueType {
    pub value: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub measurand: Option<MeasurandEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub context: Option<ReadingContextEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub phase: Option<PhaseEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub location: Option<LocationEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub signed_meter_value: Option<SignedMeterValueType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub unit_of_measure: Option<UnitOfMeasureType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
