//! `MonitorEnumType`
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MonitorEnumType {
    #[serde(rename = "UpperThreshold")]
    UpperThreshold,
    #[serde(rename = "LowerThreshold")]
    LowerThreshold,
    #[serde(rename = "Delta")]
    Delta,
    #[serde(rename = "Periodic")]
    Periodic,
    #[serde(rename = "PeriodicClockAligned")]
    PeriodicClockAligned,
    #[serde(rename = "TargetDelta")]
    TargetDelta,
    #[serde(rename = "TargetDeltaRelative")]
    TargetDeltaRelative,
}
