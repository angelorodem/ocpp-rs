//! OCPP-J SEND (message type 6) — OCPP 2.1 fire-and-forget.

use alloc::string::{String, ToString};
use serde::de::{self, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_tuple::Serialize_tuple;
use strum_macros::AsRefStr;

use crate::v21::messages::notify_periodic_event_stream::NotifyPeriodicEventStream;

/// SEND actions (currently only `NotifyPeriodicEventStream`).
#[derive(AsRefStr, Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum SendAction {
    NotifyPeriodicEventStream(NotifyPeriodicEventStream),
}

impl SendAction {
    #[must_use]
    pub const fn action_name(&self) -> &'static str {
        match self {
            Self::NotifyPeriodicEventStream(_) => "NotifyPeriodicEventStream",
        }
    }
}

#[derive(Debug, PartialEq, Serialize_tuple, Clone)]
pub struct Send {
    pub(super) message_id: i32,
    pub unique_id: String,
    pub(crate) action: String,
    pub payload: SendAction,
}

impl Send {
    #[must_use]
    pub fn new(unique_id: String, payload: SendAction) -> Self {
        Self {
            message_id: 6,
            unique_id,
            action: payload.action_name().to_string(),
            payload,
        }
    }
}

impl<'de> Deserialize<'de> for Send {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SendVisitor;

        impl<'de> Visitor<'de> for SendVisitor {
            type Value = Send;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("OCPP 2.1 SEND array [6, messageId, action, payload]")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let message_id: i32 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("missing message type"))?;
                if message_id != 6 {
                    return Err(de::Error::custom("SEND message type must be 6"));
                }
                let unique_id: String = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("missing messageId"))?;
                let action_name: String = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("missing action"))?;
                match action_name.as_str() {
                    "NotifyPeriodicEventStream" => {
                        let payload = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::custom("missing payload"))?;
                        Ok(Send {
                            message_id: 6,
                            unique_id,
                            action: action_name,
                            payload: SendAction::NotifyPeriodicEventStream(payload),
                        })
                    }
                    other => Err(de::Error::custom(alloc::format!(
                        "unknown OCPP 2.1 SEND action: {other}"
                    ))),
                }
            }
        }

        deserializer.deserialize_seq(SendVisitor)
    }
}
