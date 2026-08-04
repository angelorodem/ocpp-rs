//! Version gating helpers.

use ocpp_rs::v21::version::{
    NegotiatedVersion, allows_action, allows_message_type, is_ocpp21_only_action,
};

#[test]
fn subprotocol_parse() {
    assert_eq!(
        NegotiatedVersion::from_subprotocol("ocpp2.1"),
        Some(NegotiatedVersion::Ocpp21)
    );
    assert_eq!(
        NegotiatedVersion::from_subprotocol("ocpp2.0.1"),
        Some(NegotiatedVersion::Ocpp201)
    );
    assert_eq!(NegotiatedVersion::from_subprotocol("ocpp1.6"), None);
}

#[test]
fn message_types_and_actions() {
    use NegotiatedVersion::{Ocpp21, Ocpp201};
    assert!(allows_message_type(Ocpp201, 2));
    assert!(!allows_message_type(Ocpp201, 5));
    assert!(!allows_message_type(Ocpp201, 6));
    assert!(allows_message_type(Ocpp21, 6));
    assert!(is_ocpp21_only_action("SetDERControl"));
    assert!(!allows_action(Ocpp201, "SetDERControl"));
    assert!(allows_action(Ocpp201, "BootNotification"));
    assert!(allows_action(Ocpp21, "SetDERControl"));
}
