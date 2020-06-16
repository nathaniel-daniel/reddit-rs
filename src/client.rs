use crate::{
    error::RedditError,
    types::SubRedditListing,
};
use http::StatusCode;
use hyper::{
    client::HttpConnector,
    Client as HyperClient,
};
use hyper_tls::HttpsConnector;

/// A client to access reddit
pub struct Client {
    client: HyperClient<HttpsConnector<HttpConnector>, hyper::Body>,
}

impl Client {
    /// Create a new client
    pub fn new() -> Self {
        let https = HttpsConnector::new();
        let client = HyperClient::builder().build::<_, hyper::Body>(https);
        Client { client }
    }

    /// Get the top posts of a subreddit where subreddit is the name and n is the number of posts to retrieve.
    pub async fn get_subreddit(
        &self,
        subreddit: &str,
        n: usize,
    ) -> Result<SubRedditListing, RedditError> {
        let uri = format!("https://www.reddit.com/r/{}.json?limit={}", subreddit, n).parse()?;
        let res = self.client.get(uri).await?;

        let status = res.status();
        if !status.is_success() {
            return match status {
                StatusCode::FOUND => match res.headers().get(hyper::header::LOCATION) {
                    Some(link) => {
                        let url = b"https://www.reddit.com/subreddits/search.json?";
                        if link.as_ref().starts_with(url) {
                            Err(RedditError::NotFound)
                        } else {
                            Err(RedditError::InvalidStatusCode(status))
                        }
                    }
                    None => Err(RedditError::InvalidStatusCode(status)),
                },
                _ => Err(RedditError::InvalidStatusCode(status)),
            };
        }

        let body = hyper::body::to_bytes(res).await?;

        serde_json::from_slice(&body).map_err(|e| RedditError::Json(e, Some(body)))
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
