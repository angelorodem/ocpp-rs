//! CostType
use super::CustomDataType;
use crate::v21::enumerations::CostKindEnumType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CostType {
    pub cost_kind: CostKindEnumType,
    pub amount: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub amount_multiplier: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
