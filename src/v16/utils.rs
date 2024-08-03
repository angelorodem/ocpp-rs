use arbitrary::{self, Arbitrary};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
// New type pattern to implement Arbitrary for DateTime
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone, Default, Copy)]
pub struct DateTimeWrapper(pub DateTime<Utc>);

impl Arbitrary<'_> for DateTimeWrapper {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        
        Ok(Self(DateTime::parse_from_rfc3339("2024-06-01T19:52:45Z").unwrap().with_timezone(&chrono::Utc)))
    }
}

// impl<T> Arbitrary<'_> for Vec<T> {
//  fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
//         let len = u.int_in_range(0..100)?;
//         let mut vec = Vec::with_capacity(len);
//         for _ in 0..len {
//             vec.push(T::arbitrary(u)?);
//         }
//         Ok(vec)
//  }   
// }


// Serializer for serde that forces to be in the format of ISO8601
pub(crate) mod iso8601_date_time {
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
        let s = format!("{}", date.0.format(FORMAT));
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
        Ok(DateTimeWrapper(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc)))
    }
}

pub(crate) mod iso8601_date_time_optional {
    use chrono::{DateTime, Utc, NaiveDateTime};
    use serde::{self, Deserialize, Serializer, Deserializer};
    use super::DateTimeWrapper;

    static FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

    pub fn serialize<S>(
        date: &Option<DateTimeWrapper>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(date) => {
                let s = format!("{}", date.0.format(FORMAT));
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
                Ok(Some(DateTimeWrapper(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))))
            },
            None => Ok(None),
        }       
    }
}