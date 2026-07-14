//! `GenericDeviceModelStatusEnumType`
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GenericDeviceModelStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "NotSupported")]
    NotSupported,
    #[serde(rename = "EmptyResultSet")]
    EmptyResultSet,
}
