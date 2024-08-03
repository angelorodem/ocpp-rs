#![no_main]

use std::fmt::Debug;

use libfuzzer_sys::fuzz_target;
use ocpp_rs::v16::call_result::{self, *};
use ocpp_rs::v16::data_types::*;
use ocpp_rs::v16::enums::*;

// IMPORTANT: When deserializing data from JSON, optional fields might not be present,
// even when present and null, the deserializer will transform data with only ``status`` as
// ``GenericStatusResponse`` instead of this struct.
//
// This means, in case you are waiting for a response that matches this struct, you should
// also check if first you receive a ``GenericStatusResponse`` that matches the same ``unique_id`` you've sent.
//
// This is mostly due to the protocol not being properly projected, because Call does have the type field,
// but ``CallResult`` does not.
//
// This fuzzing test finds this issues, so we need to handle these cases.
fuzz_target!(|data: CallResult| {
    let unpacked = serde_json::to_string(&data).unwrap();
    let packed = serde_json::from_str::<CallResult>(&unpacked).unwrap();

    match &packed.payload {
        ResultPayload::PossibleEmptyResponse(response) => {
            if response.is_empty() {
                // is data empty too?
                return;
            } else {
                match response {
                    EmptyResponses::EmptyResponse(_) => {
                        panic!("this will never match if is_empty() is true");
                    },
                    EmptyResponses::PossibleIdTagInfoResponse(id_tag_response) => {
                        if let ResultPayload::PossibleEmptyResponse(v) = data.payload {
                            if let EmptyResponses::PossibleIdTagInfoResponse(v) = v {
                                assert_eq!(v.get_id_tag_info(), id_tag_response.clone().get_id_tag_info());
                                return;
                            } else {
                                panic!("Data does not match the response");
                            }
                        } else {
                            panic!("Data does not match the response");
                        }
                    },
                    _ => {
                        assert_eq!(data, packed);
                    }
                }
            }

        },
        ResultPayload::PossibleStatusResponse(response) => {
            if response.is_only_status() {
                if let ResultPayload::PossibleStatusResponse(v) = data.payload {
                    assert_eq!(v.get_status(), response.get_status());
                } else {
                    panic!("Data does not match the response");
                }
            } else {
                match response {
                    StatusResponses::StatusResponse(_) => {
                        panic!("this will never match if is_only_status() is true");
                    },
                    _ => {
                        assert_eq!(data, packed);
                    }                    
                }
            }

        },        
        _ => {
            assert_eq!(data, packed);
        }
    }

});

