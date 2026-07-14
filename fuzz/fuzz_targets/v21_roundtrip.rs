#![no_main]

use libfuzzer_sys::fuzz_target;
use ocpp_rs::v21::parse::{deserialize_to_message, serialize_message};

fuzz_target!(|data: &[u8]| {
    let Ok(s) = core::str::from_utf8(data) else {
        return;
    };
    let Ok(msg) = deserialize_to_message(s) else {
        return;
    };
    let Ok(wire) = serialize_message(&msg) else {
        return;
    };
    let Ok(again) = deserialize_to_message(&wire) else {
        panic!("reparse failed after successful parse: {wire}");
    };
    let Ok(wire2) = serialize_message(&again) else {
        return;
    };
    assert_eq!(wire, wire2, "serialize not idempotent after reparse");
});
