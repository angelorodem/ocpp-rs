//! Schema validation + MessageId length tests.

use ocpp_rs::errors::Error;
use ocpp_rs::v21::parse::deserialize_to_message;
use ocpp_rs::validate::{check_message_id_len, MESSAGE_ID_MAX_LEN};

#[test]
fn message_id_helper() {
    assert!(check_message_id_len("a").is_ok());
    assert!(check_message_id_len(&"x".repeat(MESSAGE_ID_MAX_LEN)).is_ok());
    assert!(check_message_id_len(&"x".repeat(MESSAGE_ID_MAX_LEN + 1)).is_err());
}

#[test]
#[cfg(feature = "schema_validate")]
fn rejects_overlong_vendor_name() {
    let long = "V".repeat(256);
    let data = format!(
        r#"[2, "1", "BootNotification", {{"reason":"PowerUp","chargingStation":{{"model":"M","vendorName":"{long}"}}}}]"#
    );
    let err = deserialize_to_message(&data).unwrap_err();
    assert!(matches!(err, Error::ConstraintViolation(_)), "{err}");
}

#[test]
#[cfg(feature = "schema_validate")]
fn accepts_valid_boot() {
    let data = r#"[2, "1", "BootNotification", {"reason":"PowerUp","chargingStation":{"model":"M","vendorName":"VendorX"}}]"#;
    assert!(deserialize_to_message(data).is_ok());
}
