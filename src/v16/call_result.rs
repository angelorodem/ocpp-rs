use super::enums::{
    GenericStatus,
    UnlockStatus,
};
use super::data_types::IdTagInfo;
use crate::v16::utils::DateTimeWrapper;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::AsRefStr;
use serde_tuple::{Serialize_tuple, Deserialize_tuple};
use super::utils::iso8601_date_time;
use arbitrary::Arbitrary;


#[derive(Arbitrary)]
#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ResultPayload {
    GenericStatusResponse(GenericStatusResponse),
    Authorize(Authorize),
    BootNotification(BootNotification),
    DataTransfer(DataTransfer),
    GetCompositeSchedule(GetCompositeSchedule),
    GetConfiguration(GetConfiguration),
    GetDiagnostics(GetDiagnostics),
    GetInstalledCertificateIds(GetInstalledCertificateIds),
    GetLocalListVersion(GetLocalListVersion),
    GetLog(GetLog),
    Heartbeat(Heartbeat),
    StartTransaction(StartTransaction),
    StopTransaction(StopTransaction),
    EmptyResponse(EmptyResponse),
}

#[derive(Debug, PartialEq, Eq, Serialize_tuple, Deserialize_tuple, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CallResult	 {
    pub(super) message_id: i32,
    pub unique_id: String,
    pub payload: ResultPayload,
}

impl CallResult {
    #[must_use]
    pub const fn new(unique_id: String, payload: ResultPayload) -> Self {
        Self {
            message_id: 3,
            unique_id,
            payload,
        }
    }
}

impl Arbitrary<'_> for CallResult {
    fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
        let unique_id = i32::arbitrary(u)?;
        let payload = ResultPayload::arbitrary(u)?;

        Ok(Self {
            message_id: 3,
            unique_id: unique_id.to_string(),
            payload,
        }) 
    }
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Authorize {
    pub id_tag_info: IdTagInfo,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BootNotification {
    /// ISO 8601 timestamp
    #[serde(with = "iso8601_date_time")]
    pub current_time: DateTimeWrapper,
    /// Interval in seconds
    pub interval: i32,
    pub status: GenericStatus,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Heartbeat {
    #[serde(with = "iso8601_date_time")]
    pub current_time: DateTimeWrapper,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct EmptyResponse {}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartTransaction {
    pub transaction_id: i32,
    pub id_tag_info: IdTagInfo,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct StopTransaction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_tag_info: Option<IdTagInfo>,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GenericStatusResponse {
    pub status: GenericStatus,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetInstalledCertificateIds {
    pub status: GenericStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_hash_data: Option<Vec<String>>,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCompositeSchedule {
    pub status: GenericStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_schedule: Option<HashMap<String, String>>,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_key: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_key: Option<Vec<String>>,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDiagnostics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLocalListVersion {
    pub list_version: i32,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLog {
    pub status: GenericStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct UnlockConnector {
    pub status: UnlockStatus,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DataTransfer {
    pub status: GenericStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}
