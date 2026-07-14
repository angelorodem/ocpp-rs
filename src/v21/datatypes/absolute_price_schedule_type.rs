//! `AbsolutePriceScheduleType`
use super::AdditionalSelectedServicesType;
use super::CustomDataType;
use super::DateTimeWrapper;
use super::OverstayRuleListType;
use super::PriceRuleStackType;
use super::RationalNumberType;
use super::TaxRuleType;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AbsolutePriceScheduleType {
    #[serde(with = "crate::v21::utils::rfc3339_date_time")]
    pub time_anchor: DateTimeWrapper,
    pub price_schedule_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub price_schedule_description: Option<String>,
    pub currency: String,
    pub language: String,
    pub price_algorithm: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub minimum_cost: Option<RationalNumberType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub maximum_cost: Option<RationalNumberType>,
    pub price_rule_stacks: Vec<PriceRuleStackType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tax_rules: Option<Vec<TaxRuleType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub overstay_rule_list: Option<OverstayRuleListType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub additional_selected_services: Option<Vec<AdditionalSelectedServicesType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
