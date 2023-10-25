# reddit-rs
A basic reddit client lib for Rust.
This library currently allows users to fetch posts from subreddits.

## Example
```rust
#[tokio::main]
async fn main() {
    let client = reddit::Client::new();
    
    /// Fetch 100 posts from the "aww" subreddit
    let subreddit = client.get_subreddit("aww", 100).await.expect("failed to get subreddit");

    dbg!(subreddit);
}
```

## Documentation
Master: <https://nathaniel-daniel.github.io/reddit-rs/reddit/>

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
