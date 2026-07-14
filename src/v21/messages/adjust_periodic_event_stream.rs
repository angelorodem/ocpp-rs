//! OCPP 2.1 `AdjustPeriodicEventStream` request/response payloads.

use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::PeriodicEventStreamParamsType;
use crate::v21::datatypes::StatusInfoType;
use crate::v21::enumerations::GenericStatusEnumType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AdjustPeriodicEventStreamRequest {
    pub id: i32,
    pub params: PeriodicEventStreamParamsType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AdjustPeriodicEventStreamResponse {
    pub status: GenericStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub status_info: Option<StatusInfoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
