//! OCPP 1.6 protocol types and OCPP-J framing.
//!
//! # `CallResult` typing
//!
//! CALLRESULT has no action on the wire. Use [`pending`] (same model as [`crate::v21::pending`]).
//! Datetime formatting remains the strict v16 ISO8601 helpers in [`utils`].

pub mod call;
pub mod call_error;
pub mod call_result;
pub mod data_types;
pub mod enums;
pub mod log_helper;
pub mod parse;
pub mod pending;
pub mod response_trait;
pub mod rpc_error_code;
pub mod typed_call_result;
pub mod utils;

#[cfg(feature = "schema_validate")]
pub mod validate_gen;
