#![no_main]

use libfuzzer_sys::fuzz_target;
use ocpp_rs::v21::parse::deserialize_to_message;
use ocpp_rs_fuzz::CorruptInput;

fuzz_target!(|data: &[u8]| {
    let mut u = arbitrary::Unstructured::new(data);
    let Ok(input) = CorruptInput::arbitrary_v21(&mut u) else {
        return;
    };
    let Ok(s) = std::str::from_utf8(&input.bytes) else {
        return;
    };
    let _ = deserialize_to_message(s);
});
