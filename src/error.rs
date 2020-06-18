use http::{
    uri::InvalidUri,
    StatusCode,
};

pub type RedditResult<T> = Result<T, RedditError>;

/// Error type for this library
#[derive(Debug)]
pub enum RedditError {
    /// Hyper Error
    Hyper(hyper::Error),

    /// Error parsing a uri
    InvalidUri(InvalidUri),

    /// Invalid status recieved
    InvalidStatus(StatusCode),

    /// Failed to find subreddit
    SubredditNotFound,

    /// Error parsing json. The optional bytes object is the json that failed to parse.
    Json(serde_json::Error, Option<bytes::Bytes>),
}

impl RedditError {
    /// Returns true if the error type is "SubredditNotFound", false otherwise
    pub fn is_subreddit_not_found(&self) -> bool {
        match self {
            RedditError::SubredditNotFound => true,
            _ => false,
        }
    }
}

impl From<hyper::Error> for RedditError {
    fn from(e: hyper::Error) -> Self {
        Self::Hyper(e)
    }
}

impl From<InvalidUri> for RedditError {
    fn from(e: InvalidUri) -> Self {
        RedditError::InvalidUri(e)
    }
}
