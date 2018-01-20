//! An unofficial Rust library for interacting with the osu! API.
//!
//! # Examples
//!
//! Retrieve a beatmap by ID:
//!
//! ```rust,no_run
//! extern crate hyper;
//! extern crate osu;
//!
//! use std::error::Error;
//!
//! # fn try_main() -> Result<(), Box<Error>> {
//! use hyper::Client;
//! use osu::GetBeatmapsRequest;
//!
//! let client = Client::new();
//! let response = client.get_beatmap(&key, |f| f.beatmap_id(71423))?;
//! # }
//! #
//! # fn main() {
//! #     try_main().unwrap();
//! # }
#![deny(missing_docs)]

#[macro_use] extern crate bitflags;
#[macro_use] extern crate serde_derive;

#[cfg(feature = "futures")]
extern crate futures;
#[cfg(feature="hyper")]
extern crate hyper;
#[cfg(feature = "reqwest")]
extern crate reqwest;

extern crate serde;
extern crate serde_json;

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
