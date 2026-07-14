//! OCPP 2.1 `SetNetworkProfile` request/response payloads.

use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::StatusInfoType;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum APNAuthenticationEnumType {
    #[serde(rename = "PAP")]
    PAP,
    #[serde(rename = "CHAP")]
    CHAP,
    #[serde(rename = "NONE")]
    NONE,
    #[serde(rename = "AUTO")]
    AUTO,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct APNType {
    pub apn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub apn_user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub apn_password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sim_pin: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub preferred_network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub use_only_preferred_network: Option<bool>,
    pub apn_authentication: APNAuthenticationEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OCPPInterfaceEnumType {
    #[serde(rename = "Wired0")]
    Wired0,
    #[serde(rename = "Wired1")]
    Wired1,
    #[serde(rename = "Wired2")]
    Wired2,
    #[serde(rename = "Wired3")]
    Wired3,
    #[serde(rename = "Wireless0")]
    Wireless0,
    #[serde(rename = "Wireless1")]
    Wireless1,
    #[serde(rename = "Wireless2")]
    Wireless2,
    #[serde(rename = "Wireless3")]
    Wireless3,
    #[serde(rename = "Any")]
    Any,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OCPPTransportEnumType {
    #[serde(rename = "SOAP")]
    SOAP,
    #[serde(rename = "JSON")]
    JSON,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OCPPVersionEnumType {
    #[serde(rename = "OCPP12")]
    OCPP12,
    #[serde(rename = "OCPP15")]
    OCPP15,
    #[serde(rename = "OCPP16")]
    OCPP16,
    #[serde(rename = "OCPP20")]
    OCPP20,
    #[serde(rename = "OCPP201")]
    OCPP201,
    #[serde(rename = "OCPP21")]
    OCPP21,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SetNetworkProfileStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "Failed")]
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VPNEnumType {
    #[serde(rename = "IKEv2")]
    IKEv2,
    #[serde(rename = "IPSec")]
    IPSec,
    #[serde(rename = "L2TP")]
    L2TP,
    #[serde(rename = "PPTP")]
    PPTP,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VPNType {
    pub server: String,
    pub user: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub group: Option<String>,
    pub password: String,
    pub key: String,
    #[serde(rename = "type")]
    pub type_: VPNEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NetworkConnectionProfileType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub apn: Option<APNType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ocpp_version: Option<OCPPVersionEnumType>,
    pub ocpp_interface: OCPPInterfaceEnumType,
    pub ocpp_transport: OCPPTransportEnumType,
    pub message_timeout: i32,
    pub ocpp_csms_url: String,
    pub security_profile: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub identity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub basic_auth_password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub vpn: Option<VPNType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetNetworkProfileRequest {
    pub configuration_slot: i32,
    pub connection_data: NetworkConnectionProfileType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetNetworkProfileResponse {
    pub status: SetNetworkProfileStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub status_info: Option<StatusInfoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
