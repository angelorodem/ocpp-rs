//! OCPP 2.1 `TransactionEvent` request/response payloads.

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
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

crate::lenient_str_enum! {
    /// Charging state within a transaction.
    pub enum ChargingStateEnumType {
        EVConnected,
        Charging,
        SuspendedEV,
        SuspendedEVSE,
        Idle,
    }
    @unknown Unknown
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

crate::lenient_str_enum! {
    /// Reason a transaction stopped (`TransactionType.stoppedReason`).
    ///
    /// Unknown / vendor-specific wire strings are accepted as [`ReasonEnumType::Unknown`]
    /// so a proprietary value cannot fail the whole `TransactionEvent` parse.
    pub enum ReasonEnumType {
        DeAuthorized,
        EmergencyStop,
        EnergyLimitReached,
        EVDisconnected,
        GroundFault,
        ImmediateReset,
        MasterPass,
        Local,
        LocalOutOfCredit,
        Other,
        OvercurrentFault,
        PowerLoss,
        PowerQuality,
        Reboot,
        Remote,
        SOCLimitReached,
        StoppedByEV,
        TimeLimitReached,
        Timeout,
        ReqEnergyTransferRejected,
    }
    @unknown Unknown
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

crate::lenient_str_enum! {
    /// Why a `TransactionEvent` was sent.
    pub enum TriggerReasonEnumType {
        AbnormalCondition,
        Authorized,
        CablePluggedIn,
        ChargingRateChanged,
        ChargingStateChanged,
        CostLimitReached,
        Deauthorized,
        EnergyLimitReached,
        EVCommunicationLost,
        EVConnectTimeout,
        EVDeparted,
        EVDetected,
        LimitSet,
        MeterValueClock,
        MeterValuePeriodic,
        OperationModeChanged,
        RemoteStart,
        RemoteStop,
        ResetCommand,
        RunningCost,
        SignedDataReceived,
        SoCLimitReached,
        StopAuthorized,
        TariffChanged,
        TariffNotAccepted,
        TimeLimitReached,
        Trigger,
        TxResumed,
        UnlockCommand,
    }
    @unknown Unknown
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
