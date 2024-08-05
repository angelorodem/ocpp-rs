use core::{fmt::Display, num::ParseIntError};
use crate::alloc::string::ToString;
use alloc::string::{FromUtf8Error, String};
use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    SerdeJson(serde_json::Error),
    #[from]
    ParseInt(ParseIntError),
    #[from]
    Utf8(FromUtf8Error),
    #[from]
    InvalidMessageCallType,
    #[from]
    CallTypeMismatch(CallTypeMismatch),
    #[from]
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