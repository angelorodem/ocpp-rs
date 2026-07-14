//! ChargingProfilePurposeEnumType
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChargingProfilePurposeEnumType {
    #[serde(rename = "ChargingStationExternalConstraints")]
    ChargingStationExternalConstraints,
    #[serde(rename = "ChargingStationMaxProfile")]
    ChargingStationMaxProfile,
    #[serde(rename = "TxDefaultProfile")]
    TxDefaultProfile,
    #[serde(rename = "TxProfile")]
    TxProfile,
    #[serde(rename = "PriorityCharging")]
    PriorityCharging,
    #[serde(rename = "LocalGeneration")]
    LocalGeneration,
}
