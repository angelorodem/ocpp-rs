#![no_main]

use libfuzzer_sys::fuzz_target;
use ocpp_rs::v16::call::*;
use ocpp_rs::v16::data_types::*;
use ocpp_rs::v16::enums::*;
use ocpp_rs::v16::parse::{to_message, Message};


fuzz_target!(|data: Call| {
    let unpacked = serde_json::to_string(&data).unwrap();
    let packed = serde_json::from_str::<Call>(&unpacked);
    assert_eq!(data, packed.unwrap());    
});
