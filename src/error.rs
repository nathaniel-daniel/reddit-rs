use http::{
    uri::InvalidUri,
    StatusCode,
};

pub type RedditResult<T> = Result<T, RedditError>;

/// Error type for this library
#[derive(Debug, thiserror::Error)]
pub enum RedditError {
    /// Hyper Error
    #[error(transparent)]
    Hyper(#[from] hyper::Error),

    /// Error parsing a uri
    #[error(transparent)]
    InvalidUri(#[from] InvalidUri),

    /// Invalid status recieved
    #[error("Invalid Status {0}")]
    InvalidStatus(StatusCode),

    /// Failed to find subreddit
    #[error("Failed to locate subreddit")]
    SubredditNotFound,

    /// Error parsing json. The optional bytes object is the json that failed to parse.
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

impl RedditError {
    /// Returns true if the error type is "SubredditNotFound", false otherwise
    pub fn is_subreddit_not_found(&self) -> bool {
        matches!(self, RedditError::SubredditNotFound)
    }
}
