//! OCPP 2.1 OCPP-J RPC framework error codes (CALLERROR / CALLRESULTERROR).
//!
//! `MessageTypeNotSupported` is deprecated: unknown message type numbers must be
//! silently ignored rather than answered with this code.

use core::fmt;
use serde::{Deserialize, Serialize};

/// Valid OCPP 2.1 RPC framework error codes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RpcErrorCode {
    FormatViolation,
    GenericError,
    InternalError,
    /// Deprecated: unknown message type numbers should be silently ignored.
    MessageTypeNotSupported,
    NotImplemented,
    NotSupported,
    OccurrenceConstraintViolation,
    PropertyConstraintViolation,
    ProtocolError,
    RpcFrameworkError,
    SecurityError,
    TypeConstraintViolation,
    /// Forward-compatible catch-all for unknown codes.
    #[serde(other)]
    Unknown,
}

impl RpcErrorCode {
    /// Stable wire string for this code (`Unknown` → `"Unknown"`).
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::FormatViolation => "FormatViolation",
            Self::GenericError => "GenericError",
            Self::InternalError => "InternalError",
            Self::MessageTypeNotSupported => "MessageTypeNotSupported",
            Self::NotImplemented => "NotImplemented",
            Self::NotSupported => "NotSupported",
            Self::OccurrenceConstraintViolation => "OccurrenceConstraintViolation",
            Self::PropertyConstraintViolation => "PropertyConstraintViolation",
            Self::ProtocolError => "ProtocolError",
            Self::RpcFrameworkError => "RpcFrameworkError",
            Self::SecurityError => "SecurityError",
            Self::TypeConstraintViolation => "TypeConstraintViolation",
            Self::Unknown => "Unknown",
        }
    }

    /// Whether this code is deprecated by OCPP 2.1 errata.
    #[must_use]
    pub const fn is_deprecated(&self) -> bool {
        matches!(self, Self::MessageTypeNotSupported)
    }
}

impl fmt::Display for RpcErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
