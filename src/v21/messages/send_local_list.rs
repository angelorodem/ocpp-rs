//! OCPP 2.1 SendLocalList request/response payloads.

use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::IdTokenInfoType;
use crate::v21::datatypes::IdTokenType;
use crate::v21::datatypes::StatusInfoType;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AuthorizationData {
    pub id_token: IdTokenType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub id_token_info: Option<IdTokenInfoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SendLocalListStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Failed")]
    Failed,
    #[serde(rename = "VersionMismatch")]
    VersionMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UpdateEnumType {
    #[serde(rename = "Differential")]
    Differential,
    #[serde(rename = "Full")]
    Full,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SendLocalListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub local_authorization_list: Option<Vec<AuthorizationData>>,
    pub version_number: i32,
    pub update_type: UpdateEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SendLocalListResponse {
    pub status: SendLocalListStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub status_info: Option<StatusInfoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
