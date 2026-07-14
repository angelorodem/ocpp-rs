//! Schema constraint validation (feature `schema_validate`).
//!
//! Bounds come from OCPP JSON Schemas (`maxLength`, `minItems`/`maxItems`,
//! `minimum`/`maximum`). Shape/`additionalProperties` remain serde's job.

use alloc::string::String;
use core::fmt;

/// Maximum OCPP-J MessageId / UniqueId length.
pub const MESSAGE_ID_MAX_LEN: usize = 36;

/// Result of a constraint check.
pub type Result<T> = core::result::Result<T, ConstraintViolation>;

/// A schema constraint failure (`maxLength`, array size, numeric bounds, MessageId length).
#[derive(Debug, Clone, PartialEq)]
pub struct ConstraintViolation {
    pub path: String,
    pub kind: ConstraintKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintKind {
    MaxLength { max: usize, actual: usize },
    MinItems { min: usize, actual: usize },
    MaxItems { max: usize, actual: usize },
    Minimum { min: f64, actual: f64 },
    Maximum { max: f64, actual: f64 },
    MessageIdTooLong { actual: usize },
    UnknownAction,
    Custom(String),
}

impl ConstraintViolation {
    #[must_use]
    pub fn max_length(path: impl Into<String>, max: usize, actual: usize) -> Self {
        Self {
            path: path.into(),
            kind: ConstraintKind::MaxLength { max, actual },
        }
    }

    #[must_use]
    pub fn min_items(path: impl Into<String>, min: usize, actual: usize) -> Self {
        Self {
            path: path.into(),
            kind: ConstraintKind::MinItems { min, actual },
        }
    }

    #[must_use]
    pub fn max_items(path: impl Into<String>, max: usize, actual: usize) -> Self {
        Self {
            path: path.into(),
            kind: ConstraintKind::MaxItems { max, actual },
        }
    }

    #[must_use]
    pub fn minimum(path: impl Into<String>, min: f64, actual: f64) -> Self {
        Self {
            path: path.into(),
            kind: ConstraintKind::Minimum { min, actual },
        }
    }

    #[must_use]
    pub fn maximum(path: impl Into<String>, max: f64, actual: f64) -> Self {
        Self {
            path: path.into(),
            kind: ConstraintKind::Maximum { max, actual },
        }
    }

    #[must_use]
    pub fn message_id_too_long(actual: usize) -> Self {
        Self {
            path: "messageId".into(),
            kind: ConstraintKind::MessageIdTooLong { actual },
        }
    }

    #[must_use]
    pub fn unknown_action(action: &str) -> Self {
        Self {
            path: action.into(),
            kind: ConstraintKind::UnknownAction,
        }
    }
}

impl fmt::Display for ConstraintViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ConstraintKind::MaxLength { max, actual } => {
                write!(f, "maxLength at {}: max {max}, actual {actual}", self.path)
            }
            ConstraintKind::MinItems { min, actual } => {
                write!(f, "minItems at {}: min {min}, actual {actual}", self.path)
            }
            ConstraintKind::MaxItems { max, actual } => {
                write!(f, "maxItems at {}: max {max}, actual {actual}", self.path)
            }
            ConstraintKind::Minimum { min, actual } => {
                write!(f, "minimum at {}: min {min}, actual {actual}", self.path)
            }
            ConstraintKind::Maximum { max, actual } => {
                write!(f, "maximum at {}: max {max}, actual {actual}", self.path)
            }
            ConstraintKind::MessageIdTooLong { actual } => {
                write!(
                    f,
                    "messageId length {actual} exceeds max {MESSAGE_ID_MAX_LEN}"
                )
            }
            ConstraintKind::UnknownAction => {
                write!(f, "unknown action for schema validation: {}", self.path)
            }
            ConstraintKind::Custom(msg) => write!(f, "{msg}"),
        }
    }
}

/// Check OCPP-J MessageId / UniqueId length (always available).
///
/// # Errors
/// Returns [`ConstraintViolation`] when `id` exceeds [`MESSAGE_ID_MAX_LEN`].
pub fn check_message_id_len(id: &str) -> Result<()> {
    let len = id.chars().count();
    if len > MESSAGE_ID_MAX_LEN {
        return Err(ConstraintViolation::message_id_too_long(len));
    }
    Ok(())
}

/// Types that can validate themselves against schema constraints.
pub trait Validate {
    /// # Errors
    /// Returns the first constraint violation found.
    fn validate(&self) -> Result<()>;
}

impl Validate for () {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}

impl Validate for String {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}
