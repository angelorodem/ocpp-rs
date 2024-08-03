use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    SerdeJson(serde_json::Error),
    #[from]
    ParseInt(std::num::ParseIntError),
    #[from]
    Utf8(std::string::FromUtf8Error),
    #[from]
    InvalidMessageCallType,
    #[from]
    CallTypeMismatch(CallTypeMismatch),
    #[from]
    Custom(String),    
}

impl Error {
    pub fn custom(val: impl std::fmt::Display) -> Self {
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