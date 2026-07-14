//! TariffConditionsFixedType
use super::CustomDataType;
use crate::v21::enumerations::DayOfWeekEnumType;
use crate::v21::enumerations::EvseKindEnumType;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TariffConditionsFixedType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub start_time_of_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub end_time_of_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub day_of_week: Option<Vec<DayOfWeekEnumType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub valid_from_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub valid_to_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub evse_kind: Option<EvseKindEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub payment_brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub payment_recognition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
