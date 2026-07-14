//! OCPP 1.6 OCPP-J RPC framework error codes (CALLERROR).
//!
//! Wire spellings are intentional, including historical misspellings
//! (`FormationViolation`, `OccurenceConstraintViolation`).

use core::fmt;
use serde::{Deserialize, Serialize};

/// Valid OCPP 1.6 CALLERROR `errorCode` values.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RpcErrorCode {
    NotImplemented,
    NotSupported,
    InternalError,
    ProtocolError,
    SecurityError,
    /// Wire: `FormationViolation` (not `FormatViolation`).
    FormationViolation,
    PropertyConstraintViolation,
    /// Wire: `OccurenceConstraintViolation` (single “r”).
    OccurenceConstraintViolation,
    TypeConstraintViolation,
    GenericError,
    /// Forward-compatible catch-all for unknown codes.
    #[serde(other)]
    Unknown,
}

impl RpcErrorCode {
    /// Stable wire string for this code (`Unknown` → `"Unknown"`).
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::NotImplemented => "NotImplemented",
            Self::NotSupported => "NotSupported",
            Self::InternalError => "InternalError",
            Self::ProtocolError => "ProtocolError",
            Self::SecurityError => "SecurityError",
            Self::FormationViolation => "FormationViolation",
            Self::PropertyConstraintViolation => "PropertyConstraintViolation",
            Self::OccurenceConstraintViolation => "OccurenceConstraintViolation",
            Self::TypeConstraintViolation => "TypeConstraintViolation",
            Self::GenericError => "GenericError",
            Self::Unknown => "Unknown",
        }
    }
}

impl fmt::Display for RpcErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
