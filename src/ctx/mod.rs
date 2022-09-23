use reqwest::{Client, ClientBuilder, header::HeaderMap};

use crate::Error;


/// Context for interacting with the API, containing all state needed to make requests over the
/// internet
pub struct Context {
    pub(crate) client: Client,
}

impl Context {
    /// Create a new context with the given API key
    pub fn new(tba_auth_key: impl AsRef<str>) -> Result<Self, Error> {
        let mut headers = HeaderMap::new();
        headers.insert("X-TBA-Auth-Key", tba_auth_key.as_ref().parse()?);

        Ok(Context {
            client: ClientBuilder::new()
                .default_headers(headers)
                .build()?,
        })
    }
}
