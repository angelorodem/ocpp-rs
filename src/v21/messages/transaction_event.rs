//! OCPP 2.1 TransactionEvent request/response payloads.

use alloc::string::String;
use alloc::vec::Vec;
use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::DateTimeWrapper;
use crate::v21::datatypes::EVSEType;
use crate::v21::datatypes::IdTokenInfoType;
use crate::v21::datatypes::IdTokenType;
use crate::v21::datatypes::MessageContentType;
use crate::v21::datatypes::MeterValueType;
use crate::v21::datatypes::PriceType;
use crate::v21::datatypes::TransactionLimitType;
use crate::v21::enumerations::OperationModeEnumType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChargingStateEnumType {
    #[serde(rename = "EVConnected")]
    EVConnected,
    #[serde(rename = "Charging")]
    Charging,
    #[serde(rename = "SuspendedEV")]
    SuspendedEV,
    #[serde(rename = "SuspendedEVSE")]
    SuspendedEVSE,
    #[serde(rename = "Idle")]
    Idle,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CostDimensionEnumType {
    #[serde(rename = "Energy")]
    Energy,
    #[serde(rename = "MaxCurrent")]
    MaxCurrent,
    #[serde(rename = "MinCurrent")]
    MinCurrent,
    #[serde(rename = "MaxPower")]
    MaxPower,
    #[serde(rename = "MinPower")]
    MinPower,
    #[serde(rename = "IdleTIme")]
    IdleTIme,
    #[serde(rename = "ChargingTime")]
    ChargingTime,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CostDimensionType {
    #[serde(rename = "type")]
    pub type_: CostDimensionEnumType,
    pub volume: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChargingPeriodType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub dimensions: Option<Vec<CostDimensionType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tariff_id: Option<String>,
    #[serde(with = "crate::v21::utils::rfc3339_date_time")]
    pub start_period: DateTimeWrapper,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PreconditioningStatusEnumType {
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Ready")]
    Ready,
    #[serde(rename = "NotReady")]
    NotReady,
    #[serde(rename = "Preconditioning")]
    Preconditioning,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReasonEnumType {
    #[serde(rename = "DeAuthorized")]
    DeAuthorized,
    #[serde(rename = "EmergencyStop")]
    EmergencyStop,
    #[serde(rename = "EnergyLimitReached")]
    EnergyLimitReached,
    #[serde(rename = "EVDisconnected")]
    EVDisconnected,
    #[serde(rename = "GroundFault")]
    GroundFault,
    #[serde(rename = "ImmediateReset")]
    ImmediateReset,
    #[serde(rename = "MasterPass")]
    MasterPass,
    #[serde(rename = "Local")]
    Local,
    #[serde(rename = "LocalOutOfCredit")]
    LocalOutOfCredit,
    #[serde(rename = "Other")]
    Other,
    #[serde(rename = "OvercurrentFault")]
    OvercurrentFault,
    #[serde(rename = "PowerLoss")]
    PowerLoss,
    #[serde(rename = "PowerQuality")]
    PowerQuality,
    #[serde(rename = "Reboot")]
    Reboot,
    #[serde(rename = "Remote")]
    Remote,
    #[serde(rename = "SOCLimitReached")]
    SOCLimitReached,
    #[serde(rename = "StoppedByEV")]
    StoppedByEV,
    #[serde(rename = "TimeLimitReached")]
    TimeLimitReached,
    #[serde(rename = "Timeout")]
    Timeout,
    #[serde(rename = "ReqEnergyTransferRejected")]
    ReqEnergyTransferRejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TariffCostEnumType {
    #[serde(rename = "NormalCost")]
    NormalCost,
    #[serde(rename = "MinCost")]
    MinCost,
    #[serde(rename = "MaxCost")]
    MaxCost,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TotalPriceType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub excl_tax: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub incl_tax: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TotalCostType {
    pub currency: String,
    pub type_of_cost: TariffCostEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub fixed: Option<PriceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub energy: Option<PriceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub charging_time: Option<PriceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub idle_time: Option<PriceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub reservation_time: Option<PriceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub reservation_fixed: Option<PriceType>,
    pub total: TotalPriceType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TotalUsageType {
    pub energy: f64,
    pub charging_time: i32,
    pub idle_time: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub reservation_time: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CostDetailsType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub charging_periods: Option<Vec<ChargingPeriodType>>,
    pub total_cost: TotalCostType,
    pub total_usage: TotalUsageType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub failure_to_calculate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub failure_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransactionEventEnumType {
    #[serde(rename = "Ended")]
    Ended,
    #[serde(rename = "Started")]
    Started,
    #[serde(rename = "Updated")]
    Updated,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TransactionType {
    pub transaction_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub charging_state: Option<ChargingStateEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub time_spent_charging: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stopped_reason: Option<ReasonEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub remote_start_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub operation_mode: Option<OperationModeEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tariff_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub transaction_limit: Option<TransactionLimitType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TriggerReasonEnumType {
    #[serde(rename = "AbnormalCondition")]
    AbnormalCondition,
    #[serde(rename = "Authorized")]
    Authorized,
    #[serde(rename = "CablePluggedIn")]
    CablePluggedIn,
    #[serde(rename = "ChargingRateChanged")]
    ChargingRateChanged,
    #[serde(rename = "ChargingStateChanged")]
    ChargingStateChanged,
    #[serde(rename = "CostLimitReached")]
    CostLimitReached,
    #[serde(rename = "Deauthorized")]
    Deauthorized,
    #[serde(rename = "EnergyLimitReached")]
    EnergyLimitReached,
    #[serde(rename = "EVCommunicationLost")]
    EVCommunicationLost,
    #[serde(rename = "EVConnectTimeout")]
    EVConnectTimeout,
    #[serde(rename = "EVDeparted")]
    EVDeparted,
    #[serde(rename = "EVDetected")]
    EVDetected,
    #[serde(rename = "LimitSet")]
    LimitSet,
    #[serde(rename = "MeterValueClock")]
    MeterValueClock,
    #[serde(rename = "MeterValuePeriodic")]
    MeterValuePeriodic,
    #[serde(rename = "OperationModeChanged")]
    OperationModeChanged,
    #[serde(rename = "RemoteStart")]
    RemoteStart,
    #[serde(rename = "RemoteStop")]
    RemoteStop,
    #[serde(rename = "ResetCommand")]
    ResetCommand,
    #[serde(rename = "RunningCost")]
    RunningCost,
    #[serde(rename = "SignedDataReceived")]
    SignedDataReceived,
    #[serde(rename = "SoCLimitReached")]
    SoCLimitReached,
    #[serde(rename = "StopAuthorized")]
    StopAuthorized,
    #[serde(rename = "TariffChanged")]
    TariffChanged,
    #[serde(rename = "TariffNotAccepted")]
    TariffNotAccepted,
    #[serde(rename = "TimeLimitReached")]
    TimeLimitReached,
    #[serde(rename = "Trigger")]
    Trigger,
    #[serde(rename = "TxResumed")]
    TxResumed,
    #[serde(rename = "UnlockCommand")]
    UnlockCommand,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TransactionEventRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cost_details: Option<CostDetailsType>,
    pub event_type: TransactionEventEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub meter_value: Option<Vec<MeterValueType>>,
    #[serde(with = "crate::v21::utils::rfc3339_date_time")]
    pub timestamp: DateTimeWrapper,
    pub trigger_reason: TriggerReasonEnumType,
    pub seq_no: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub offline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub number_of_phases_used: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cable_max_current: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub reservation_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub preconditioning_status: Option<PreconditioningStatusEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub evse_sleep: Option<bool>,
    pub transaction_info: TransactionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub evse: Option<EVSEType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub id_token: Option<IdTokenType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TransactionEventResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub total_cost: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub charging_priority: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub id_token_info: Option<IdTokenInfoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub transaction_limit: Option<TransactionLimitType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub updated_personal_message: Option<MessageContentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub updated_personal_message_extra: Option<Vec<MessageContentType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
