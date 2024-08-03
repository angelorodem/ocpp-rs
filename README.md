# OCPP-RS
Idiomatic implementation of OCPP 1.6 protocol

[Documentation](https://docs.rs/ocpp_rs/latest/ocpp_rs/)

- Full implementation of OCPP 1.6 Protocol
- Includes packet [parsing](https://docs.rs/ocpp_rs/latest/ocpp_rs/v16/parse/index.html)
- Fuzzed tested (please read the comment on the call_result fuzzing)
- Inspired by a [python ocpp library](https://github.com/mobilityhouse/ocpp)

## Example
Receiving a payload from a client:
```rust
use ocpp_rs::v16::parse::{self, Message};
use ocpp_rs::v16::call::{Action, Call};

// Example incoming message
let incoming_text = "[2, \"19223201\", \"BootNotification\", { \"chargePointVendor\": \"VendorX\", \"chargePointModel\": \"SingleSocketCharger\" }]";
let incoming_message = parse::to_message(incoming_text);
if let Ok(Message::Call(call)) = incoming_message {
    match call.payload {
        Action::BootNotification(boot_notification) => {
           // Do something with boot_notification
        },
        _ => {
          // Handle other actions
        }
   }
}
```
//!
Sending a payload to a client:
```rust
use ocpp_rs::v16::call::StartTransaction;
use ocpp_rs::v16::call_result::{self, CallResult, ResultPayload};
use ocpp_rs::v16::data_types::IdTagInfo;
use ocpp_rs::v16::enums::ChargePointStatus;
use ocpp_rs::v16::parse::Message;

let response = Message::CallResult(CallResult::new(
    "1234".to_string(),
    ResultPayload::StartTransaction(call_result::StartTransaction {
        transaction_id: 0,
        id_tag_info: IdTagInfo {
            status: ocpp_rs::v16::enums::GenericStatus::Accepted,
            expiry_date: None,
            parent_id_tag: None,
        },
    }),
));
//!```