use crate::{
    error::{
        RedditError,
        RedditResult,
    },
    types::Thing,
};
use bytes::Bytes;
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

    async fn get_bytes(&self, uri: hyper::Uri) -> RedditResult<Bytes> {
        let res = self.client.get(uri).await?;

        let status = res.status();
        if !status.is_success() {
            return Err(RedditError::InvalidStatus(status));
        }
        let body = hyper::body::to_bytes(res).await?;

        Ok(body)
    }

    /// Get the top posts of a subreddit where subreddit is the name and num_posts is the number of posts to retrieve.
    pub async fn get_subreddit(&self, subreddit: &str, num_posts: usize) -> RedditResult<Thing> {
        let uri = format!(
            "https://www.reddit.com/r/{}.json?limit={}",
            subreddit, num_posts
        )
        .parse()?;
        let res = self.client.get(uri).await?;

        let status = res.status();
        if !status.is_success() {
            return match status {
                StatusCode::FOUND => match res.headers().get(hyper::header::LOCATION) {
                    Some(link) => {
                        let url = b"https://www.reddit.com/subreddits/search.json?";
                        if link.as_ref().starts_with(url) {
                            Err(RedditError::SubredditNotFound)
                        } else {
                            Err(RedditError::InvalidStatus(status))
                        }
                    }
                    None => Err(RedditError::InvalidStatus(status)),
                },
                _ => Err(RedditError::InvalidStatus(status)),
            };
        }

        let body = hyper::body::to_bytes(res).await?;

        serde_json::from_slice(&body).map_err(|e| RedditError::Json(e, Some(body)))
    }

    /// Get the post data for a post from a given subreddit
    pub async fn get_post(&self, subreddit: &str, post_id: &str) -> RedditResult<Vec<Thing>> {
        let uri = format!(
            "https://www.reddit.com/r/{}/comments/{}.json",
            subreddit, post_id
        )
        .parse()?;
        let body = self.get_bytes(uri).await?;

        serde_json::from_slice(&body).map_err(|e| RedditError::Json(e, Some(body)))
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    async fn get_subreddit(name: &str) -> RedditResult<()> {
        let client = Client::new();
        // 25 is the default
        let subreddit = client.get_subreddit(name, 100).await?;
        println!("{}", subreddit.data.as_listing().unwrap().children.len());
        Ok(())
    }

    #[tokio::test]
    async fn get_post_works() {
        let post_data = [
            ("dankmemes", "h966lq"),
            ("dankvideos", "h8p0py"),
            ("oddlysatisfying", "ha7obv"),
        ];
        let client = Client::new();

        for (subreddit, post_id) in post_data.iter() {
            let res = client.get_post(subreddit, post_id).await.unwrap();
            dbg!(res);
        }
    }

    #[tokio::test]
    async fn get_subreddit_works() {
        let subreddits = [
            "forbiddensnacks",
            "dankmemes",
            "cursedimages",
            "MEOW_IRL",
            "cuddleroll",
            "cromch",
            "cats",
            "cursed_images",
            "aww",
        ];

        for subreddit in subreddits.iter() {
            get_subreddit(subreddit).await.unwrap();
        }
    }

    #[tokio::test]
    async fn invalid_subreddit() {
        let client = Client::new();
        let err = client.get_subreddit("gfdghfj", 25).await.unwrap_err();
        assert!(err.is_subreddit_not_found(), "err = {:#?}", err);
    }
}
