use super::enums::{
    AuthorizationStatus,
    ChargingProfileKindType,
    ChargingProfilePurposeType,
    ChargingRateUnitType,
    CiStringType,
    HashAlgorithm,
    Location,
    Measurand,
    Phase,
    ReadingContext,
    RecurrencyKind,
    UnitOfMeasure,
    ValueFormat,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct IdTagInfo {
    status: AuthorizationStatus,
    parent_id_tag: Option<String>,
    expiry_date: Option<NaiveDateTime>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AuthorizationData {
    id_tag: String,
    id_tag_info: Option<IdTagInfo>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ChargingSchedulePeriod {
    start_period: i32,
    limit: f32,
    number_phases: Option<i32>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ChargingSchedule {
    charging_rate_unit: ChargingRateUnitType,
    charging_schedule_period: Vec<ChargingSchedulePeriod>,
    duration: Option<i32>,
    start_schedule: Option<String>,
    min_charging_rate: Option<f32>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ChargingProfile {
    charging_profile_id: i32,
    stack_level: i32,
    charging_profile_purpose: ChargingProfilePurposeType,
    charging_profile_kind: ChargingProfileKindType,
    charging_schedule: ChargingSchedule,
    transaction_id: Option<i32>,
    recurrency_kind: Option<RecurrencyKind>,
    valid_from: Option<String>,
    valid_to: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct KeyValue {
    key: String,
    readonly: bool,
    value: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SampledValue {
    value: String,
    context: Option<ReadingContext>,
    format: Option<ValueFormat>,
    measurand: Option<Measurand>,
    phase: Option<Phase>,
    location: Option<Location>,
    unit: Option<UnitOfMeasure>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MeterValue {
    timestamp: String,
    sampled_value: Vec<SampledValue>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CertificateHashData {
    hash_algorithm: HashAlgorithm,
    issuer_name_hash: String,
    issuer_key_hash: String,
    serial_number: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Firmware {
    location: String,
    retrieve_date_time: String,
    signing_certificate: String,
    install_date_time: Option<String>,
    signature: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogParameters {
    remote_location: String,
    oldest_timestamp: Option<String>,
    latest_timestamp: Option<String>,
}