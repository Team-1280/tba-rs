use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserializer, Deserialize};

pub mod id;
pub mod team;
pub mod event;
pub mod matches;

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug,Deserialize)]
#[serde(transparent)]
pub struct Year(u16);

/// Error returned from [Year::new] if the given value was not 4 digits
#[derive(Clone, Copy,Debug)]
pub struct Not4Digits;

impl Year {
    /// Create a new year from a 4-digit number
    pub fn new(year: u16) -> Result<Self, Not4Digits> {
        match std::iter::successors(Some(year), |y| (*y >= 10).then(|| y / 10)).count() {
            4 => Ok(Self(year)),
            _ => Err(Not4Digits)
        }
    }
}

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

impl std::fmt::Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
