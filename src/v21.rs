//! OCPP 2.1 protocol types and OCPP-J framing.
//!
//! # CallResult typing
//!
//! CALLRESULT frames do not include an action name. Options:
//!
//! - [`pending::PendingCalls`] — in-process `messageId → Action` (sticky sessions)
//! - [`pending::PendingActionNames`] / [`pending::resolve_with_action_name`] — store only the
//!   action string (Redis-friendly across load-balanced nodes)
//! - [`call_result::CallResultRaw::into_typed`] / [`call_result::deserialize_call_result`] —
//!   when your app already knows `T`
//! - [`call_result::CallResultRaw::probe_candidates`] / [`pending::try_resolve_unique`] —
//!   last resort when the expected type is unknown (**ambiguous** for empty/status-only JSON)
//!
//! See [`pending`] module docs for scaling shortcomings.

pub mod call;
pub mod call_error;
pub mod call_result;
pub mod call_result_error;
pub mod datatypes;
pub mod enumerations;
pub mod log_helper;
pub mod messages;
pub mod parse;
pub mod pending;
pub mod response_trait;
pub mod send;
pub mod typed_call_result;
pub mod utils;
