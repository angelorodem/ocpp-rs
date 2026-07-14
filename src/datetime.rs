//! Shared datetime wire helpers for OCPP 1.6 and 2.1.
//!
//! **Parse** always uses RFC3339 (accepts `%.3fZ`, offsets, variable fractional seconds, etc.).
//! **Serialize** defaults to strict `%Y-%m-%dT%H:%M:%S%.3fZ`; enable
//! `datetime_serialize_rfc3339` to emit `to_rfc3339_opts(Millis, true)` instead.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// UTC datetime wrapper used by OCPP payload fields.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone, Default, Copy)]
pub struct DateTimeWrapper(DateTime<Utc>);

impl DateTimeWrapper {
    #[must_use]
    pub const fn new(dt: DateTime<Utc>) -> Self {
        Self(dt)
    }

    #[must_use]
    pub const fn inner(&self) -> DateTime<Utc> {
        self.0
    }
}

const STRICT_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

fn serialize_utc<S>(dt: DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    #[cfg(feature = "datetime_serialize_rfc3339")]
    {
        use chrono::SecondsFormat;
        let s = dt.to_rfc3339_opts(SecondsFormat::Millis, true);
        serializer.serialize_str(&s)
    }
    #[cfg(not(feature = "datetime_serialize_rfc3339"))]
    {
        use alloc::format;
        let s = format!("{}", dt.format(STRICT_FORMAT));
        serializer.serialize_str(&s)
    }
}

fn parse_utc(s: &str) -> Result<DateTime<Utc>, alloc::string::String> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| alloc::format!("{e}"))
}

/// Serde helpers for required [`DateTimeWrapper`] fields.
pub mod date_time {
    use super::{DateTimeWrapper, parse_utc, serialize_utc};
    use alloc::string::String;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &DateTimeWrapper, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_utc(date.inner(), serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTimeWrapper, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        parse_utc(&s)
            .map(DateTimeWrapper::new)
            .map_err(serde::de::Error::custom)
    }
}

/// Serde helpers for optional [`DateTimeWrapper`] fields.
pub mod date_time_optional {
    use super::{DateTimeWrapper, parse_utc, serialize_utc};
    use alloc::string::String;
    use serde::{self, Deserialize, Deserializer, Serializer};

    #[allow(clippy::ref_option)]
    pub fn serialize<S>(date: &Option<DateTimeWrapper>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(date) => serialize_utc(date.inner(), serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTimeWrapper>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<String> = Option::deserialize(deserializer)?;
        match opt {
            Some(s) => parse_utc(&s)
                .map(|dt| Some(DateTimeWrapper::new(dt)))
                .map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}
