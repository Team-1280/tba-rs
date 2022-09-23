use crate::{ctx::Context, Error};

use super::{id::{Key, KeyReferenced}, Year};
use async_trait::async_trait;
use chrono::{Date, NaiveDate};
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

pub struct Event {
    simple: SimpleEvent,
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
