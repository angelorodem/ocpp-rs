//! OCPP 2.1 `NotifyEVChargingNeeds` request/response payloads.

use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::DateTimeWrapper;
use crate::v21::datatypes::StatusInfoType;
use crate::v21::enumerations::DERControlEnumType;
use crate::v21::enumerations::EnergyTransferModeEnumType;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ACChargingParametersType {
    pub energy_amount: f64,
    pub ev_min_current: f64,
    pub ev_max_current: f64,
    pub ev_max_voltage: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ControlModeEnumType {
    #[serde(rename = "ScheduledControl")]
    ScheduledControl,
    #[serde(rename = "DynamicControl")]
    DynamicControl,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DCChargingParametersType {
    pub ev_max_current: f64,
    pub ev_max_voltage: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ev_max_power: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ev_energy_capacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub energy_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub state_of_charge: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub full_so_c: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub bulk_so_c: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EVPowerScheduleEntryType {
    pub duration: i32,
    pub power: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EVPowerScheduleType {
    pub ev_power_schedule_entries: Vec<EVPowerScheduleEntryType>,
    #[serde(with = "crate::v21::utils::rfc3339_date_time")]
    pub time_anchor: DateTimeWrapper,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EVPriceRuleType {
    pub energy_fee: f64,
    pub power_range_start: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EVAbsolutePriceScheduleEntryType {
    pub duration: i32,
    pub ev_price_rule: Vec<EVPriceRuleType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EVAbsolutePriceScheduleType {
    #[serde(with = "crate::v21::utils::rfc3339_date_time")]
    pub time_anchor: DateTimeWrapper,
    pub currency: String,
    pub ev_absolute_price_schedule_entries: Vec<EVAbsolutePriceScheduleEntryType>,
    pub price_algorithm: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EVEnergyOfferType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ev_absolute_price_schedule: Option<EVAbsolutePriceScheduleType>,
    pub ev_power_schedule: EVPowerScheduleType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum IslandingDetectionEnumType {
    #[serde(rename = "NoAntiIslandingSupport")]
    NoAntiIslandingSupport,
    #[serde(rename = "RoCoF")]
    RoCoF,
    #[serde(rename = "UVP_OVP")]
    UVPOVP,
    #[serde(rename = "UFP_OFP")]
    UFPOFP,
    #[serde(rename = "VoltageVectorShift")]
    VoltageVectorShift,
    #[serde(rename = "ZeroCrossingDetection")]
    ZeroCrossingDetection,
    #[serde(rename = "OtherPassive")]
    OtherPassive,
    #[serde(rename = "ImpedanceMeasurement")]
    ImpedanceMeasurement,
    #[serde(rename = "ImpedanceAtFrequency")]
    ImpedanceAtFrequency,
    #[serde(rename = "SlipModeFrequencyShift")]
    SlipModeFrequencyShift,
    #[serde(rename = "SandiaFrequencyShift")]
    SandiaFrequencyShift,
    #[serde(rename = "SandiaVoltageShift")]
    SandiaVoltageShift,
    #[serde(rename = "FrequencyJump")]
    FrequencyJump,
    #[serde(rename = "RCLQFactor")]
    RCLQFactor,
    #[serde(rename = "OtherActive")]
    OtherActive,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DERChargingParametersType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ev_supported_der_control: Option<Vec<DERControlEnumType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ev_over_excited_max_discharge_power: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ev_over_excited_power_factor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ev_under_excited_max_discharge_power: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ev_under_excited_power_factor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_apparent_power: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_charge_apparent_power: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_charge_apparent_power_l2: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_charge_apparent_power_l3: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_discharge_apparent_power: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_discharge_apparent_power_l2: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_discharge_apparent_power_l3: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_charge_reactive_power: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_charge_reactive_power_l2: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_charge_reactive_power_l3: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub min_charge_reactive_power: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub min_charge_reactive_power_l2: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub min_charge_reactive_power_l3: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_discharge_reactive_power: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_discharge_reactive_power_l2: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_discharge_reactive_power_l3: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub min_discharge_reactive_power: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub min_discharge_reactive_power_l2: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub min_discharge_reactive_power_l3: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub nominal_voltage: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub nominal_voltage_offset: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_nominal_voltage: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub min_nominal_voltage: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ev_inverter_manufacturer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ev_inverter_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ev_inverter_serial_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ev_inverter_sw_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ev_inverter_hw_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ev_islanding_detection_method: Option<Vec<IslandingDetectionEnumType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ev_islanding_trip_time: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ev_maximum_level1_dc_injection: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ev_duration_level1_dc_injection: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ev_maximum_level2_dc_injection: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ev_duration_level2_dc_injection: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ev_reactive_susceptance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ev_session_total_discharge_energy_available: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MobilityNeedsModeEnumType {
    #[serde(rename = "EVCC")]
    EVCC,
    #[serde(rename = "EVCC_SECC")]
    EVCCSECC,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NotifyEVChargingNeedsStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "Processing")]
    Processing,
    #[serde(rename = "NoChargingProfile")]
    NoChargingProfile,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct V2XChargingParametersType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub min_charge_power: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub min_charge_power_l2: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub min_charge_power_l3: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_charge_power: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_charge_power_l2: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_charge_power_l3: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub min_discharge_power: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub min_discharge_power_l2: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub min_discharge_power_l3: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_discharge_power: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_discharge_power_l2: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_discharge_power_l3: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub min_charge_current: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_charge_current: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub min_discharge_current: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_discharge_current: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub min_voltage: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_voltage: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ev_target_energy_request: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ev_min_energy_request: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ev_max_energy_request: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ev_min_v2_x_energy_request: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ev_max_v2_x_energy_request: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub target_so_c: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChargingNeedsType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ac_charging_parameters: Option<ACChargingParametersType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub der_charging_parameters: Option<DERChargingParametersType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ev_energy_offer: Option<EVEnergyOfferType>,
    pub requested_energy_transfer: EnergyTransferModeEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub dc_charging_parameters: Option<DCChargingParametersType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub v2x_charging_parameters: Option<V2XChargingParametersType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub available_energy_transfer: Option<Vec<EnergyTransferModeEnumType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub control_mode: Option<ControlModeEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub mobility_needs_mode: Option<MobilityNeedsModeEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::v21::utils::rfc3339_date_time_optional")]
    pub departure_time: Option<DateTimeWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NotifyEVChargingNeedsRequest {
    pub evse_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_schedule_tuples: Option<i32>,
    pub charging_needs: ChargingNeedsType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::v21::utils::rfc3339_date_time_optional")]
    pub timestamp: Option<DateTimeWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NotifyEVChargingNeedsResponse {
    pub status: NotifyEVChargingNeedsStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub status_info: Option<StatusInfoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
