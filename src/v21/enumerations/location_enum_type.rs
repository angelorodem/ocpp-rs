//! `LocationEnumType`
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LocationEnumType {
    #[serde(rename = "Body")]
    Body,
    #[serde(rename = "Cable")]
    Cable,
    #[serde(rename = "EV")]
    EV,
    #[serde(rename = "Inlet")]
    Inlet,
    #[serde(rename = "Outlet")]
    Outlet,
    #[serde(rename = "Upstream")]
    Upstream,
}
