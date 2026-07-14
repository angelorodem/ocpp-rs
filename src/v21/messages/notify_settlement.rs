//! OCPP 2.1 NotifySettlement request/response payloads.

use crate::v21::datatypes::AddressType;
use crate::v21::datatypes::CustomDataType;
use crate::v21::datatypes::DateTimeWrapper;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PaymentStatusEnumType {
    #[serde(rename = "Settled")]
    Settled,
    #[serde(rename = "Canceled")]
    Canceled,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "Failed")]
    Failed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NotifySettlementRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub transaction_id: Option<String>,
    pub psp_ref: String,
    pub status: PaymentStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub status_info: Option<String>,
    pub settlement_amount: f64,
    #[serde(with = "crate::v21::utils::rfc3339_date_time")]
    pub settlement_time: DateTimeWrapper,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub receipt_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub receipt_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub vat_company: Option<AddressType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub vat_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NotifySettlementResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub receipt_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub receipt_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub custom_data: Option<CustomDataType>,
}
