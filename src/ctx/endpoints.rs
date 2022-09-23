
use moka::sync::Cache;
use once_cell::sync::Lazy;
use reqwest::{RequestBuilder, Request, Method, Url, header::{IF_NONE_MATCH, ETAG}, StatusCode};
use serde::{Deserialize, de::DeserializeOwned};
use crate::{Error, model::team::Team};
use std::sync::{Arc, Weak};
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
    async fn get(&mut self, params: Self::Params, ctx: &Context) -> Result<Self::Value, Error>;
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
    full_page: FullPageEP,
}

pub struct FullPageEP {
    cache: Cache<usize, EndPointCacheEntry<Arc<Vec<Team>>>>
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

#[async_trait]
impl EndPoint for FullPageEP {
    type Params = usize;
    type Value = Arc<Vec<Team>>;

    async fn get(&mut self, params: Self::Params, ctx: &Context) -> Result<Self::Value, Error> {
        let path = format!("teams/{}", params);
        get_ep::<Self>(
            path,
            params,
            &self.cache,
            ctx
        ).await
    }
}
