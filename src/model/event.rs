use crate::{ctx::Context, Error};

use super::{id::{Key, KeyReferenced}, Year};
use async_trait::async_trait;
use chrono::{Date, NaiveDate};
use reqwest::Url;
use serde::Deserialize;
use serde_repr::Deserialize_repr;

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

pub enum PlayoffType {

}

#[derive(Clone, Debug, Deserialize)]
pub struct WebCast {
    #[serde(rename="type")]
    pub type_: WebCastType,
    pub channel: String,
    #[serde(deserialize_with="super::deserialize_yyyymmdd")]
    pub date: Option<NaiveDate>,
    pub file: Option<String>,
    pub division_keys: Option<Vec<EventKey>>,
    pub parent_event_key: Option<EventKey>,
    pub playoff_type: Option<PlayoffType>,
    pub playoff_type_string: Option<String>,
}

pub struct TeamEventStatus {

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

#[async_trait]
impl KeyReferenced for Event {
    async fn dereference(key: Key<Self>, ctx: &Context) -> Result<Self, Error> {
         
    }
}
