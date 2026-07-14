#![no_main]

use libfuzzer_sys::fuzz_target;
use ocpp_rs_fuzz::CorruptInput;
use ocpp_rs::v16::parse::deserialize_to_message;

fuzz_target!(|input: CorruptInput| {
    let Ok(s) = std::str::from_utf8(&input.bytes) else {
        // Invalid UTF-8 is a valid fuzz case for the outer entrypoint.
        return;
    };
    let _ = deserialize_to_message(s);
});
