//! `ChargingProfileKindEnumType`
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChargingProfileKindEnumType {
    #[serde(rename = "Absolute")]
    Absolute,
    #[serde(rename = "Recurring")]
    Recurring,
    #[serde(rename = "Relative")]
    Relative,
    #[serde(rename = "Dynamic")]
    Dynamic,
}
