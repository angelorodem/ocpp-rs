#![no_main]

use libfuzzer_sys::fuzz_target;
use ocpp_rs::v16::parse::deserialize_to_message;

fuzz_target!(|data: &[u8]| {
    let Ok(s) = core::str::from_utf8(data) else {
        return;
    };
    let _ = deserialize_to_message(s);
});
