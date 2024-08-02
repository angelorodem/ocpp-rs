#[macro_use]
extern crate afl;
extern crate ocpp_rs;

use ocpp_rs::v16::call::*;
use ocpp_rs::v16::data_types::*;
use ocpp_rs::v16::enums::*;
use ocpp_rs::v16::parse::{to_message, Message};

fn main() {
    fuzz!(|data: Call| {
        // println!("\nincoming: {:?}", data);
        let unpacked = serde_json::to_string(&data).unwrap();
        // println!("serialized: {:?}", unpacked);
        let packed = serde_json::from_str::<Call>(&unpacked);
        // println!("deserialized: {:?}\n", packed);
        assert_eq!(data, packed.unwrap());    
    });
}