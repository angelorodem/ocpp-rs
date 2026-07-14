//! `MessageStateEnumType`
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MessageStateEnumType {
    #[serde(rename = "Charging")]
    Charging,
    #[serde(rename = "Faulted")]
    Faulted,
    #[serde(rename = "Idle")]
    Idle,
    #[serde(rename = "Unavailable")]
    Unavailable,
    #[serde(rename = "Suspended")]
    Suspended,
    #[serde(rename = "Discharging")]
    Discharging,
}
