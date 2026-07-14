//! DERUnitEnumType
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DERUnitEnumType {
    #[serde(rename = "Not_Applicable")]
    NotApplicable,
    #[serde(rename = "PctMaxW")]
    PctMaxW,
    #[serde(rename = "PctMaxVar")]
    PctMaxVar,
    #[serde(rename = "PctWAvail")]
    PctWAvail,
    #[serde(rename = "PctVarAvail")]
    PctVarAvail,
    #[serde(rename = "PctEffectiveV")]
    PctEffectiveV,
}
