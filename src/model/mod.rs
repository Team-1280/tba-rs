use chrono::NaiveDate;
use serde::{Deserializer, Deserialize};

pub mod id;
pub mod team;
pub mod event;

pub type Year = u32;

pub fn deserialize_yyyymmdd<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NaiveDate, D::Error> {
    let str = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&str, "%Y-%m-%d")
        .map(serde::de::Error::custom)
}
