//! `TariffType`
use super::CustomDataType;
use super::DateTimeWrapper;
use super::MessageContentType;
use super::PriceType;
use super::TariffEnergyType;
use super::TariffFixedType;
use super::TariffTimeType;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TariffType {
    pub tariff_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<Vec<MessageContentType>>,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub energy: Option<TariffEnergyType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::v21::utils::rfc3339_date_time_optional")]
    pub valid_from: Option<DateTimeWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub charging_time: Option<TariffTimeType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub idle_time: Option<TariffTimeType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub fixed_fee: Option<TariffFixedType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub reservation_time: Option<TariffTimeType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub reservation_fixed: Option<TariffFixedType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub min_cost: Option<PriceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_cost: Option<PriceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
