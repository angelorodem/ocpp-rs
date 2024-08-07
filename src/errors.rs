use core::{fmt::Display, num::ParseIntError};
use crate::alloc::string::ToString;
use alloc::string::{FromUtf8Error, String};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    SerdeJson(serde_json::Error),
    ParseInt(ParseIntError),
    Utf8(FromUtf8Error),
    InvalidMessageCallType,
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

#[derive(Debug)]
pub struct CallTypeMismatch {
    pub expected: i32,
    pub found: i32,
}