/// Error type for this library
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

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
