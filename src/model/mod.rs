use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserializer, Deserialize};

pub mod id;
pub mod team;
pub mod event;
pub mod matches;

pub type Year = u32;

pub fn deserialize_yyyymmdd_opt<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<NaiveDate>, D::Error> {
    let str = Option::<&str>::deserialize(deserializer)?;
    str
        .map(|str| NaiveDate::parse_from_str(&str, "%Y-%m-%d")
            .map_err(serde::de::Error::custom)
        )
        .transpose()
}


pub fn deserialize_yyyymmdd<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NaiveDate, D::Error> {
    let str = <&str as Deserialize>::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&str, "%Y-%m-%d")
        .map_err(serde::de::Error::custom)
}

pub fn deserialize_ts<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NaiveDateTime, D::Error> {
    let n = <i64 as Deserialize>::deserialize(deserializer)?;
    Ok(NaiveDateTime::from_timestamp(n, 0))
}
