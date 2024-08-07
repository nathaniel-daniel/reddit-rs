use crate::{
    error::Error,
    types::Thing,
};

// Guesses for good defaults for the user agent.

// TODO: Extract from target
const DEFAULT_PLATFORM: &str = "pc";

const DEFAULT_APP_ID: &str = env!("CARGO_PKG_NAME");
const DEFAULT_APP_VERSION: &str = env!("CARGO_PKG_VERSION");

// TODO: Is there really a good default to choose here?
const DEFAULT_REDDIT_USERNAME: &str = "deleted";

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
    /// Create a new [`Client`].
    pub fn new() -> Self {
        Self::new_with_user_agent(
            DEFAULT_PLATFORM,
            DEFAULT_APP_ID,
            DEFAULT_APP_VERSION,
            DEFAULT_REDDIT_USERNAME,
        )
    }

    /// Create a new [`Client`] with a user-agent.
    ///
    /// See https://github.com/reddit-archive/reddit/wiki/API#rules
    pub fn new_with_user_agent(
        platform: &str,
        app_id: &str,
        app_version: &str,
        reddit_username: &str,
    ) -> Self {
        let user_agent = format!("{platform}:{app_id}:v{app_version} (by /u/{reddit_username})");

        let mut client_builder = reqwest::Client::builder();
        client_builder = client_builder.user_agent(user_agent);

        let client = client_builder
            .build()
            .expect("failed to build reddit client");

        Self { client }
    }

    /// Get the top posts of a subreddit where subreddit is the name and num_posts is the number of posts to retrieve.
    pub async fn get_subreddit(&self, subreddit: &str, num_posts: usize) -> Result<Thing, Error> {
        let url = format!("https://www.reddit.com/r/{subreddit}.json?limit={num_posts}");
        let res = self.client.get(&url).send().await?.error_for_status()?;

        // Reddit will redirect us here if the subreddit could not be found.
        const SEARCH_URL: &str = "https://www.reddit.com/subreddits/search.json?";
        if res.url().as_str().starts_with(SEARCH_URL) {
            return Err(Error::SubredditNotFound);
        }

        let text = res.text().await?;
        serde_json::from_str(&text).map_err(|error| Error::Json {
            data: text.into(),
            error,
        })
    }

    /// Get the post data for a post from a given subreddit
    pub async fn get_post(&self, subreddit: &str, post_id: &str) -> Result<Vec<Thing>, Error> {
        let url = format!("https://www.reddit.com/r/{subreddit}/comments/{post_id}.json");
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
        println!(
            "# of children: {}",
            subreddit.data.as_listing().unwrap().children.len()
        );
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn get_post_works() {
        let post_data = [
            ("dankmemes", "h966lq"),
            // ("dankvideos", "h8p0py"), // Subreddit got privated, last tested 12/23/2022. Uncomment in the future to see if that is still the case.
            ("oddlysatisfying", "ha7obv"),
        ];
        let client = Client::new();

        for (subreddit, post_id) in post_data.iter() {
            let post = client
                .get_post(subreddit, post_id)
                .await
                .expect("failed to get post");
            dbg!(&post);
        }
    }

    #[tokio::test]
    #[ignore]
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
                Err(Error::Json { data, error }) => {
                    let line = error.line();
                    let column = error.column();

                    // Try to get error in data
                    let maybe_data = data.split('\n').nth(line.saturating_sub(1)).map(|line| {
                        let start = column.saturating_sub(30);

                        &line[start..]
                    });

                    let _ = tokio::fs::write("subreddit-error.json", data.as_bytes())
                        .await
                        .is_ok();

                    panic!(
                        "failed to get subreddit \"{subreddit}\": {error:#?}\ndata: {maybe_data:?}"
                    );
                }
                Err(error) => {
                    panic!("failed to get subreddit \"{subreddit}\": {error:#?}");
                }
            }
        }
    }

    #[tokio::test]
    #[ignore]
    async fn invalid_subreddit() {
        let client = Client::new();
        let error = client.get_subreddit("gfdghfj", 25).await.unwrap_err();
        assert!(error.is_subreddit_not_found(), "error = {error:#?}");
    }
}
