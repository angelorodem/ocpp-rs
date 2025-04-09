//! # Call Result    
//! This module contains all response messages to a given `Call` message.\\
//! Note that `CallResult` does not contain a type field, so you need to handle special cases where JSON can be ambiguous.\\
//! These special cases are inside specific enum variants, `PossibleEmptyResponse` and `PossibleStatusResponse`.\ 
//! Inside `PossibleEmptyResponse` you will find `PossibleIdTagInfoResponse` that also contain two different cases.   
//!
//! ## Example
//!
//! Sending a payload to a client:
//! ```rust
//! use ocpp_rs::v16::call::StartTransaction;
//! use ocpp_rs::v16::call_result::{self, CallResult, ResultPayload};
//! use ocpp_rs::v16::data_types::IdTagInfo;
//! use ocpp_rs::v16::enums::ChargePointStatus;
//! use ocpp_rs::v16::parse::Message;
//!
//! let response = Message::CallResult(CallResult::new(
//!     "1234".to_string(),
//!     ResultPayload::StartTransaction(call_result::StartTransaction {
//!         transaction_id: 0,
//!         id_tag_info: IdTagInfo {
//!             status: ocpp_rs::v16::enums::ParsedGenericStatus::Accepted,
//!             expiry_date: None,
//!             parent_id_tag: None,
//!         },
//!     }),
//! ));
//!```

use super::data_types::{DateTimeWrapper, IdTagInfo, KeyValue};
use super::enums::ParsedGenericStatus;
use super::utils::iso8601_date_time;
use alloc::collections::btree_map::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_tuple::{Deserialize_tuple, Serialize_tuple};
use strum_macros::AsRefStr;

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
/// This enum is used to handle the different types of responses that might come from the server.  
pub enum ResultPayload {
    StartTransaction(StartTransaction),
    BootNotification(BootNotification),
    Heartbeat(Heartbeat),
    GetLocalListVersion(GetLocalListVersion),
    PossibleStatusResponse(StatusResponses),
    PossibleEmptyResponse(EmptyResponses),
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
/// Enum containing all possible empty responses that might come from the server.
///
///  Since some structs might come as empty due to the optional fields,\\
/// this enum is used to handle those cases, since the serializer has no way\\
/// to know which struct to use when deserializing, since there is no type field\\
/// in the `CallResult` spec.    
pub enum EmptyResponses {
    /// Matches all empty responses
    EmptyResponse(EmptyResponse),
    /// Remember to handle `EmptyResponse` before this since it is ambiguous when deserializing,
    /// and might get interpreted as `EmptyResponse` instead of `GenericIdTagInfoResponse`.
    GenericIdTagInfoResponse(GenericIdTagInfo),
    /// Remember to handle `EmptyResponse` before this since it is ambiguous when deserializing,
    /// and might get interpreted as `EmptyResponse` instead of `GetConfiguration`.
    GetConfiguration(GetConfiguration),
    /// Remember to handle `EmptyResponse` before this since it is ambiguous when deserializing,
    /// and might get interpreted as `EmptyResponse` instead of `GetDiagnostics`.
    GetDiagnostics(GetDiagnostics),
}

impl EmptyResponses {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::EmptyResponse(_) => true,
            Self::GetConfiguration(get_configuration) => get_configuration.is_empty(),
            Self::GetDiagnostics(get_diagnostics) => get_diagnostics.is_empty(),
            Self::GenericIdTagInfoResponse(generic_id_tag_info) => generic_id_tag_info.is_empty(),
        }
    }
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
/// Enum containing all possible status responses that might come from the server.
///
///  IMPORTANT: When deserializing data from JSON, optional fields might not be present,\\
/// even when fields are present and null, the deserializer will transform data with only `status` as\\
/// `GenericStatusResponse` instead of other structs that implement status plus other **optional** fields.\\
/// (cases that contain non optional fields are not affected)    
///     
/// This means, in case you are waiting for a response that matches a struct that contains `status`, you should\\
/// also check if first you receive a `GenericStatusResponse` that matches the same `unique_id` you've sent.    
///     
/// This is mostly due to the protocol not being properly projected, because Call does have the type field,\\
/// but `CallResult` does not.    
pub enum StatusResponses {
    StatusResponse(GenericStatusResponse),
    GetInstalledCertificateIds(GetInstalledCertificateIds),
    GetCompositeSchedule(GetCompositeSchedule),
    GetLog(GetLog),
    DataTransfer(DataTransfer),
}

impl StatusResponses {
    #[must_use]
    pub const fn get_status(&self) -> &ParsedGenericStatus {
        match self {
            Self::StatusResponse(generic_status_response) => &generic_status_response.status,
            Self::GetInstalledCertificateIds(get_installed_certificate_ids) => {
                &get_installed_certificate_ids.status
            }
            Self::GetCompositeSchedule(get_composite_schedule) => &get_composite_schedule.status,
            Self::GetLog(get_log) => &get_log.status,
            Self::DataTransfer(data_transfer) => &data_transfer.status,
        }
    }

    #[must_use]
    pub fn is_only_status(&self) -> bool {
        match self {
            Self::StatusResponse(generic_status_response) => {
                generic_status_response.is_only_status()
            }
            Self::GetInstalledCertificateIds(get_installed_certificate_ids) => {
                get_installed_certificate_ids.is_only_status()
            }
            Self::GetCompositeSchedule(get_composite_schedule) => {
                get_composite_schedule.is_only_status()
            }
            Self::GetLog(get_log) => get_log.is_only_status(),
            Self::DataTransfer(data_transfer) => data_transfer.is_only_status(),
        }
    }
}

pub trait Status {
    /// This return true if the type contains only the status field.    
    fn is_only_status(&self) -> bool;
}

pub trait PossibleEmpty {
    fn is_empty(&self) -> bool;
}

#[derive(Debug, PartialEq, Eq, Serialize_tuple, Deserialize_tuple, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CallResult {
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

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BootNotification {
    /// ISO 8601 timestamp    
    #[serde(with = "iso8601_date_time")]
    pub current_time: DateTimeWrapper,
    /// Interval in seconds    
    pub interval: i32,
    pub status: ParsedGenericStatus,
}

impl Status for BootNotification {
    fn is_only_status(&self) -> bool {
        false
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Heartbeat {
    #[serde(with = "iso8601_date_time")]
    pub current_time: DateTimeWrapper,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
/// This struct will come as empty.\\
/// This might be ill interpreted by the deserializer.    
pub struct EmptyResponse {}

impl PossibleEmpty for EmptyResponse {
    fn is_empty(&self) -> bool {
        true
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StartTransaction {
    pub transaction_id: i32,
    pub id_tag_info: IdTagInfo,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
/// This struct might come as empty due to the optional fields.\\
/// This might be ill interpreted by the deserializer.    
pub struct GenericIdTagInfo {
    pub id_tag_info: Option<IdTagInfo>,
}

impl PossibleEmpty for GenericIdTagInfo {
    fn is_empty(&self) -> bool {
        self.id_tag_info.is_none()
    }
}

impl GenericIdTagInfo {
    #[must_use]
    /// When waiting for a response that might contain `IdTagInfo` or `StopTransaction`,\\
    /// this function will return the `IdTagInfo` if it exists.\\
    /// No need for matching    
    pub fn get_id_tag_info(self) -> Option<IdTagInfo> {
        self.id_tag_info
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
/// Generic status response that might come as empty.
///
/// IMPORTANT: When deserializing data from JSON, optional fields might not be present,\\
/// even when fields are present and null, the deserializer will transform data with only `status` as\\
/// `GenericStatusResponse` instead of other structs that implement status plus other **optional** fields.\\
/// (cases that contain non optional fields are not affected)    
///     
/// This means, in case you are waiting for a response that matches a struct that contains `status`, you should\\
/// also check if first you receive a `GenericStatusResponse` that matches the same `unique_id` you've sent.    
///     
/// This is mostly due to the protocol not being properly projected, because Call does have the type field,\\
/// but `CallResult` does not.\\
/// TODO: create a generic and use strict types for each type
pub struct GenericStatusResponse {
    pub status: ParsedGenericStatus,
}

impl Status for GenericStatusResponse {
    fn is_only_status(&self) -> bool {
        true
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
/// Result including the certificate hash data
///
/// IMPORTANT: When deserializing data from JSON, optional fields might not be present,\\
/// even when present and null, the deserializer will transform data with only `status` as\\
/// `GenericStatusResponse` instead of this struct.    
///     
/// This means, in case you are waiting for a response that matches this struct, you should\\
/// also check if first you receive a `GenericStatusResponse` that matches the same `unique_id` you've sent.    
///     
/// This is mostly due to the protocol not being properly projected, because Call does have the type field,\\
/// but `CallResult` does not.    
pub struct GetInstalledCertificateIds {
    pub status: ParsedGenericStatus,
    pub certificate_hash_data: Option<Vec<String>>,
}

impl Status for GetInstalledCertificateIds {
    fn is_only_status(&self) -> bool {
        self.certificate_hash_data.is_none()
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
/// Result including the composite schedule
///
///  IMPORTANT: When deserializing data from JSON, optional fields might not be present,\\
/// even when present and null, the deserializer will transform data with only `status` as\\
/// `GenericStatusResponse` instead of this struct.    
///     
/// This means, in case you are waiting for a response that matches this struct, you should\\
/// also check if first you receive a `GenericStatusResponse` that matches the same `unique_id` you've sent.    
///     
/// This is mostly due to the protocol not being properly projected, because Call does have the type field,\\
/// but `CallResult` does not.    
pub struct GetCompositeSchedule {
    pub status: ParsedGenericStatus,
    pub connector_id: Option<i32>,
    pub schedule_start: Option<String>,
    pub charging_schedule: Option<BTreeMap<String, String>>,
}

impl Status for GetCompositeSchedule {
    fn is_only_status(&self) -> bool {
        self.connector_id.is_none()
            && self.schedule_start.is_none()
            && self.charging_schedule.is_none()
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
/// This struct might come as empty due to the optional fields.\\
/// This might be ill interpreted by the deserializer.    
pub struct GetConfiguration {
    pub configuration_key: Option<Vec<KeyValue>>,
    pub unknown_key: Option<Vec<String>>,
}

impl PossibleEmpty for GetConfiguration {
    fn is_empty(&self) -> bool {
        self.configuration_key.is_none() && self.unknown_key.is_none()
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
/// This struct might come as empty due to the optional fields.\\
/// This might be ill interpreted by the deserializer.    
pub struct GetDiagnostics {
    pub file_name: Option<String>,
}

impl PossibleEmpty for GetDiagnostics {
    fn is_empty(&self) -> bool {
        self.file_name.is_none()
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GetLocalListVersion {
    pub list_version: i32,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
/// Non-standard for returning the filename for the logs
///
///  IMPORTANT: When deserializing data from JSON, optional fields might not be present,\\
/// even when present and null, the deserializer will transform data with only `status` as\\
/// `GenericStatusResponse` instead of this struct.    
///     
/// This means, in case you are waiting for a response that matches this struct, you should\\
/// also check if first you receive a `GenericStatusResponse` that matches the same `unique_id` you've sent.    
///     
/// This is mostly due to the protocol not being properly projected, because Call does have the type field,\\
/// but `CallResult` does not.    
pub struct GetLog {
    pub status: ParsedGenericStatus,
    pub filename: Option<String>,
}

impl Status for GetLog {
    fn is_only_status(&self) -> bool {
        self.filename.is_none()
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UnlockConnector {
    pub status: GenericStatusResponse,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
/// Generic data transfer response
///
///  IMPORTANT: When deserializing data from JSON, optional fields might not be present,\\
/// even when present and null, the deserializer will transform data with only `status` as\\
/// `GenericStatusResponse` instead of this struct.    
///     
/// This means, in case you are waiting for a response that matches this struct, you should\\
/// also check if first you receive a `GenericStatusResponse` that matches the same `unique_id` you've sent.    
///     
/// This is mostly due to the protocol not being properly projected, because Call does have the type field,\\
/// but `CallResult` does not.    
pub struct DataTransfer {
    pub status: ParsedGenericStatus,
    pub data: Option<String>,
}

impl Status for DataTransfer {
    fn is_only_status(&self) -> bool {
        self.data.is_none()
    }
}
