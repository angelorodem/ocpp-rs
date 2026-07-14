//! `MessagePriorityEnumType`
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MessagePriorityEnumType {
    #[serde(rename = "AlwaysFront")]
    AlwaysFront,
    #[serde(rename = "InFront")]
    InFront,
    #[serde(rename = "NormalCycle")]
    NormalCycle,
}
