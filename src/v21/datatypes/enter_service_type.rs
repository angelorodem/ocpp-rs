//! EnterServiceType
use serde::{Deserialize, Serialize};
use super::CustomDataType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EnterServiceType {
    pub priority: i32,
    pub high_voltage: f64,
    pub low_voltage: f64,
    pub high_freq: f64,
    pub low_freq: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub delay: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub random_delay: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ramp_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
