use reqwest::StatusCode;


#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP Error: {}", .0)]
    Http(#[from] reqwest::Error),
    #[error("Invalid HTTP header: {}", .0)]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),
    #[error("Failed to parse URL: {}", .0)]
    URLParse(#[from] url::ParseError),
    #[error("Http request returned status code {}: {}", .0.as_u16(), .0.as_str())]
    BadResponse(StatusCode),
    #[error("Failed to convert an HTTP response header value to string: {}", .0)]
    BadHeaderValue(#[from] reqwest::header::ToStrError),
}
