//! `CostKindEnumType`
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CostKindEnumType {
    #[serde(rename = "CarbonDioxideEmission")]
    CarbonDioxideEmission,
    #[serde(rename = "RelativePricePercentage")]
    RelativePricePercentage,
    #[serde(rename = "RenewableGenerationPercentage")]
    RenewableGenerationPercentage,
}
