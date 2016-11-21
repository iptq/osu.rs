#[macro_use] extern crate bitflags;

extern crate hyper;
extern crate serde_json;

const API_URL: &'static str = "https://osu.ppy.sh/api";

pub mod builder;
pub mod error;

#[macro_use]
mod utils;

mod api;
mod model;

pub use api::*;
pub use error::{Error, Result};
pub use model::*;

pub enum GetBeatmapUser {
    UserId(u64),
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
