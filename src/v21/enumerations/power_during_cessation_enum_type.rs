//! PowerDuringCessationEnumType
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PowerDuringCessationEnumType {
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "Reactive")]
    Reactive,
}
