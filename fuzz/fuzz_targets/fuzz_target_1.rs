#![no_main]

use libfuzzer_sys::fuzz_target;
use ocpp_rs::v16::call::*;
use ocpp_rs::v16::data_types::*;
use ocpp_rs::v16::enums::*;
use ocpp_rs::v16::parse::{to_message, Message};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = to_message(s);
    }
});
