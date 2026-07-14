use std::path::PathBuf;
#[test]
fn v16_seeds_do_not_panic() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fuzz/corpus/v16_deserialize");
    let mut parsed = 0usize;
    for entry in std::fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let s = std::fs::read_to_string(&path).unwrap();
        let _ = ocpp_rs::v16::parse::deserialize_to_message(s.trim());
        parsed += 1;
    }
    assert_eq!(parsed, 59);
}
#[test]
fn v21_seeds_do_not_panic() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fuzz/corpus/v21_deserialize");
    let mut parsed = 0usize;
    let mut calls = 0usize;
    for entry in std::fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let s = std::fs::read_to_string(&path).unwrap();
        if let Ok(msg) = ocpp_rs::v21::parse::deserialize_to_message(s.trim()) {
            use ocpp_rs::v21::parse::Message;
            if matches!(msg, Message::Call(_) | Message::Send(_)) {
                calls += 1;
            }
        }
        parsed += 1;
    }
    assert_eq!(parsed, 115);
    assert!(calls >= 91, "expected >=91 call/send parses, got {calls}");
}
