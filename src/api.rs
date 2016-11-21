use hyper::Client;
use serde_json;
use std::collections::BTreeMap;
use super::{API_URL, GetBeatmapUser};
use ::builder::*;
use ::error::Result;
use ::model::*;
use ::utils::decode_array;

pub fn get_beatmaps<F>(key: &str, f: F) -> Result<Vec<Beatmap>>
    where F: FnOnce(GetBeatmapsRequest) -> GetBeatmapsRequest {
    let params = params(f(GetBeatmapsRequest::default()).0);
    let response = try!(Client::new()
        .get(&format!("{}/get_beatmaps?k={}{}", API_URL, key, params))
        .send());

    decode_array(try!(serde_json::from_reader(response)), Beatmap::decode)
}

pub fn get_match(key: &str, match_id: u64) -> Result<Match> {
    let response = try!(Client::new()
        .get(&format!("{}/get_match?k={}&mp={}", API_URL, key, match_id))
        .send());

    Match::decode(try!(serde_json::from_reader(response)))
}

pub fn get_scores<F>(key: &str, beatmap_id: u64, f: F) -> Result<Vec<GameScore>>
    where F: FnOnce(GetScoreRequest) -> GetScoreRequest {
    let params = params(f(GetScoreRequest::default()).0);
    let response = try!(Client::new()
        .get(&format!("{}/get_scores?k={}&b={}{}", API_URL, key, beatmap_id, params))
        .send());

    decode_array(try!(serde_json::from_reader(response)), GameScore::decode)
}

pub fn get_user<F, U>(key: &str, user: U, f: F) -> Result<User>
    where F: FnOnce(GetUserRequest) -> GetUserRequest, U: Into<GetBeatmapUser> {
    let params = params(f(GetUserRequest::default()).user(user.into()).0);
    let response = try!(Client::new()
        .get(&format!("{}/get_user?k={}{}", API_URL, key, params))
        .send());

    User::decode(try!(serde_json::from_reader(response)))
}

pub fn get_user_best<F, U>(key: &str, user: U, f: F) -> Result<Vec<Performance>>
    where F: FnOnce(GetUserBestRequest) -> GetUserBestRequest,
          U: Into<GetBeatmapUser> {
    let params = params(f(GetUserBestRequest::default()).user(user.into()).0);
    let response = try!(Client::new()
        .get(&format!("{}/get_user_best?k={}{}", API_URL, key, params))
        .send());

    decode_array(try!(serde_json::from_reader(response)), Performance::decode)
}

pub fn get_user_recent<F, U>(key: &str, user: U, f: F) -> Result<Vec<RecentPlay>>
    where F: FnOnce(GetUserRecentRequest) -> GetUserRecentRequest,
          U: Into<GetBeatmapUser> {
    let params = params(f(GetUserRecentRequest::default()).user(user.into()).0);
    let response = try!(Client::new()
        .get(&format!("{}/get_user_recent?k={}{}", API_URL, key, params))
        .send());

    decode_array(try!(serde_json::from_reader(response)), RecentPlay::decode)
}

fn params(map: BTreeMap<&str, String>) -> String {
    let mut uri = String::new();

    for (k, v) in map {
        uri.push('&');
        uri.push_str(k);
        uri.push('=');
        uri.push_str(&v);
    }

    uri
}
