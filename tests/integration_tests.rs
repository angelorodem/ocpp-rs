use ocpp_rs::v16::call::{Action, BootNotification, Call};
use ocpp_rs::v16::parse::{to_message, Message};

#[test]
fn test_parse_boot_notification() {
    let data = "[2, \"19223201\", \"BootNotification\", { \"chargePointVendor\": \"VendorX\", \"chargePointModel\": \"SingleSocketCharger\" }]";
    let message = to_message(data).unwrap();
    println!("\nParsed: {:?}\n", message);

    let message_eq: Message = Message::Call(Call::new(
        "19223201".to_string(),
        "BootNotification".to_string(),
        Action::BootNotification(BootNotification {
            charge_point_vendor: "VendorX".to_string(),
            charge_point_model: "SingleSocketCharger".to_string(),
            ..Default::default()
        }),
    ));

    assert_eq!(message, message_eq);
    match message {
        ocpp_rs::v16::parse::Message::Call(call) => {
            assert_eq!(call.unique_id, "19223201");
            assert_eq!(call.action, "BootNotification");
        }
        _ => panic!("Unexpected message type"),
    }
}
