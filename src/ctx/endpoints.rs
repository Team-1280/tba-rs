
use moka::sync::Cache;
use once_cell::sync::Lazy;
use reqwest::{RequestBuilder, Request, Method, Url, header::{IF_NONE_MATCH, ETAG}, StatusCode};
use serde::{Deserialize, de::DeserializeOwned};
use crate::{Error, model::{team::{Team, SimpleTeam, TeamKey}, Year, event::{EventKey, TeamEventStatus}}};
use std::{sync::{Arc, Weak}, collections::HashMap};
use async_trait::async_trait;

use super::Context;

static BASE_ENDPOINT: Lazy<Url> = Lazy::new(|| Url::parse("http://https://www.thebluealliance.com/api/v3").unwrap());

/// Trait implemented by all structures that represent endpoints of the TBA API with methods to
/// make requests using given parameters
#[async_trait]
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
pub struct EndPoints {
    
}

/// Structure representing requests made to the /teams endpoint
pub struct TeamsEndPoint {
    /// Representing the /teams/{page_num} endpoint
    full_page: TeamPageEP,
    /// Represents the /teams/{page_num}/simpl endpoint
    simple_page: SimpleTeamPageEP,
}

macro_rules! endpoint {
    ($name:ident: ($($params:ty),+) => $val:ty where ($($names:ident),+) $path:literal) => {
        struct $name { cache: Cache<($($params),+,), EndPointCacheEntry<$val>> }
        #[async_trait]
        impl self::EndPoint for $name {
            type Params = ($($params),+,);
            type Value = $val;
            async fn get(&self, params: ($($params),+,), ctx: &Context) -> ::std::result::Result<Self::Value, Error> {
                let ($($names),+,) = params;
                let path = ::std::format!($path);
                get_ep::<Self>(
                    path,
                    params,
                    &self.cache,
                    ctx
                ).await
            }
        }
    };
}

endpoint!{TeamPageEP: (usize) => Arc<Vec<Team>> where (page_num) "teams/{page_num}"}
endpoint!{SimpleTeamPageEP: (usize) => Arc<Vec<SimpleTeam>> where (page_num) "teams/{page_num}"}
endpoint!{KeysTeamPageEP: (usize) => Arc<Vec<TeamKey>> where (page_num) "teams/{page_num}"}
endpoint!{TeamPageByYearEP: (Year, usize) => Arc<Vec<Team>> where (year, page_num) "teams/{year}/{page_num}"}
endpoint!{SimpleTeamPageByYearEP: (Year, usize) => Arc<Vec<SimpleTeam>> where (year, page_num) "teams/{year}/{page_num}/simple"}
endpoint!{KeysTeamPageByYearEP: (Year, usize) => Arc<Vec<TeamKey>> where (year, page_num) "teams/{year}/{page_num}/keys"}
endpoint!{
    EventStatusByYearEP: (TeamKey, Year) => Arc<HashMap<EventKey, TeamEventStatus>>
    where (team_key, year) "team/{team_key}/events/{year}/statuses"
}


/// Get the given path from the given endpoint, utilizing the cache
async fn get_ep<T: EndPoint + 'static>(
    path: String,
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
            BASE_ENDPOINT.join(&path)?
        );
    if let Some(ref cached) = cached {
        request = request.header(IF_NONE_MATCH, cached.etag.clone());
    }
    
    let response = request.send().await?;
    match (response.status(), cached) {
        (StatusCode::NOT_MODIFIED, Some(cached)) => Ok(cached.val),
        (code, _) if code.is_success() => {
            let etag = response.headers().get(ETAG).map(|v| v.to_str().map(str::to_owned));
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


