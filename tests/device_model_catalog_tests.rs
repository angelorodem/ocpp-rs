//! Device-model catalog smoke tests (feature `device_model_catalog`).

#![cfg(feature = "device_model_catalog")]

use ocpp_rs::v21::device_model::{
    STANDARD_COMPONENTS, STANDARD_VARIABLES, is_standard_component, is_standard_variable,
    rows_for_component,
};

#[test]
fn catalogs_are_non_empty() {
    assert!(STANDARD_COMPONENTS.len() > 50);
    assert!(STANDARD_VARIABLES.len() > 100);
    assert!(is_standard_component("ChargingStation"));
    assert!(is_standard_component("chargingstation"));
    assert!(is_standard_variable("HeartbeatInterval"));
}

#[test]
fn matrix_lookup() {
    let rows: Vec<_> = rows_for_component("AuthCtrlr").collect();
    assert!(!rows.is_empty());
}
