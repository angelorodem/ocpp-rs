//! ChargingRateUnitEnumType
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChargingRateUnitEnumType {
    #[serde(rename = "W")]
    W,
    #[serde(rename = "A")]
    A,
}
