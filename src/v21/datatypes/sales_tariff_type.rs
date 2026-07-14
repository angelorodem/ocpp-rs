//! `SalesTariffType`
use super::CustomDataType;
use super::SalesTariffEntryType;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SalesTariffType {
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sales_tariff_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub num_e_price_levels: Option<i32>,
    pub sales_tariff_entry: Vec<SalesTariffEntryType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
