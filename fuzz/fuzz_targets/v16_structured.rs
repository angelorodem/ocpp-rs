#![no_main]

use libfuzzer_sys::fuzz_target;
use ocpp_rs_fuzz::StructuredFrame;
use ocpp_rs::v16::parse::{deserialize_to_message, serialize_message};

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
    assert_eq!(wire, wire2, "serialize not idempotent after reparse");
});
