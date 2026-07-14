//! `OperationModeEnumType`
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OperationModeEnumType {
    #[serde(rename = "Idle")]
    Idle,
    #[serde(rename = "ChargingOnly")]
    ChargingOnly,
    #[serde(rename = "CentralSetpoint")]
    CentralSetpoint,
    #[serde(rename = "ExternalSetpoint")]
    ExternalSetpoint,
    #[serde(rename = "ExternalLimits")]
    ExternalLimits,
    #[serde(rename = "CentralFrequency")]
    CentralFrequency,
    #[serde(rename = "LocalFrequency")]
    LocalFrequency,
    #[serde(rename = "LocalLoadBalancing")]
    LocalLoadBalancing,
}
