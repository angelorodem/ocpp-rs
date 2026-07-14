//! OCPP 2.1 SEND payload: `NotifyPeriodicEventStream`.

use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::DateTimeWrapper;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StreamDataElementType {
    pub t: f64,
    pub v: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NotifyPeriodicEventStream {
    pub data: Vec<StreamDataElementType>,
    pub id: i32,
    pub pending: i32,
    #[serde(with = "crate::v21::utils::rfc3339_date_time")]
    pub basetime: DateTimeWrapper,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
