//! RecurrencyKindEnumType
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RecurrencyKindEnumType {
    #[serde(rename = "Daily")]
    Daily,
    #[serde(rename = "Weekly")]
    Weekly,
}
