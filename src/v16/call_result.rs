use super::enums::{
    GenericStatus,
    UnlockStatus,
};
use super::data_types::{DateTimeWrapper, IdTagInfo};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::AsRefStr;
use serde_tuple::{Serialize_tuple, Deserialize_tuple};
use super::utils::iso8601_date_time;
use arbitrary::Arbitrary;


#[derive(Arbitrary)]
#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
/// The position of each variant is important, since the deserializer will try to match the first variant    
pub enum ResultPayload {
    StartTransaction(StartTransaction),
    BootNotification(BootNotification),
    Heartbeat(Heartbeat),
    PossibleStatusResponse(StatusResponses),
    PossibleEmptyResponse(EmptyResponses),
}


#[derive(Arbitrary)]
#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum IdTagInfoResponses {
    Authorize(Authorize),
    StopTransaction(StopTransaction),
}

impl IdTagInfoResponses {
    #[must_use]
    /// When waiting for a response that might contain `IdTagInfo` or `StopTransaction`,    
    /// this function will return the `IdTagInfo` if it exists.    
    /// No need for matching    
    pub fn get_id_tag_info(self) -> Option<IdTagInfo> {
        match self {
            Self::Authorize(id_tag_info) => Some(id_tag_info.id_tag_info),
            Self::StopTransaction(stop_transaction) => stop_transaction.id_tag_info,
        }
    }
    
}


#[derive(Arbitrary)]
#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
/// Since some structs might come as empty due to the optional fields,    
/// this enum is used to handle those cases, since the serializer has no way    
/// to know which struct to use when deserializing, since there is no type field    
/// in the `CallResult` spec.    
pub enum EmptyResponses {
    EmptyResponse(EmptyResponse),
    GetConfiguration(GetConfiguration),
    GetDiagnostics(GetDiagnostics),
    /// For Stop transaction    
    PossibleIdTagInfoResponse(IdTagInfoResponses),
}

impl EmptyResponses {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::EmptyResponse(_) => true,
            Self::GetConfiguration(get_configuration) => get_configuration.is_empty(),
            Self::GetDiagnostics(get_diagnostics) => get_diagnostics.is_empty(),
            Self::PossibleIdTagInfoResponse(id_tag_info_responses) => match id_tag_info_responses {
                IdTagInfoResponses::Authorize(_) => false,
                IdTagInfoResponses::StopTransaction(stop_transaction) => stop_transaction.is_empty(),
            },
        }
    }
    
}

#[derive(Arbitrary)]
#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
/// IMPORTANT: When deserializing data from JSON, optional fields might not be present,    
/// even when fields are present and null, the deserializer will transform data with only `status` as    
/// `GenericStatusResponse` instead of other structs that implement status plus other **optional** fields.    
/// (cases that contain non optional fields are not affected)    
///     
/// This means, in case you are waiting for a response that matches a struct that contains `status`, you should    
/// also check if first you receive a `GenericStatusResponse` that matches the same `unique_id` you've sent.    
///     
/// This is mostly due to the protocol not being properly projected, because Call does have the type field,    
/// but `CallResult` does not.    
pub enum StatusResponses {
    GetInstalledCertificateIds(GetInstalledCertificateIds),
    GetCompositeSchedule(GetCompositeSchedule),
    GetLog(GetLog),
    DataTransfer(DataTransfer),
    StatusResponse(GenericStatusResponse),
}

impl StatusResponses {
    #[must_use]
    pub const fn get_status(&self) -> &GenericStatus {
        match self {
            Self::StatusResponse(generic_status_response) => &generic_status_response.status,
            Self::GetInstalledCertificateIds(get_installed_certificate_ids) => &get_installed_certificate_ids.status,
            Self::GetCompositeSchedule(get_composite_schedule) => &get_composite_schedule.status,
            Self::GetLog(get_log) => &get_log.status,
            Self::DataTransfer(data_transfer) => &data_transfer.status,            
        }
    }

    #[must_use]
    pub fn is_only_status(&self) -> bool {
        match self {
            Self::StatusResponse(generic_status_response) => generic_status_response.is_only_status(),
            Self::GetInstalledCertificateIds(get_installed_certificate_ids) => get_installed_certificate_ids.is_only_status(),
            Self::GetCompositeSchedule(get_composite_schedule) => get_composite_schedule.is_only_status(),
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

impl Status for BootNotification {
    fn is_only_status(&self) -> bool {
        false
    } 
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
/// This struct will come as empty.    
/// This might be ill interpreted by the deserializer.    
pub struct EmptyResponse {}

impl PossibleEmpty for EmptyResponse {
    fn is_empty(&self) -> bool {
        true
    }        
}

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
/// This struct might come as empty due to the optional fields.    
/// This might be ill interpreted by the deserializer.    
pub struct StopTransaction {
    pub id_tag_info: Option<IdTagInfo>,
}

impl PossibleEmpty for StopTransaction {
    fn is_empty(&self) -> bool {
        self.id_tag_info.is_none()
    }    
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
/// IMPORTANT: When deserializing data from JSON, optional fields might not be present,    
/// even when fields are present and null, the deserializer will transform data with only `status` as    
/// `GenericStatusResponse` instead of other structs that implement status plus other **optional** fields.    
/// (cases that contain non optional fields are not affected)    
///     
/// This means, in case you are waiting for a response that matches a struct that contains `status`, you should    
/// also check if first you receive a `GenericStatusResponse` that matches the same `unique_id` you've sent.    
///     
/// This is mostly due to the protocol not being properly projected, because Call does have the type field,    
/// but `CallResult` does not.    
pub struct GenericStatusResponse {
    pub status: GenericStatus,
}

impl Status for GenericStatusResponse {
    fn is_only_status(&self) -> bool {
        true
    }
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
/// IMPORTANT: When deserializing data from JSON, optional fields might not be present,    
/// even when present and null, the deserializer will transform data with only `status` as    
/// `GenericStatusResponse` instead of this struct.    
///     
/// This means, in case you are waiting for a response that matches this struct, you should    
/// also check if first you receive a `GenericStatusResponse` that matches the same `unique_id` you've sent.    
///     
/// This is mostly due to the protocol not being properly projected, because Call does have the type field,    
/// but `CallResult` does not.    
pub struct GetInstalledCertificateIds {
    pub status: GenericStatus,
    pub certificate_hash_data: Option<Vec<String>>,
}

impl Status for GetInstalledCertificateIds {
    fn is_only_status(&self) -> bool {
        self.certificate_hash_data.is_none()
    }       
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
/// IMPORTANT: When deserializing data from JSON, optional fields might not be present,    
/// even when present and null, the deserializer will transform data with only `status` as    
/// `GenericStatusResponse` instead of this struct.    
///     
/// This means, in case you are waiting for a response that matches this struct, you should    
/// also check if first you receive a `GenericStatusResponse` that matches the same `unique_id` you've sent.    
///     
/// This is mostly due to the protocol not being properly projected, because Call does have the type field,    
/// but `CallResult` does not.    
pub struct GetCompositeSchedule {
    pub status: GenericStatus,
    pub connector_id: Option<i32>,
    pub schedule_start: Option<String>,
    pub charging_schedule: Option<HashMap<String, String>>,
}

impl Status for GetCompositeSchedule {
    fn is_only_status(&self) -> bool {
        self.connector_id.is_none() && self.schedule_start.is_none() && self.charging_schedule.is_none()
    }      
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
/// This struct might come as empty due to the optional fields.    
/// This might be ill interpreted by the deserializer.    
pub struct GetConfiguration {
    pub configuration_key: Option<Vec<String>>,
    pub unknown_key: Option<Vec<String>>,
}

impl PossibleEmpty for GetConfiguration {
    fn is_empty(&self) -> bool {
        self.configuration_key.is_none() && self.unknown_key.is_none()
    }        
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
/// This struct might come as empty due to the optional fields.    
/// This might be ill interpreted by the deserializer.    
pub struct GetDiagnostics {
    pub file_name: Option<String>,
}

impl PossibleEmpty for GetDiagnostics {
    fn is_empty(&self) -> bool {
        self.file_name.is_none()
    }        
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
/// IMPORTANT: When deserializing data from JSON, optional fields might not be present,    
/// even when present and null, the deserializer will transform data with only `status` as    
/// `GenericStatusResponse` instead of this struct.    
///     
/// This means, in case you are waiting for a response that matches this struct, you should    
/// also check if first you receive a `GenericStatusResponse` that matches the same `unique_id` you've sent.    
///     
/// This is mostly due to the protocol not being properly projected, because Call does have the type field,    
/// but `CallResult` does not.    
pub struct GetLog {
    pub status: GenericStatus,
    pub filename: Option<String>,
}

impl Status for GetLog {
    fn is_only_status(&self) -> bool {
        self.filename.is_none()
    }      
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
/// IMPORTANT: When deserializing data from JSON, optional fields might not be present,    
/// even when present and null, the deserializer will transform data with only `status` as    
/// `GenericStatusResponse` instead of this struct.    
///     
/// This means, in case you are waiting for a response that matches this struct, you should    
/// also check if first you receive a `GenericStatusResponse` that matches the same `unique_id` you've sent.    
///     
/// This is mostly due to the protocol not being properly projected, because Call does have the type field,    
/// but `CallResult` does not.    
pub struct DataTransfer {
    pub status: GenericStatus,
    pub data: Option<String>,
}

impl Status for DataTransfer {
   fn is_only_status(&self) -> bool {
        self.data.is_none()
   }       
}
