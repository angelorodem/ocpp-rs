//! OCPP 2.1 `AFRRSignal` request/response payloads.

use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::DateTimeWrapper;
use crate::v21::datatypes::StatusInfoType;
use crate::v21::enumerations::GenericStatusEnumType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AFRRSignalRequest {
    #[serde(with = "crate::v21::utils::rfc3339_date_time")]
    pub timestamp: DateTimeWrapper,
    pub signal: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AFRRSignalResponse {
    pub status: GenericStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub status_info: Option<StatusInfoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
