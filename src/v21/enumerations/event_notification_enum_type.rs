//! `EventNotificationEnumType`
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EventNotificationEnumType {
    #[serde(rename = "HardWiredNotification")]
    HardWiredNotification,
    #[serde(rename = "HardWiredMonitor")]
    HardWiredMonitor,
    #[serde(rename = "PreconfiguredMonitor")]
    PreconfiguredMonitor,
    #[serde(rename = "CustomMonitor")]
    CustomMonitor,
}
