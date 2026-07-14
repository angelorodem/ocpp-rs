//! OCPP 2.1 GetPeriodicEventStream request/response payloads.

use crate::v21::datatypes::ConstantStreamDataType;
use crate::v21::datatypes::CustomDataType;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GetPeriodicEventStreamRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GetPeriodicEventStreamResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub constant_stream_data: Option<Vec<ConstantStreamDataType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
