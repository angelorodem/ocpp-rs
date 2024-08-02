//!# OCPP-RS
//!
//! OCPP-RS is a Rust library for implementing the Open Charge Point Protocol (OCPP) in Rust.
//! it currently supports OCPP 1.6.
//!
//! ## Usage
//! In Cargo.toml, add the following dependency:
//! ```toml
//! [dependencies]
//! ocpp-rs = "0.1"
//! ```
//!
//! ## Example
//! Receiving a payload from a client:
//! ```rust
//! use ocpp_rs::v16::parse::{self, Message};
//! use ocpp_rs::v16::call::{Action, Call};
//! 
//! // Example incoming message
//! let incoming_text = "[2, \"19223201\", \"BootNotification\", { \"chargePointVendor\": \"VendorX\", \"chargePointModel\": \"SingleSocketCharger\" }]";
//! let incoming_message = parse::to_message(incoming_text);
//! if let Ok(Message::Call(call)) = incoming_message {
//!     match call.payload {
//!         Action::BootNotification(boot_notification) => {
//!            // Do something with boot_notification
//!         },
//!         _ => {
//!           // Handle other actions
//!         }
//!    }
//! }
//! ```
//!
//! Sending a payload to a client:
//! ```rust
//! use ocpp_rs::v16::call::StartTransaction;
//! use ocpp_rs::v16::call_result::{self, CallResult, ResultPayload};
//! use ocpp_rs::v16::data_types::IdTagInfo;
//! use ocpp_rs::v16::enums::ChargePointStatus;
//! use ocpp_rs::v16::parse::Message;
//! 
//! let response = Message::CallResult(CallResult::new(
//!     "1234".to_string(),
//!     ResultPayload::StartTransaction(call_result::StartTransaction {
//!         transaction_id: 0,
//!         id_tag_info: IdTagInfo {
//!             status: ocpp_rs::v16::enums::GenericStatus::Accepted,
//!             expiry_date: None,
//!             parent_id_tag: None,
//!         },
//!     }),
//! ));
//!```
//! 
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

pub mod errors;
pub mod v16;
