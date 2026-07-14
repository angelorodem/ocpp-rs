#![no_main]

use libfuzzer_sys::fuzz_target;
use ocpp_rs_fuzz::CorruptInput;
use ocpp_rs::v21::parse::deserialize_to_message;

fuzz_target!(|input: CorruptInput| {
    let Ok(s) = std::str::from_utf8(&input.bytes) else {
        return;
    };
    let _ = deserialize_to_message(s);
});
