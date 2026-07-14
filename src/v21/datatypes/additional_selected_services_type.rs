//! AdditionalSelectedServicesType
use super::CustomDataType;
use super::RationalNumberType;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AdditionalSelectedServicesType {
    pub service_fee: RationalNumberType,
    pub service_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
