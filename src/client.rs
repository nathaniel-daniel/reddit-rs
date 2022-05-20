use crate::{
    error::Error,
    types::Thing,
};

/// A client to access reddit
#[derive(Clone)]
pub struct Client {
    /// The inner http client.
    ///
    /// It probably shouldn't be used directly by you.
    /// It also sets a strange user-agent as well in accordance with reddit's request.
    pub client: reqwest::Client,
}

impl Client {
    /// Create a new [`Client`]
    pub fn new() -> Self {
        // Just guess some good defaults.
        // TODO: Extract from target
        let platform = "pc";
        let app_id = env!("CARGO_PKG_NAME");
        let version = env!("CARGO_PKG_VERSION");
        // TODO: Is there really a good default to choose here?
        let reddit_username = "deleted";
        Self::new_with_user_agent(platform, app_id, version, reddit_username)
    }

    /// Create a new [`Client`] with a user-agent.
    ///
    /// See https://github.com/reddit-archive/reddit/wiki/API#rules
    pub fn new_with_user_agent(
        platform: &str,
        app_id: &str,
        version: &str,
        reddit_username: &str,
    ) -> Self {
        let user_agent = format!(
            "{platform}:{app_id}:{version} (by /u/{reddit_username})",
            platform = platform,
            app_id = app_id,
            version = version,
            reddit_username = reddit_username
        );

        let mut client_builder = reqwest::Client::builder();
        client_builder = client_builder.user_agent(user_agent);

        #[cfg(feature = "force-native-tls")]
        {
            client_builder = client_builder.use_native_tls();
        }

        #[cfg(feature = "force-rustls-tls")]
        {
            client_builder = client_builder.use_rustls_tls();
        }

        Self {
            client: client_builder
                .build()
                .expect("failed to build reddit client"),
        }
    }

    /// Get the top posts of a subreddit where subreddit is the name and num_posts is the number of posts to retrieve.
    pub async fn get_subreddit(&self, subreddit: &str, num_posts: usize) -> Result<Thing, Error> {
        let url = format!(
            "https://www.reddit.com/r/{}.json?limit={}",
            subreddit, num_posts
        );
        let res = self.client.get(&url).send().await?.error_for_status()?;

        // Reddit will redirect us here if the subreddit could not be found.
        const SEARCH_URL: &str = "https://www.reddit.com/subreddits/search.json?";
        if res.url().as_str().starts_with(SEARCH_URL) {
            return Err(Error::SubredditNotFound);
        }

        Ok(res.json().await?)
    }

    /// Get the post data for a post from a given subreddit
    pub async fn get_post(&self, subreddit: &str, post_id: &str) -> Result<Vec<Thing>, Error> {
        let url = format!(
            "https://www.reddit.com/r/{}/comments/{}.json",
            subreddit, post_id
        );
        Ok(self
            .client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
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

    async fn get_subreddit(name: &str) -> Result<(), Error> {
        let client = Client::new();
        // 25 is the default
        let subreddit = client.get_subreddit(name, 100).await?;
        println!("# of children: {}", subreddit.data.as_listing().unwrap().children.len());
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
            match get_subreddit(subreddit).await {
                Ok(()) => {}
                Err(error) => {
                    panic!("failed to get subreddit `{}`: {:?}", subreddit, error);
                }
            }
        }
    }

    #[tokio::test]
    async fn invalid_subreddit() {
        let client = Client::new();
        let err = client.get_subreddit("gfdghfj", 25).await.unwrap_err();
        assert!(err.is_subreddit_not_found(), "err = {:#?}", err);
    }
}
