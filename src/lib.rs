pub mod client;
pub mod error;
pub mod types;

pub use crate::{
    client::Client,
    error::Error,
    types::{
        Link,
        Listing,
        PostHint,
        Thing,
    },
};

#[cfg(all(feature = "force-native-tls", feature = "force-rustls-tls"))]
compile_error!("cannot enable `force-native-tls` and `force-rustls-tls` at once");
