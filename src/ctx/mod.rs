pub mod endpoints;

use std::{time::Duration, sync::Arc};


use awc::{Client, http::header::HeaderName, ClientBuilder, error::HttpError};

use self::endpoints::EndPoints;



/// Context for interacting with the API, containing all state needed to make requests over the
/// internet
pub struct Context {
    pub(crate) client: Arc<Client>,
    pub(crate) endpoints: EndPoints,
}

impl Context {
    /// Create a new context with the given API key
    pub fn authenticate(tba_auth_key: impl AsRef<str>) -> Result<Self, HttpError> {
        Ok(Context {
            client: Arc::new(ClientBuilder::new()
                .add_default_header(
                    (
                        HeaderName::from_static("X-TBA-Auth-Key"),
                        tba_auth_key
                            .as_ref()
                            .trim()
                    )
                )
                .timeout(Duration::from_secs(30))
                .finish()
            ),
            endpoints: Default::default(),
        })
    }
}
