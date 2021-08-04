use crate::{
    error::Error,
    types::Thing,
};

/// A client to access reddit
#[derive(Clone)]
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    /// Create a new [`Client`]
    pub fn new() -> Self {
        Client {
            client: reqwest::Client::new(),
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
