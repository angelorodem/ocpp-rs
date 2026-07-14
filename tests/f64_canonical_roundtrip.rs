//! Regression: pathological f64 JSON is not a fixed point of the *first*
//! parse→serialize, but the canonical form after one cycle must be stable.

#[test]
fn v21_canonical_serialize_stable_after_one_cycle() {
    let s = concat!(
        r#"[2,"t","BatterySwap",{"batteryData":[{"evseId":0,"serialNumber":"x","soC":"#,
        "444444444444444444444440.0",
        r#","soH":0.0}],"eventType":"BatteryIn","idToken":{"idToken":"x","type":"x"},"requestId":0}]"#,
    );
    let msg = ocpp_rs::v21::parse::deserialize_to_message(s).unwrap();
    let wire = ocpp_rs::v21::parse::serialize_message(&msg).unwrap();
    let again = ocpp_rs::v21::parse::deserialize_to_message(&wire).unwrap();
    let wire2 = ocpp_rs::v21::parse::serialize_message(&again).unwrap();
    let again2 = ocpp_rs::v21::parse::deserialize_to_message(&wire2).unwrap();
    let wire3 = ocpp_rs::v21::parse::serialize_message(&again2).unwrap();
    assert_eq!(wire2, wire3);
    // First emit may differ from canonical (serde_json/ryu); that is expected.
    let _ = wire;
}
