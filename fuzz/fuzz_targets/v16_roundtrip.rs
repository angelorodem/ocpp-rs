#![no_main]

use libfuzzer_sys::fuzz_target;
use ocpp_rs::v16::parse::{deserialize_to_message, serialize_message};

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
        return;
    };
    assert_eq!(msg, again);
});
