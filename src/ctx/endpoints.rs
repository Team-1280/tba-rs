
use moka::sync::Cache;
use once_cell::sync::Lazy;
use reqwest::{RequestBuilder, Request, Method, Url, header::{IF_NONE_MATCH, ETAG}, StatusCode};
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

#[async_trait]
impl EndPoint for FullPageEP {
    type Params = usize;
    type Value = Arc<Vec<Team>>;

    async fn get(&mut self, params: Self::Params, ctx: &Context) -> Result<Self::Value, Error> {
        let path = format!("teams/{}", params);
        let cached = self.cache.get(&params);
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
                let val = Arc::new(response.json::<Vec<Team>>().await?);
                if let Some(etag) = etag {
                    self.cache.insert(
                        params,
                        EndPointCacheEntry {
                            val: Arc::clone(&val),
                            etag: etag?
                        }
                    );
                }

                Ok(val)
            },
            (code, _) => Err(Error::BadResponse(code)),
        }
    }
}
