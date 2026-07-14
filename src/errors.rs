use alloc::string::{FromUtf8Error, String, ToString};
use core::{fmt::Display, num::ParseIntError};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    SerdeJson(serde_json::Error),
    ParseInt(ParseIntError),
    Utf8(FromUtf8Error),
    InvalidMessageCallType,
    InvalidMessageCallTypeParsing,
    /// Message type number outside the supported OCPP-J range (or unknown).
    UnsupportedMessageType(u8),
    /// No pending CALL was registered for this CALLRESULT `messageId`.
    UnknownPendingMessageId(String),
    /// Action name string is not a known OCPP 1.6 or 2.1 CALL action.
    UnknownActionName(String),
    /// CallResult payload matched multiple response schemas and no single action was supplied.
    AmbiguousCallResult(String),
    CallTypeMismatch(CallTypeMismatch),
    Custom(String),
}

impl Error {
    pub fn custom(val: impl Display + ToString) -> Self {
        Self::Custom(val.to_string())
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Self::Custom(s.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::SerdeJson(e) => write!(f, "SerdeJson: {e}"),
            Self::ParseInt(e) => write!(f, "ParseInt: {e}"),
            Self::Utf8(e) => write!(f, "Utf8: {e}"),
            Self::InvalidMessageCallType => write!(f, "InvalidMessageCallType"),
            Self::InvalidMessageCallTypeParsing => write!(f, "InvalidMessageCallTypeParsing"),
            Self::UnsupportedMessageType(t) => write!(f, "UnsupportedMessageType: {t}"),
            Self::UnknownPendingMessageId(id) => {
                write!(f, "UnknownPendingMessageId: {id}")
            }
            Self::UnknownActionName(name) => write!(f, "UnknownActionName: {name}"),
            Self::AmbiguousCallResult(detail) => write!(f, "AmbiguousCallResult: {detail}"),
            Self::CallTypeMismatch(e) => write!(f, "CallTypeMismatch: {e:?}"),
            Self::Custom(e) => write!(f, "{e}"),
        }
    }
}

impl core::error::Error for Error {}

#[derive(Debug)]
pub struct CallTypeMismatch {
    pub expected: i32,
    pub found: i32,
}
