use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::{ctx::endpoints::EndPoint, key};

use super::{event::{PlayoffLevel, EventKey}, team::TeamKey};


key!(MatchKey(String) -> Match => (self, ctx) with ctx
            .endpoints
            .matches
            .matches
            .get((self,), ctx)
            .await
);

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
pub enum MatchWinner {
    Red,
    Blue,
    Tie
}

#[derive(Debug,Clone,Deserialize)]
pub struct Match {
    pub key: MatchKey,
    pub comp_level: PlayoffLevel,
    pub set_number: u32,
    pub match_number: u32,
    pub alliances: MatchAlliances,
    pub winning_alliance: MatchWinner,
    pub event_key: EventKey,
    #[serde(deserialize_with="super::deserialize_ts")]
    pub time: NaiveDateTime,
    #[serde(deserialize_with="super::deserialize_ts")]
    pub actual_time: NaiveDateTime,
    #[serde(deserialize_with="super::deserialize_ts")]
    pub predicted_time: NaiveDateTime,
    #[serde(deserialize_with="super::deserialize_ts")]
    pub post_result_time: NaiveDateTime,
    pub videos: Vec<MatchVideo>,
}

#[derive(Debug,Clone,Copy,)]
pub enum MatchVideoType {
    Youtube,
    TBA,
}

#[derive(Debug,Clone,Deserialize)]
pub struct MatchVideo {
    #[serde(rename="type")]
    pub type_: MatchVideoType,
    pub key: String,
}

#[derive(Clone,Debug,Deserialize)]
pub struct MatchAlliances {
    pub red: MatchAlliance,
    pub blue: MatchAlliance
}

#[derive(Clone,Debug,Deserialize)]
pub struct MatchAlliance {
    pub score: Option<i32>,
    pub team_keys: Vec<TeamKey>,
    pub surrogate_team_keys: Vec<TeamKey>,
    pub dq_team_keys: Vec<TeamKey>,
}


impl<'de> Deserialize<'de> for MatchWinner {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let str = <&str as Deserialize<'de>>::deserialize(deserializer)?;
        match str {
            "red" => Ok(Self::Red),
            "blue" => Ok(Self::Blue),
            "" => Ok(Self::Tie),
            _ => Err(serde::de::Error::custom(format!("Unknown match winner string: '{}'", str)))
        }
    }
}

impl<'de> Deserialize<'de> for MatchVideoType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let str = <&str as Deserialize<'de>>::deserialize(deserializer)?;
        match str {
            "youtube" => Ok(Self::Youtube),
            "tba" => Ok(Self::TBA),
            _ => Err(serde::de::Error::custom(format!("Unknown match video type '{}'", str)))
        }
    }
}
