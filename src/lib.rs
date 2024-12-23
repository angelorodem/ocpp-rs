//!# OCPP-RS
//!
//! OCPP-RS is a Rust library for implementing the Open Charge Point Protocol (OCPP) in Rust.   
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
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::correctness,
    clippy::cargo
)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]
#![deny(clippy::multiple_crate_versions)]

extern crate alloc;
pub mod errors;
pub mod v16;
