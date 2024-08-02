#![no_main]

use libfuzzer_sys::fuzz_target;
use ocpp_rs::v16::call_result::*;
use ocpp_rs::v16::data_types::*;
use ocpp_rs::v16::enums::*;
use ocpp_rs::v16::parse::{to_message, Message};

// Will crash for SignCertificate and CancelReservation
// Open charge alliance did not project the call result properly like the call
// there is no type field to determine the type of the call result
// structures that have equal bodies in JSON might be serialized as the first that matches
// in alphabetical order
fuzz_target!(|data: CallResult| {
    println!("\nincoming: {:?}", data);
    let unpacked = serde_json::to_string(&data).unwrap();
    println!("serialized: {:?}", unpacked);
    let packed = serde_json::from_str::<CallResult>(&unpacked);
    println!("deserialized: {:?}\n", packed);
    assert_eq!(data, packed.unwrap());    
});
