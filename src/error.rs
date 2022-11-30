use awc::http::StatusCode;



#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Send Request Error: {0}")]
    Http(#[from] awc::error::SendRequestError),
    #[error("Failed to convert HTTP header value to string: {0}")]
    ToStr(#[from] awc::http::header::ToStrError),
    #[error("Failed to deserialize JSON response: {0}")]
    JSON(#[from] awc::error::JsonPayloadError),
    #[error("Invalid HTTP header: {0}")]
    InvalidHeader(#[from] awc::http::header::InvalidHeaderName),
    #[error("Failed to parse URL: {0}")]
    URLParse(#[from] url::ParseError),
    #[error("Http request returned status code {}: {}", .0.as_u16(), .0.as_str())]
    BadResponse(StatusCode),
    #[error("Failed to convert an HTTP response header value to string: {0}")]
    BadHeaderValue(#[from] awc::http::header::InvalidHeaderValue),
}
