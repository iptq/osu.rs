//! Support for the Hyper crate.

use futures::{Future, Stream, future};
use hyper::client::{Client, Connect};
use hyper::{Error as HyperError, Uri};
use serde_json;
use std::collections::BTreeMap;
use std::fmt::Write;
use std::str::FromStr;
use ::builder::*;
use ::*;

macro_rules! try_uri {
    ($uri:ident) => {
        match Uri::from_str(&$uri) {
            Ok(v) => v,
            Err(why) => return Box::new(future::err(Error::Uri(why))),
        }
    }
}

/// A trait used for implementation on Hyper's client.
pub trait OsuHyperRequester {
    /// Retrieves filtered beatmap results.
    fn get_beatmaps<F, T>(&self, key: T, f: F)
        -> Box<Future<Item = Vec<Beatmap>, Error = Error>>
        where F: FnOnce(GetBeatmapsRequest) -> GetBeatmapsRequest,
              T: AsRef<str>;

    /// Retrieves information about a match.
    fn get_match<T: AsRef<str>>(&self, key: T, match_id: u64)
        -> Box<Future<Item = Match, Error = Error>>;

    /// Retrieves scores for a beatmap.
    fn get_scores<F, T>(&self, key: T, beatmap_id: u64, f: F)
        -> Box<Future<Item = Vec<GameScore>, Error = Error>>
        where F: FnOnce(GetScoreRequest) -> GetScoreRequest,
              T: AsRef<str>;

    /// Retrieves information about a user.
    fn get_user<F, U>(&self, key: &str, user: U, f: F)
        -> Box<Future<Item = Vec<User>, Error = Error>>
        where F: FnOnce(GetUserRequest) -> GetUserRequest,
              U: Into<GetBeatmapUser>;

    /// Retrieves the user's best performances.
    fn get_user_best<F, T, U>(&self, key: T, user: U, f: F)
        -> Box<Future<Item = Vec<Performance>, Error = Error>>
        where F: FnOnce(GetUserBestRequest) -> GetUserBestRequest,
              T: AsRef<str>,
              U: Into<GetBeatmapUser>;

    /// Retrieves information about a user's recent plays.
    fn get_user_recent<F, T, U>(&self, key: T, user: U, f: F)
        -> Box<Future<Item = Vec<RecentPlay>, Error = Error>>
        where F: FnOnce(GetUserRecentRequest) -> GetUserRecentRequest,
              T: AsRef<str>,
              U: Into<GetBeatmapUser>;
}

impl<B, C: Connect> OsuHyperRequester for Client<C, B>
    where B: Stream<Error = HyperError> + 'static, B::Item: AsRef<[u8]> {
    fn get_beatmaps<F, T>(&self, key: T, f: F)
        -> Box<Future<Item = Vec<Beatmap>, Error = Error>>
        where F: FnOnce(GetBeatmapsRequest) -> GetBeatmapsRequest,
              T: AsRef<str> {
        let mut uri = String::new();
        uri.push_str(API_URL);
        uri.push_str("/get_beatmaps?k=");
        uri.push_str(key.as_ref());

        mutate_uri(&mut uri, f(GetBeatmapsRequest::default()).0);

        let uri = try_uri!(uri);

        Box::new(self.get(uri)
            .and_then(|res| res.body().concat2())
            .from_err()
            .and_then(|body| serde_json::from_slice(&body).map_err(From::from)))
    }

    fn get_match<T: AsRef<str>>(&self, key: T, match_id: u64)
        -> Box<Future<Item = Match, Error = Error>> {
        let mut uri = String::new();
        uri.push_str(API_URL);
        uri.push_str("/get_match?k=");
        uri.push_str(key.as_ref());
        let _ = write!(uri, "&mp={}", match_id);

        let uri = try_uri!(uri);

        Box::new(self.get(uri)
            .and_then(|res| res.body().concat2())
            .from_err()
            .and_then(|body| serde_json::from_slice(&body).map_err(From::from)))
    }

    fn get_scores<F, T>(&self, key: T, beatmap_id: u64, f: F)
        -> Box<Future<Item = Vec<GameScore>, Error = Error>>
        where F: FnOnce(GetScoreRequest) -> GetScoreRequest,
              T: AsRef<str> {
        let mut uri = String::new();
        uri.push_str(API_URL);
        uri.push_str("/get_scores?k=");
        uri.push_str(key.as_ref());
        let _ = write!(uri, "&b={}", beatmap_id);

        mutate_uri(&mut uri, f(GetScoreRequest::default()).0);

        let uri = try_uri!(uri);

        Box::new(self.get(uri)
            .and_then(|res| res.body().concat2())
            .from_err()
            .and_then(|body| serde_json::from_slice(&body).map_err(From::from)))
    }

    fn get_user<F, U>(&self, key: &str, user: U, f: F)
        -> Box<Future<Item = Vec<User>, Error = Error>>
        where F: FnOnce(GetUserRequest) -> GetUserRequest,
              U: Into<GetBeatmapUser> {
        let mut uri = String::new();
        uri.push_str(API_URL);
        uri.push_str("/get_user?k=");
        uri.push_str(key.as_ref());

        mutate_uri(&mut uri, f(GetUserRequest::default()).user(user.into()).0);

        let uri = try_uri!(uri);

        Box::new(self.get(uri)
            .and_then(|res| res.body().concat2())
            .from_err()
            .and_then(|body| serde_json::from_slice(&body).map_err(From::from)))
    }

    fn get_user_best<F, T, U>(&self, key: T, user: U, f: F)
        -> Box<Future<Item = Vec<Performance>, Error = Error>>
        where F: FnOnce(GetUserBestRequest) -> GetUserBestRequest,
              T: AsRef<str>,
              U: Into<GetBeatmapUser> {
        let mut uri = String::new();
        uri.push_str(API_URL);
        uri.push_str("/get_user_best?k=");
        uri.push_str(key.as_ref());

        mutate_uri(&mut uri, f(GetUserBestRequest::default()).user(user.into()).0);

        let uri = try_uri!(uri);

        Box::new(self.get(uri)
            .and_then(|res| res.body().concat2())
            .from_err()
            .and_then(|body| serde_json::from_slice(&body).map_err(From::from)))
    }

    fn get_user_recent<F, T, U>(&self, key: T, user: U, f: F)
        -> Box<Future<Item = Vec<RecentPlay>, Error = Error>>
        where F: FnOnce(GetUserRecentRequest) -> GetUserRecentRequest,
              T: AsRef<str>,
              U: Into<GetBeatmapUser> {
        let mut uri = String::new();
        uri.push_str(API_URL);
        uri.push_str("/get_user_recent?k=");
        uri.push_str(key.as_ref());

        mutate_uri(&mut uri, f(GetUserRecentRequest::default()).user(user.into()).0);

        let uri = try_uri!(uri);

        Box::new(self.get(uri)
            .and_then(|res| res.body().concat2())
            .from_err()
            .and_then(|body| serde_json::from_slice(&body).map_err(From::from)))
    }
}

fn mutate_uri(uri: &mut String, map: BTreeMap<&str, String>) {
    for (k, v) in map {
        uri.push('&');
        uri.push_str(k);
        uri.push('=');

        let value_bytes = v.into_bytes();

        unsafe {
            let uri_bytes = uri.as_mut_vec();
            uri_bytes.extend(value_bytes);
        }
    }
}
