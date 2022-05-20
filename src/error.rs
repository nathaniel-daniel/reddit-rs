/// Error type for this library
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    /// Json parse error
    #[error("failed to parse json")]
    Json {
        /// The data that was being parsed
        data: Box<str>,

        /// The parse error
        #[source]
        error: serde_json::Error,
    },

    /// Failed to find subreddit
    #[error("failed to locate the subreddit")]
    SubredditNotFound,
}

impl Error {
    /// Returns `true` if the error type is `SubredditNotFound`, `false` otherwise.
    pub fn is_subreddit_not_found(&self) -> bool {
        matches!(self, Self::SubredditNotFound)
    }
}
