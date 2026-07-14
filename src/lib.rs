//!# OCPP-RS
//!
//! OCPP-RS is a Rust library for implementing the Open Charge Point Protocol (OCPP) in Rust.\
//! It supports OCPP 1.6 and OCPP 2.1.
//!
//! ## Usage
//! In Cargo.toml, add the following dependency:
//! ```toml
//! [dependencies]
//! ocpp-rs = "^0.4"
//! ```
//!
//! # Particularities
//!
//! ## OCPP 1.6
//! CALLRESULT has no action on the wire. Blind parse yields [`v16::call_result::CallResultRaw`].
//! Correlate with [`v16::pending::PendingCalls`] (or action-name store /
//! [`v16::pending::resolve_with_action_name`]). Do **not** guess from JSON shape.
//! Datetime: parse always accepts RFC3339; serialize defaults to `%Y-%m-%dT%H:%M:%S%.3fZ`
//! (enable `datetime_serialize_rfc3339` to emit RFC3339 millis — see [`datetime`]).
//! Migration from 0.2.x: `guides/migration-0.4.md`.
//!
//! ## OCPP 2.1
//! Same CallResult model under [`v21::pending`]. Framing includes types 2–6
//! (CALL, CALLRESULT, CALLERROR, CALLRESULTERROR, SEND).
//! Datetime helpers share [`datetime`] with v16 (RFC3339 parse; `%.3fZ` serialize by default).
//!
//! ## Example (1.6 CALL)
//! ```rust
//! use ocpp_rs::v16::parse::{self, Message};
//! use ocpp_rs::v16::call::Action;
//!
//! let incoming_text = "[2, \"19223201\", \"BootNotification\", { \"chargePointVendor\": \"VendorX\", \"chargePointModel\": \"SingleSocketCharger\" }]";
//! let incoming_message = parse::deserialize_to_message(incoming_text);
//! if let Ok(Message::Call(call)) = incoming_message {
//!     match call.payload {
//!         Action::BootNotification(_boot_notification) => {}
//!         _ => {}
//!     }
//! }
//! ```
//!
//! ## Example (1.6 CallResult with PendingCalls)
//! ```rust
//! use ocpp_rs::v16::call::{Action, Call, Heartbeat};
//! use ocpp_rs::v16::parse::TypedMessage;
//! use ocpp_rs::v16::pending::PendingCalls;
//! use ocpp_rs::v16::typed_call_result::TypedCallResult;
//!
//! let mut pending = PendingCalls::new();
//! let _ = pending
//!     .send_call(Call::new("1".to_string(), Action::Heartbeat(Heartbeat {})))
//!     .expect("serialize");
//! let typed = pending
//!     .deserialize_typed(r#"[3, "1", {"currentTime": "2024-01-01T00:00:00.000Z"}]"#)
//!     .expect("typed");
//! assert!(matches!(
//!     typed,
//!     TypedMessage::CallResult(TypedCallResult::Heartbeat(_))
//! ));
//! ```
//!
//! ## Example (2.1)
//! ```rust
//! use ocpp_rs::v21::call::{Action, Call};
//! use ocpp_rs::v21::messages::heartbeat::HeartbeatRequest;
//! use ocpp_rs::v21::parse::TypedMessage;
//! use ocpp_rs::v21::pending::PendingCalls;
//! use ocpp_rs::v21::typed_call_result::TypedCallResult;
//!
//! let mut pending = PendingCalls::new();
//! let call = Call::new(
//!     "1".to_string(),
//!     Action::Heartbeat(HeartbeatRequest { custom_data: None }),
//! );
//! let _outgoing = pending.send_call(call).expect("serialize");
//! let typed = pending
//!     .deserialize_typed(r#"[3, "1", {"currentTime": "2024-01-01T00:00:00.000Z"}]"#)
//!     .expect("typed result");
//! assert!(matches!(
//!     typed,
//!     TypedMessage::CallResult(TypedCallResult::Heartbeat(_))
//! ));
//! ```
//!
#![no_std]
#![warn(clippy::pedantic, clippy::nursery)]
#![warn(
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::correctness
)]
#![warn(
    clippy::unseparated_literal_suffix,
    clippy::unreachable,
    clippy::unneeded_field_pattern,
    clippy::unnecessary_self_imports,
    clippy::unimplemented,
    clippy::try_err,
    clippy::todo,
    clippy::tests_outside_test_module,
    clippy::suspicious_xor_used_as_pow,
    clippy::string_slice,
    clippy::string_lit_chars_any,
    clippy::string_add,
    clippy::shadow_unrelated,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::renamed_function_params,
    clippy::ref_patterns,
    clippy::rc_mutex,
    clippy::rc_buffer,
    clippy::print_stdout,
    clippy::print_stderr,
    clippy::mutex_atomic,
    clippy::map_with_unused_argument_over_ranges,
    clippy::lossy_float_literal,
    clippy::integer_division,
    clippy::if_then_some_else_none,
    clippy::format_push_string,
    clippy::float_cmp_const,
    clippy::empty_enum_variants_with_brackets,
    clippy::else_if_without_else
)]
#![allow(clippy::module_name_repetitions)]
#![deny(
    clippy::unwrap_used,
    clippy::undocumented_unsafe_blocks,
    clippy::expect_used,
    clippy::panic_in_result_fn,
    clippy::unwrap_in_result,
    clippy::unused_result_ok,
    clippy::panic,
    clippy::get_unwrap,
    clippy::multiple_crate_versions,
    clippy::std_instead_of_core,
    clippy::std_instead_of_alloc,
    unused_must_use
)]
#![recursion_limit = "256"]

extern crate alloc;
pub mod datetime;
pub mod errors;
pub mod v16;
pub mod v21;
pub mod validate;
