//!# OCPP-RS
//!
//! OCPP-RS is a Rust library for implementing the Open Charge Point Protocol (OCPP) in Rust.\
//! it currently only supports OCPP 1.6.
//!
//! ## Usage
//! In Cargo.toml, add the following dependency:
//! ```toml
//! [dependencies]
//! ocpp-rs = "^0.2"
//! ```
//!
//! # Particularities
//! Since the original OCPP 1.6 protocol does not contain a type field for `CallResult`, when parsing `CallResult`lt, you need to handle
//! Special cases where JSON payloads are ambiguous, like empty objects like: ```{}```, these might get serialized as a `EmptyResponse` instead of the variant
//! you are waiting for like `GetConfiguration`.
//!
//! Look at this file to see how to properly handle `CallResults`: [`valid_call_result.rs`](https://github.com/angelorodem/ocpp-rs/blob/main/fuzz/fuzz_targets/valid_call_result.rs)
//!
//! ## Example
//! Receiving a payload from a client:
//! ```rust
//! use ocpp_rs::v16::parse::{self, Message};
//! use ocpp_rs::v16::call::{Action, Call};
//!
//! // Example incoming message
//! let incoming_text = "[2, \"19223201\", \"BootNotification\", { \"chargePointVendor\": \"VendorX\", \"chargePointModel\": \"SingleSocketCharger\" }]";
//! let incoming_message = parse::deserialize_to_message(incoming_text);
//! if let Ok(Message::Call(call)) = incoming_message {
//!
//! // Handle incoming message (Check the type of the message)
//! match call.payload {
//!         Action::BootNotification(_boot_notification) => {
//!             // Do something with boot_notification
//!         }
//!         _ => {
//!             // Handle other actions
//!         }
//!     }
//! }
//! ```
//!
//! Sending a payload to a client:
//! ```rust
//! use ocpp_rs::v16::parse::Message;
//! use ocpp_rs::v16::call_result::CallResult;
//! use ocpp_rs::v16::call_result::ResultPayload;
//! use ocpp_rs::v16::call_result;
//! use ocpp_rs::v16::data_types::IdTagInfo;
//! use ocpp_rs::v16::parse;
//! let response = Message::CallResult(CallResult::new(
//!     "1234".to_string(),
//!     ResultPayload::StartTransaction(call_result::StartTransaction {
//!         transaction_id: 0,
//!         id_tag_info: IdTagInfo {
//!             status: ocpp_rs::v16::enums::ParsedGenericStatus::Accepted,
//!             expiry_date: None,
//!             parent_id_tag: None,
//!         },
//!     }),
//! ));
//!
//! let json = parse::serialize_message(&response);
//! println!("Sending to client: {:?}", json);
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
    clippy::string_to_string,
    clippy::string_slice,
    clippy::string_lit_chars_any,
    clippy::string_add,
    clippy::std_instead_of_core,
    clippy::std_instead_of_alloc,
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
    unused_must_use
)]
#![recursion_limit = "256"]

extern crate alloc;
pub mod errors;
pub mod v16;
