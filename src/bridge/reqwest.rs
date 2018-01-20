use reqwest::Client;
use serde_json;
use std::collections::BTreeMap;
use std::fmt::Write;
use ::builder::*;
use ::*;

/// A trait used for implementation on Reqwest's client.
pub trait OsuReqwestRequester {
    /// Retrieves filtered beatmap results.
    fn get_beatmaps<F, T>(&self, key: T, f: F) -> Result<Vec<Beatmap>>
        where F: FnOnce(GetBeatmapsRequest) -> GetBeatmapsRequest,
              T: AsRef<str>;

    /// Retrieves information about a match.
    fn get_match<T: AsRef<str>>(&self, key: T, match_id: u64) -> Result<Match>;

    /// Retrieves scores for a beatmap.
    fn get_scores<F, T>(&self, key: T, beatmap_id: u64, f: F)
        -> Result<Vec<GameScore>>
        where F: FnOnce(GetScoreRequest) -> GetScoreRequest, T: AsRef<str>;

    /// Retrieves information about a user.
    fn get_user<F, T, U>(&self, key: T, user: U, f: F) -> Result<Vec<User>>
        where F: FnOnce(GetUserRequest) -> GetUserRequest,
              T: AsRef<str>,
              U: Into<GetBeatmapUser>;

    /// Retrieves the user's best performances.
    fn get_user_best<F, T, U>(&self, key: T, user: U, f: F)
        -> Result<Vec<Performance>>
        where F: FnOnce(GetUserBestRequest) -> GetUserBestRequest,
              T: AsRef<str>,
              U: Into<GetBeatmapUser>;

    /// Retrieves information about a user's recent plays.
    fn get_user_recent<F, T, U>(&self, key: T, user: U, f: F)
        -> Result<Vec<RecentPlay>>
        where F: FnOnce(GetUserRecentRequest) -> GetUserRecentRequest,
              T: AsRef<str>,
              U: Into<GetBeatmapUser>;
}

impl OsuReqwestRequester for Client {
    fn get_beatmaps<F, T>(&self, key: T, f: F) -> Result<Vec<Beatmap>>
        where F: FnOnce(GetBeatmapsRequest) -> GetBeatmapsRequest,
              T: AsRef<str> {
        let mut uri = String::new();
        uri.push_str(API_URL);
        uri.push_str("/get_beatmaps?k=");
        uri.push_str(key.as_ref());

        mutate_uri(&mut uri, f(GetBeatmapsRequest::default()).0);

        let response = self.get(&uri).send()?;

        serde_json::from_reader(response).map_err(From::from)
    }

    fn get_match<T: AsRef<str>>(&self, key: T, match_id: u64) -> Result<Match> {
        let mut uri = String::new();
        uri.push_str(API_URL);
        uri.push_str("/get_match?k=");
        uri.push_str(key.as_ref());
        write!(uri, "&mp={}", match_id)?;

        let response = self.get(&uri).send()?;

        serde_json::from_reader(response).map_err(From::from)
    }

    fn get_scores<F, T>(&self, key: T, beatmap_id: u64, f: F)
        -> Result<Vec<GameScore>>
        where F: FnOnce(GetScoreRequest) -> GetScoreRequest,
              T: AsRef<str> {
        let mut uri = String::new();
        uri.push_str(API_URL);
        uri.push_str("/get_scores?k=");
        uri.push_str(key.as_ref());
        write!(uri, "&b={}", beatmap_id)?;

        mutate_uri(&mut uri, f(GetScoreRequest::default()).0);

        let response = self.get(&uri).send()?;

        serde_json::from_reader(response).map_err(From::from)
    }

    fn get_user<F, T, U>(&self, key: T, user: U, f: F)
        -> Result<Vec<User>>
        where F: FnOnce(GetUserRequest) -> GetUserRequest,
              T: AsRef<str>,
              U: Into<GetBeatmapUser> {
        let mut uri = String::new();
        uri.push_str(API_URL);
        uri.push_str("/get_user?k=");
        uri.push_str(key.as_ref());

        mutate_uri(&mut uri, f(GetUserRequest::default()).user(user.into()).0);

        let response = self.get(&uri).send()?;

        serde_json::from_reader(response).map_err(From::from)
    }

    fn get_user_best<F, T, U>(&self, key: T, user: U, f: F)
        -> Result<Vec<Performance>>
        where F: FnOnce(GetUserBestRequest) -> GetUserBestRequest,
              T: AsRef<str>,
              U: Into<GetBeatmapUser> {
        let mut uri = String::new();
        uri.push_str(API_URL);
        uri.push_str("/get_user_best?k=");
        uri.push_str(key.as_ref());

        mutate_uri(&mut uri, f(GetUserBestRequest::default()).user(user.into()).0);

        let response = self.get(&uri).send()?;

        serde_json::from_reader(response).map_err(From::from)
    }

    fn get_user_recent<F, T, U>(&self, key: T, user: U, f: F)
        -> Result<Vec<RecentPlay>>
        where F: FnOnce(GetUserRecentRequest) -> GetUserRecentRequest,
              T: AsRef<str>,
              U: Into<GetBeatmapUser> {
        let mut uri = String::new();
        uri.push_str(API_URL);
        uri.push_str("/get_user_recent?k=");
        uri.push_str(key.as_ref());

        mutate_uri(&mut uri, f(GetUserRecentRequest::default()).user(user.into()).0);

        let response = self.get(&uri).send()?;

        serde_json::from_reader(response).map_err(From::from)
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
