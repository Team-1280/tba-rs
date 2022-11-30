
use awc::http::{Method, header::{IF_NONE_MATCH, ETAG}, StatusCode};
use moka::sync::Cache;

use serde::de::DeserializeOwned;

use crate::{Error, model::{team::{Team, SimpleTeam, TeamKey}, Year, event::{EventKey, TeamEventStatus, Event, EliminationAlliance, EventOPRs, EventDistrictPoints}, matches::{Match, MatchKey}}};
use std::{sync::Arc, collections::HashMap};
use async_trait::async_trait;

use super::Context;

const BASE_ENDPOINT: &str = "http://www.thebluealliance.com/api/v3/";

/// Trait implemented by all structures that represent endpoints of the TBA API with methods to
/// make requests using given parameters
#[async_trait(?Send)]
pub trait EndPoint: Sized {
    type Params;
    type Value;
    
    /// Make a request to the API endpoint represented by `self` with the given parameters
    async fn get(&self, params: Self::Params, ctx: &Context) -> Result<Self::Value, Error>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EndPointCacheEntry<T> {
    pub val: T,
    /// Version of this cached entry
    pub etag: String,
}

/// A collection of API endpoints that each cache requests made to them
#[derive(Default)]
pub struct EndPoints {
    pub teams: TeamsEndPoint,
    pub team: TeamEndPoint,
    pub event: EventEndPoint,
    pub matches: MatchEndPoint,
}

/// Structure representing requests made to the /teams endpoint
#[derive(Default)]
pub struct TeamsEndPoint {
    /// Representing the /teams/{page_num} endpoint
    pub(crate) full_page: TeamPageEP,
    /// Represents the /teams/{page_num}/simple endpoint
    pub(crate) simple_page: SimpleTeamPageEP,
    /// Represents /teams/{page_num}/keys
    pub(crate) key_page: KeysTeamPageEP,
    /// Represents /teams/{year}/{page_num}
    pub(crate) team_by_year: TeamPageByYearEP,
    /// Represents /teams/{year}/{page_num}/simple
    pub(crate) simple_team_by_year: SimpleTeamPageEP,
    /// Represents /teams/{year}/{page_num}/keys
    pub(crate) keys_by_year: KeysTeamPageByYearEP,
}

/// Container with all /team/ endpoints
#[derive(Default)]
pub struct TeamEndPoint {
    /// Represents /team/{team_key}
    pub(crate) team: TeamEP,
}

/// Container with all /event/ endpoints modelled
#[derive(Default)]
pub struct EventEndPoint {
    /// Represents the /event/{event_key} endpoint
    pub(crate) event: EventEP, 
    /// Represents the /event/{event_key}/simple endpoint
    pub(crate) simple: SimpleEventEP,
    /// Represents the /event/{event_key}/alliances endpoint
    pub(crate) alliances: EliminationAlliancesEP,
    /// Represents the /event/{event_key}/oprs endpoint
    pub(crate) oprs: EventOPRsEP,
    /// Represents the /event/{event_key}/district_points endpoint
    pub(crate) district_points: EventDistrictPointsEP,
    /// Represents the /event/{event_key}/teams/keys endpoint
    pub(crate) team_keys: EventTeamKeysEP,
    /// Represents the /event/{event_key}/teams/statuses endpoint
    pub(crate) team_statuses: EventTeamStatusesEP,
    /// Represents the /event/{event_key}/matches
    pub(crate) matches: EventMatchesEP,
    /// Represents the /event/{event_key}/matches/keys
    pub(crate) match_keys: EventMatchKeysEP,
}

#[derive(Default)]
pub struct MatchEndPoint {
    /// Represents the /match/{match_key} endpoint
    pub(crate) matches: MatchEP,
}

macro_rules! endpoint {
    ($name:ident: ($($params:ty),+) => $val:ty where ($($names:ident),+) $path:literal) => {
        pub struct $name { cache: Cache<($($params),+,), EndPointCacheEntry<$val>> }
        #[async_trait(?Send)]
        impl self::EndPoint for $name {
            type Params = ($($params),+,);
            type Value = $val;
            async fn get(&self, params: ($($params),+,), ctx: &Context) -> ::std::result::Result<Self::Value, Error> {
                let ($(ref $names),+,) = params;
                let path = ::std::format_args!($path);
                get_ep::<Self>(
                    path,
                    params,
                    &self.cache,
                    ctx
                ).await
            }
        }

        impl ::std::default::Default for $name {
            fn default() -> Self {
                Self { cache: Cache::new(10_000 / ::std::mem::size_of::<($($params),+,)>() as u64) }
            }
        }
    };
}

endpoint!{TeamPageEP: (usize) => Arc<Vec<Team>> where (page_num) "teams/{page_num}"}
endpoint!{SimpleTeamPageEP: (usize) => Arc<Vec<SimpleTeam>> where (page_num) "teams/{page_num}/simple"}
endpoint!{KeysTeamPageEP: (usize) => Arc<Vec<TeamKey>> where (page_num) "teams/{page_num}/keys"}
endpoint!{TeamPageByYearEP: (Year, usize) => Arc<Vec<Team>> where (year, page_num) "teams/{year}/{page_num}"}
endpoint!{SimpleTeamPageByYearEP: (Year, usize) => Arc<Vec<SimpleTeam>> where (year, page_num) "teams/{year}/{page_num}/simple"}
endpoint!{KeysTeamPageByYearEP: (Year, usize) => Arc<Vec<TeamKey>> where (year, page_num) "teams/{year}/{page_num}/keys"}
endpoint!{
    EventStatusByYearEP: (TeamKey, Year) => Arc<HashMap<EventKey, TeamEventStatus>>
    where (team_key, year) "team/{team_key}/events/{year}/statuses"
}
endpoint!{
    TeamEP: (TeamKey) => Arc<Team>
    where (team_key) "team/{team_key}"
}

endpoint!{EventEP: (EventKey) => Arc<Event> where (event_key) "event/{event_key}"}
endpoint!{SimpleEventEP: (EventKey) => Arc<Event> where (event_key) "event/{event_key}/simple"}
endpoint!{EliminationAlliancesEP: (EventKey) => Arc<Vec<EliminationAlliance>> where (event_key) "event/{event_key}/alliances"}
endpoint!{EventOPRsEP: (EventKey) => Arc<EventOPRs> where (event_key) "event/{event_key}/oprs"}
endpoint!{EventDistrictPointsEP: (EventKey) => Arc<EventDistrictPoints> where (event_key) "event/{event_key}/district_points"}
endpoint!{EventTeamKeysEP: (EventKey) => Arc<Vec<TeamKey>> where (event_key) "event/{event_key}/teams/keys"}
endpoint!{EventTeamStatusesEP: (EventKey) => Arc<HashMap<EventKey, TeamEventStatus>> where (event_key) "event/{event_key}/teams/statuses"}
endpoint!{EventMatchesEP: (EventKey) => Arc<Vec<Match>> where (event_key) "event/{event_key}/matches"}
endpoint!{EventMatchKeysEP: (EventKey) => Arc<Vec<MatchKey>> where (event_key) "event/{event_key}/matches/keys"}

endpoint!{MatchEP: (MatchKey) => Arc<Match> where (match_key) "match/{match_key}"}


/// Get the given path from the given endpoint, utilizing the cache
async fn get_ep<T: EndPoint + 'static>(
    path: std::fmt::Arguments<'_>,
    params: T::Params,
    cache: &Cache<T::Params, EndPointCacheEntry<T::Value>>,
    ctx: &Context,
) -> Result<T::Value, Error> 
where 
    T::Params: std::hash::Hash + std::cmp::Eq + Send + Sync,
    T::Value: Clone + Send + Sync + DeserializeOwned {
    let cached = cache.get(&params);
    let mut request = ctx
        .client
        .request(
            Method::GET,
            format!("{}{}", BASE_ENDPOINT, path),
        );

    if let Some(ref cached) = cached {
        request = request.insert_header((IF_NONE_MATCH, cached.etag.clone()));
    }
    
    let mut response = request.send().await?;
    match (response.status(), cached) {
        (StatusCode::NOT_MODIFIED, Some(cached)) => Ok(cached.val),
        (code, _) if code.is_success() => {
            let etag = response
                .headers()
                .get(ETAG)
                .map(|v| v
                    .to_str()
                    .map(str::to_owned)
                );
            let val = response.json::<T::Value>().await?;
            if let Some(etag) = etag {
                cache.insert(
                    params,
                    EndPointCacheEntry {
                        val: val.clone(),
                        etag: etag?
                    }
                );
            }

            Ok(val)
        },
        (code, _) => Err(Error::BadResponse(code)),
    }
}
