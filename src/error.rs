
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP Error: {}", .0)]
    Http(#[from] reqwest::Error),
}
