pub mod client;
pub mod error;
pub mod types;

pub use crate::{
    client::Client,
    error::{
        RedditError,
        RedditResult,
    },
    types::{
        Listing,
        PostHint,
        SubRedditEntry,
        SubRedditEntryData,
        SubRedditListing,
    },
};
pub use http::{
    uri::InvalidUri,
    StatusCode,
};
