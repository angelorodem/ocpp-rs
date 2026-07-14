#![no_main]

use libfuzzer_sys::fuzz_target;
use ocpp_rs::v16::parse::{deserialize_to_message, serialize_message};
use ocpp_rs_fuzz::StructuredFrame;

fuzz_target!(|data: &[u8]| {
    let mut u = arbitrary::Unstructured::new(data);
    let Ok(frame) = StructuredFrame::arbitrary_v16(&mut u) else {
        return;
    };
    let Ok(msg) = deserialize_to_message(&frame.wire) else {
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
    let Ok(again2) = deserialize_to_message(&wire2) else {
        panic!("reparse failed for canonical wire: {wire2}");
    };
    let Ok(wire3) = serialize_message(&again2) else {
        return;
    };
    assert_eq!(wire2, wire3, "canonical serialize not idempotent");
});
