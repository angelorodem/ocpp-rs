# OCPP-RS

OCPP-RS is a Rust library for implementing the Open Charge Point Protocol (OCPP) in Rust.    
    
it currently supports OCPP 1.6.    

[Documentation](https://docs.rs/ocpp_rs/latest/ocpp_rs/)

- Full implementation of OCPP 1.6 Protocol
- Currently most feature complete implementation of OCPP 1.6 in rust
- Batteries included, check packet [parsing(to_message) serialization(from_message)](https://docs.rs/ocpp_rs/latest/ocpp_rs/v16/parse/index.html)
- Inspired by a [python ocpp library](https://github.com/mobilityhouse/ocpp)


## Usage
In Cargo.toml, add the following dependency:
```toml
[dependencies]
ocpp-rs = "0.1"
```

# Particularities
Since the original OCPP 1.6 protocol does not contain a type field for `CallResult`, when parsing `CallResult`lt, you need to handle
Special cases where JSON payloads are ambiguous, like empty objects like: ```{}```, these might get serialized as a `EmptyResponse` instead of the variant
you are waiting for like `GetConfiguration`.

Look at this file to see how to properly handle `CallResults`: [`example`](example/src/main.rs)

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
```