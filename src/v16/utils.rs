use super::data_types::DateTimeWrapper;
// New type pattern to implement Arbitrary for DateTime

// Serializer for serde that forces to be in the format of ISO8601
pub(crate) mod iso8601_date_time {
    use alloc::{format, string::String};
    use chrono::{DateTime, Utc, NaiveDateTime};
    use serde::{self, Deserialize, Serializer, Deserializer};
    use super::DateTimeWrapper;

    static FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

    pub fn serialize<S>(
        date: &DateTimeWrapper,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.inner().format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTimeWrapper, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(DateTimeWrapper::new(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc)))
    }
}

pub(crate) mod iso8601_date_time_optional {
    use alloc::{format, string::String};
    use chrono::{DateTime, Utc, NaiveDateTime};
    use serde::{self, Deserialize, Serializer, Deserializer};
    use super::DateTimeWrapper;

    static FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";
    
    #[allow(clippy::ref_option)]
    pub fn serialize<S>(
        date: &Option<DateTimeWrapper>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(date) => {
                let s = format!("{}", date.inner().format(FORMAT));
                serializer.serialize_str(&s)
            },
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<DateTimeWrapper>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<String> = Option::deserialize(deserializer)?;
        match opt {
            Some(s) => {
                let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
                Ok(Some(DateTimeWrapper::new(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))))
            },
            None => Ok(None),
        }       
    }
}