//! # osu.rs
//!
//! Unofficial Rust crate for the osu! API.
//!
//! [Documentation][docs]
//!
//! ### Installation
//!
//! Add the following dependency to your Cargo.toml:
//!
//! ```toml
//! osu = "0.2"
//! ```
//!
//! And include it in your project:
//!
//! ```rust
//! extern crate osu;
//! ```
//!
//! ### Examples
//!
//! Using `hyper` with the `hyper-tls` HTTPS connector, retrieve the start time of a
//! match by ID:
//!
//! ```rust,no_run
//! # #[cfg(feature = "hyper-support")]
//! extern crate futures;
//! # #[cfg(feature = "hyper-support")]
//! extern crate hyper;
//! # #[cfg(feature = "hyper-support")]
//! extern crate hyper_tls;
//! extern crate osu;
//! # #[cfg(feature = "hyper-support")]
//! extern crate tokio_core;
//!
//! # #[cfg(feature = "hyper-support")]
//! use futures::Future;
//! # #[cfg(feature = "hyper-support")]
//! use hyper::client::{Client, HttpConnector};
//! # #[cfg(feature = "hyper-support")]
//! use hyper_tls::HttpsConnector;
//! use osu::bridge::hyper::OsuHyperRequester;
//! use std::error::Error;
//! use std::env;
//! # #[cfg(feature = "hyper-support")]
//! use tokio_core::reactor::Core;
//!
//! # #[cfg(feature = "hyper-support")]
//! fn try_main() -> Result<(), Box<Error>> {
//!     let mut core = Core::new()?;
//!     let client = Client::configure()
//!         .connector(HttpsConnector::new(4, &core.handle())?)
//!         .build(&core.handle());
//!     let key = env::var("OSU_KEY")?;
//!
//!     let done = client.get_match(&key, 71641).map(|found| {
//!         println!("Match start time: {}", found.start_time);
//!
//!         ()
//!     }).map_err(|_| ());
//!
//!     core.run(done).expect("Error running core");
//!
//!     Ok(())
//! }
//!
//! fn main() {
//!     # #[cfg(feature = "hyper-support")]
//!     try_main().unwrap();
//! }
//! ```
//!
//! Using reqwest, retrieve a match's start time by ID:
//!
//! ```rust,no_run
//! extern crate osu;
//! # #[cfg(feature = "reqwest-support")]
//! extern crate reqwest;
//!
//! # #[cfg(feature = "reqwest-support")]
//! use osu::bridge::reqwest::OsuReqwestRequester;
//! # #[cfg(feature = "reqwest-support")]
//! use reqwest::Client;
//! use std::error::Error;
//! use std::env;
//!
//! # #[cfg(feature = "reqwest-support")]
//! fn try_main() -> Result<(), Box<Error>> {
//!     let key = env::var("OSU_KEY")?;
//!     let client = Client::new();
//!     let found = client.get_match(&key, 71641)?;
//!
//!     println!("Match start time: {}", found.start_time);
//!
//!     Ok(())
//! }
//!
//! fn main() {
//!     # #[cfg(feature = "reqwest-support")]
//!     try_main().unwrap();
//! }
//! ```

// #![deny(missing_docs)]

#[macro_use] extern crate bitflags;
#[macro_use] extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[cfg(feature = "futures")]
extern crate futures;
#[cfg(feature = "hyper")]
extern crate hyper;
#[cfg(feature = "reqwest")]
extern crate reqwest;

/// The base URL for all requests to osu!'s API.
///
/// Although this is not necessary for use with the library, but is exposed for
/// any extra purposes that might be needed.
pub const API_URL: &'static str = "https://osu.ppy.sh/api";

pub mod bridge;
pub mod builder;
pub mod error;

mod model;

pub use error::{Error, Result};
pub use model::*;

/// Information for retrieving a user.
///
/// User retrieval can either be done with an ID or with a username.
pub enum GetBeatmapUser {
    /// The user's ID.
    UserId(u64),
    /// The user's username.
    Username(String),
}

impl<'a> From<&'a str> for GetBeatmapUser {
    fn from(username: &'a str) -> GetBeatmapUser {
        GetBeatmapUser::Username(username.to_owned())
    }
}

impl From<String> for GetBeatmapUser {
    fn from(username: String) -> GetBeatmapUser {
        GetBeatmapUser::Username(username)
    }
}

impl From<u64> for GetBeatmapUser {
    fn from(user_id: u64) -> GetBeatmapUser {
        GetBeatmapUser::UserId(user_id)
    }
}
