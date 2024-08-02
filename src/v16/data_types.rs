use super::enums::{
    AuthorizationStatus,
    ChargingProfileKindType,
    ChargingProfilePurposeType,
    ChargingRateUnitType,
    HashAlgorithm,
    Location,
    Measurand,
    Phase,
    ReadingContext,
    RecurrencyKind,
    UnitOfMeasure,
    ValueFormat,
};
use crate::v16::utils::DateTimeWrapper;
use serde::{Deserialize, Serialize};
use validator::Validate;
use super::utils::{iso8601_date_time_optional,iso8601_date_time};
use arbitrary::Arbitrary;


//TODO: Check if this expiry date is serialized to the iso format
#[derive(Arbitrary)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone, Validate, Default)]
#[serde(rename_all = "camelCase")]
pub struct IdTagInfo {
    /// Optional. This contains the date at which idTag should be removed from the Authorization Cache.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "iso8601_date_time_optional")]
    pub expiry_date: Option<DateTimeWrapper>,
    /// Optional. This contains the parent-identifier. IdToken
    #[validate(length(min = 1, max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id_tag: Option<String>,
    /// Required. This contains whether the idTag has been accepted or not by the Central System.
    pub status: AuthorizationStatus,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizationData {
    /// Required. The identifier to which this authorization applies.
    #[validate(length(min = 1, max = 20))]
    pub id_tag: String,
    /// Optional. (Required when UpdateType is Full) This contains information about authorization status, expiry and parent id. For a Differential update the following applies: If this element is present, then this entry SHALL be added or updated in the Local Authorization List. If this element is absent, than the entry for this idtag in the Local Authorization List SHALL be deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_tag_info: Option<IdTagInfo>,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargingSchedulePeriod {
    /// Required. Start of the period, in seconds from the start of schedule. The value of StartPeriod also defines the stop time of the previous period.
    pub start_period: i32,
    /// Required. Charging rate limit during the schedule period, in the applicable chargingRateUnit, for example in Amperes or Watts. Accepts at most one digit fraction (e.g. 8.1).
    pub limit: f32,
    /// Optional. The number of phases that can be used for charging. If a number of phases is needed, numberPhases=3 will be assumed unless another number is given.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_phases: Option<i32>,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargingSchedule {
    /// Optional. Duration of the charging schedule in seconds. If the duration is left empty, the last period will continue indefinitely or until end of the transaction in case startSchedule is absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// Optional. Starting point of an absolute schedule. If absent the schedule will be relative to start of charging.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "iso8601_date_time_optional")]
    pub start_schedule: Option<DateTimeWrapper>,
    /// Required. The unit of measure Limit is expressed in.
    pub charging_rate_unit: ChargingRateUnitType,
    /// Required. List of ChargingSchedulePeriod elements defining maximum power or current usage over time. The startSchedule of the first ChargingSchedulePeriod SHALL always be 0.
    pub charging_schedule_period: Vec<ChargingSchedulePeriod>,
    /// Optional. Minimum charging rate supported by the electric vehicle. The unit of measure is defined by the chargingRateUnit. This parameter is intended to be used by a local smart charging algorithm to optimize the power allocation for in the case a charging process is inefficient at lower charging rates. Accepts at most one digit fraction (e.g. 8.1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_charging_rate: Option<f32>,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargingProfile {
    pub charging_profile_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<i32>,
    pub stack_level: u32,
    pub charging_profile_purpose: ChargingProfilePurposeType,
    pub charging_profile_kind: ChargingProfileKindType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrency_kind: Option<RecurrencyKind>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "iso8601_date_time_optional")]
    pub valid_from: Option<DateTimeWrapper>,
    #[serde(default)]
    #[serde(with = "iso8601_date_time_optional")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<DateTimeWrapper>,
    pub charging_schedule: ChargingSchedule,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct KeyValue {
    /// Required.
    #[validate(length(min = 1, max = 50))]
    pub key: String,
    /// Required. False if the value can be set with the ChangeConfiguration message.
    pub readonly: bool,
    /// Optional. If key is known but not set, this field may be absent.
    #[validate(length(min = 1, max = 500))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Validate, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SampledValue {
    /// Required. Value as a “Raw” (decimal) number or “SignedData”. Field Type is “string” to allow for digitally signed data readings.
    /// Decimal numeric values are also acceptable to allow fractional values for measurands such as Temperature and Current.
    pub value: String,
    /// Optional. Type of detail value: start, end or sample. Default = “Sample.Periodic”
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ReadingContext>,
    /// Optional. Raw or signed data. Default = “Raw”
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ValueFormat>,
    /// Optional. Type of measurement. Default = “Energy.Active.Import.Register”
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurand: Option<Measurand>,
    /// Optional. indicates how the measured value is to be interpreted. For instance between L1 and neutral (L1-N) Please note that not all values of phase are applicable to all Measurands. When phase is absent, the measured value is interpreted as an overall value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<Phase>,
    /// Optional. Location of measurement. Default=”Outlet”
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// Optional. Unit of the value. Default = “Wh” if the (default) measurand is an “Energy” type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<UnitOfMeasure>,
}

#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MeterValue {
    /// Required. Timestamp for measured value(s).
    #[serde(with = "iso8601_date_time")]
    pub timestamp: DateTimeWrapper,
    /// Required. One or more measured values
    pub sampled_value: Vec<SampledValue>,
}

// Not standard 1.6
#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CertificateHashData {
    /// Required. Used algorithms for the hashes provided.
    pub hash_algorithm: HashAlgorithm,
    /// Required. Hashed value of the Issuer DN (Distinguished Name).
    #[validate(length(min = 0, max = 128))]
    pub issuer_name_hash: String,
    /// Required. Hashed value of the issuers public key
    #[validate(length(min = 0, max = 128))]
    pub issuer_key_hash: String,
    /// Required. The serial number of the certificate.
    #[validate(length(min = 0, max = 40))]
    pub serial_number: String,
}

// Not standard 1.6
#[derive(Arbitrary)]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Firmware {
    pub location: String,
    #[serde(with = "iso8601_date_time")]
    pub retrieve_date_time: DateTimeWrapper,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "iso8601_date_time_optional")]
    pub install_date_time: Option<DateTimeWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_certificate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

// Not standard 1.6
#[derive(Arbitrary)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogParameters {
    pub remote_location: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "iso8601_date_time_optional")]
    pub oldest_timestamp: Option<DateTimeWrapper>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "iso8601_date_time_optional")]
    pub latest_timestamp: Option<DateTimeWrapper>,
}