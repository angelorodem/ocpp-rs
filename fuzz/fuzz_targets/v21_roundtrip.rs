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
    // Pathological f64 JSON may shift once under serde_json/ryu; the form after
    // one parse→serialize cycle must then be a fixed point. Avoid PartialEq on
    // typed f64 / Value for the same reason.
    let Ok(wire2) = serialize_message(&again) else {
        return;
    };
    let Ok(again2) = deserialize_to_message(&wire2) else {
        panic!("reparse failed for canonical wire: {wire2}");
    };
    let Ok(wire3) = serialize_message(&again2) else {
        return;
    };
    assert_eq!(wire2, wire3, "canonical serialize not idempotent");
});
