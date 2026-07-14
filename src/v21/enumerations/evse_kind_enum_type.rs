//! EvseKindEnumType
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EvseKindEnumType {
    #[serde(rename = "AC")]
    AC,
    #[serde(rename = "DC")]
    DC,
}
