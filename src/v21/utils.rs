//! Datetime serde helpers for OCPP 2.1 (re-exports shared [`crate::datetime`] modules).
//!
//! Parse is always RFC3339. Serialize defaults to `%.3fZ` unless
//! `datetime_serialize_rfc3339` is enabled.

pub use crate::datetime::{
    date_time as rfc3339_date_time, date_time_optional as rfc3339_date_time_optional,
};
