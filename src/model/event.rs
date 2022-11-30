use std::{sync::Arc, collections::HashMap};

use crate::{ctx::{Context, endpoints::EndPoint}, Error};

use super::{id::{Key, KeyReferenced}, Year, team::TeamKey};
use async_trait::async_trait;
use chrono::NaiveDate;
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use url::Url;

pub type EventKey = Key<Event>;

#[derive(Clone, Copy, Deserialize_repr, Debug)]
#[repr(i8)]
pub enum EventType {
    Regional = 0,
    District = 1,
    DistrictCmp = 2,
    CmpDivision = 3,
    CmpFinals = 4,
    DistrictCmpDivision = 5,
    FOC = 6,
    Remote = 7,

    Offseason = 99,
    Preseason = 100,
    Unlabeled = -1,
}

#[derive(Clone, Debug, Deserialize,)]
pub struct SimpleEvent {
    pub key: EventKey,
    pub name: String,
    pub event_code: String,
    pub event_type: EventType,
    pub district: Option<DistrictList>,
    pub city: Option<String>,
    pub state_prov: Option<String>,
    pub country: Option<String>,
    #[serde(deserialize_with="super::deserialize_yyyymmdd")]
    pub start_date: NaiveDate,
    #[serde(deserialize_with="super::deserialize_yyyymmdd")]
    pub end_date: NaiveDate,
    pub year: Year,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DistrictList {
    pub abbreviation: String,
    pub display_name: String,
    pub key: String,
    pub year: Year,
}

#[derive(Clone, Debug, Deserialize,)]
pub struct Event {
    #[serde(flatten)]
    pub simple: SimpleEvent,
    pub short_name: Option<String>,
    pub event_type_string: String,
    pub week: Option<u32>,
    pub address: Option<String>,
    pub postal_code: Option<String>,
    pub gmaps_place_id: Option<String>,
    pub gmaps_url: Option<Url>,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub location_name: Option<String>,
    pub timezone: Option<String>,
    pub website: Option<Url>,
    pub first_event_id: Option<String>,
    pub first_event_code: Option<String>,
    pub webcasts: Vec<WebCast>,
    pub division_keys: Vec<EventKey>,
    pub parent_event_key: String,
    pub playoff_type: Option<PlayoffType>,
    pub playoff_type_string: Option<String>,

}

#[derive(Clone, Debug)]
pub enum WebCastType {
    Youtube,
    Twitch,
    Ustream,
    Iframe,
    Html5,
    Rtmp,
    Livestream,
    DirectLink,
    Mms,
    Justin,
    Stemtv,
    Dacast,
}

#[derive(Clone, Debug, Deserialize_repr)]
#[repr(u8)]
pub enum PlayoffType {
    Bracket8Team = 0,
    Bracket16Team = 1,
    Bracket4Team = 2,
    AvgScore8Team = 3,
    RoundRobin6Team = 4,
    DoubleElim8Team = 5,
    Bo5Finals = 6,
    Bo3Finals = 7,
    Custom = 8,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WebCast {
    #[serde(rename="type")]
    pub type_: WebCastType,
    pub channel: String,
    #[serde(deserialize_with="super::deserialize_yyyymmdd_opt")]
    pub date: Option<NaiveDate>,
    pub file: Option<String>,
    pub division_keys: Option<Vec<EventKey>>,
    pub parent_event_key: Option<EventKey>,
    pub playoff_type: Option<PlayoffType>,
    pub playoff_type_string: Option<String>,
}

#[derive(Clone,Debug,Deserialize)]
pub struct TeamEventStatusAlliance {
    pub name: Option<String>,
    pub number: u16,
    pub backup: Option<TeamEventStatusAllianceBackup>,
    pub pick: u8,
}

#[derive(Clone,Debug,Deserialize)]
pub struct TeamEventStatusAllianceBackup {
    pub out: Option<TeamKey>,
    #[serde(rename="in")]
    pub in_: Option<TeamKey>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayoffLevel {
    QM,
    EF,
    QF,
    SF,
    F,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayoffStatus {
    Won,
    Eliminated,
    Playing,
}

#[derive(Clone,Debug,Deserialize)]
pub struct TeamEventStatusPlayoff {
    pub level: PlayoffLevel,
    pub current_level_record: WLTRecord,
    pub record: WLTRecord,
    pub status: String,
}

#[derive(Clone,Debug,Deserialize)]
pub struct TeamEventStatusRank {
    pub num_teams: Option<u16>,
    pub ranking: Option<TeamEventStatusRankRanking>,
    pub sort_order_info: Option<Vec<TeamEventStatusRankSortOrderInfo>>,
    pub status: Option<String>,
}

#[derive(Clone,Debug,Deserialize)]
pub struct TeamEventStatusRankSortOrderInfo {
    pub precision: Option<u16>,
    pub name: Option<String>,
}

#[derive(Clone,Debug,Deserialize)]
pub struct TeamEventStatusRankRanking {
    pub matches_played: Option<u16>,
    pub qual_average: Option<f64>,
    pub sort_orders: Option<Vec<f64>>,
    pub record: Option<WLTRecord>,
    pub rank: Option<u32>,
    pub dq: Option<u32>,
    pub team_key: Option<TeamKey>,
}

/// A Win-Loss-Tie record for a team or alliance
#[derive(Clone,Debug,Deserialize)]
pub struct WLTRecord {
    pub losses: Option<u16>,
    pub wins: Option<u16>,
    pub ties: Option<u16>,
}

#[derive(Clone,Debug,Deserialize)]
pub struct TeamEventStatus {
    pub qual: TeamEventStatusRank,
    pub alliance: TeamEventStatusAlliance,
}

#[derive(Clone,Debug,Deserialize)]
pub struct EliminationAlliance {
    pub name: Option<String>,
    pub backup: Option<TeamEventStatusAllianceBackup>,
    pub declined: Vec<TeamKey>,
    pub picks: Vec<TeamKey>,
    pub status: EliminationAllianceStatus,
}

#[derive(Clone,Debug,Deserialize)]
pub struct EventOPRs {
    pub oprs: HashMap<TeamKey, f64>,
    pub dprs: HashMap<TeamKey, f64>,
    pub ccwms: HashMap<TeamKey, f64>,
}

#[derive(Clone,Debug,Deserialize)]
pub struct EliminationAllianceStatus {
    pub playoff_average: f64,
    #[serde(flatten)]
    pub team_event: TeamEventStatusPlayoff,
}

#[derive(Clone,Debug,Deserialize)]
pub struct EventDistrictPoints {
    pub points: HashMap<TeamKey, EventDistrictPoints>,
    pub tiebreakers: HashMap<TeamKey, EventDistrictPointsTiebreaker>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct EventDistrictPointsPoints {
    pub total: i32,
    pub alliance_points: i32,
    pub elim_points: i32,
    pub award_points: i32,
    pub qual_points: i32,
}

#[derive(Clone,Debug,Deserialize)]
pub struct EventDistrictPointsTiebreaker {
    pub highest_qual_scores: Vec<i32>,
    pub qual_wins: u32,
}

impl AsRef<SimpleEvent> for Event {
    fn as_ref(&self) -> &SimpleEvent {
        &self.simple
    }
}
impl AsMut<SimpleEvent> for Event {
    fn as_mut(&mut self) -> &mut SimpleEvent {
        &mut self.simple
    }
}

#[async_trait(?Send)]
impl KeyReferenced for Event {
    async fn dereference(key: Key<Self>, ctx: &Context) -> Result<Arc<Self>, Error> {
        ctx
            .endpoints
            .event
            .event
            .get((key,), ctx)
            .await
    }
}

impl<'de> Deserialize<'de> for WebCastType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let str = <&str as Deserialize>::deserialize(deserializer)?;
        Ok(match str {
            "youtube" => Self::Youtube,
            "twitch" => Self::Twitch,
            "ustream" => Self::Ustream,
            "iframe" => Self::Iframe,
            "html5" => Self::Html5,
            "rtmp" => Self::Rtmp,
            "livestream" => Self::Livestream,
            "direct_link" => Self::DirectLink,
            "mms" => Self::Mms,
            "justin" => Self::Justin,
            "stemtv" => Self::Stemtv,
            "dacast" => Self::Dacast,
            _ => return Err(serde::de::Error::custom(format!("Unknown webcast type '{}'", str)))
        })
    }
}
impl<'de> Deserialize<'de> for PlayoffLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let str = <&str as Deserialize>::deserialize(deserializer)?;
        Ok(match str {
            "qm" => Self::QM,
            "ef" => Self::EF,
            "qf" => Self::QF,
            "sf" => Self::SF,
            "f" => Self::F,
            _ => return Err(serde::de::Error::custom(format!("Unknown playoff level '{}'", str)))
        })
    }
}
impl<'de> Deserialize<'de> for PlayoffStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let str = <&str as Deserialize>::deserialize(deserializer)?;
        Ok(match str {
            "won" => Self::Won,
            "eliminated" => Self::Eliminated,
            "playing" => Self::Playing,
            _ => return Err(serde::de::Error::custom(format!("Unknown ")))
        })
    }
}
