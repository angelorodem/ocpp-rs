//! Version gating helpers.

use ocpp_rs::v21::version::{
    allows_action, allows_message_type, is_ocpp21_only_action, NegotiatedVersion,
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
    let v201 = NegotiatedVersion::Ocpp201;
    let v21 = NegotiatedVersion::Ocpp21;
    assert!(allows_message_type(v201, 2));
    assert!(!allows_message_type(v201, 5));
    assert!(!allows_message_type(v201, 6));
    assert!(allows_message_type(v21, 6));
    assert!(is_ocpp21_only_action("SetDERControl"));
    assert!(!allows_action(v201, "SetDERControl"));
    assert!(allows_action(v201, "BootNotification"));
    assert!(allows_action(v21, "SetDERControl"));
}
