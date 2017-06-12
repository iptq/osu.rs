use hyper::client::{Client, Response as HyperResponse};
use serde_json;
use std::collections::BTreeMap;
use std::fmt::Write;
use super::{API_URL, GetBeatmapUser};
use ::builder::*;
use ::error::Result;
use ::model::*;

pub fn get_beatmaps<F>(key: &str, f: F) -> Result<Vec<Beatmap>>
    where F: FnOnce(GetBeatmapsRequest) -> GetBeatmapsRequest {
    let mut uri = String::new();
    uri.push_str(API_URL);
    uri.push_str("/get_beatmaps?k=");
    uri.push_str(key);

    mutate_uri(&mut uri, f(GetBeatmapsRequest::default()).0);

    let response = Client::new().get(&uri).send()?;

    serde_json::from_reader::<HyperResponse, Vec<Beatmap>>(response).map_err(From::from)
}

pub fn get_match(key: &str, match_id: u64) -> Result<Match> {
    let mut uri = String::new();
    uri.push_str(API_URL);
    uri.push_str("/get_match?k=");
    uri.push_str(key);
    write!(uri, "&mp={}", match_id)?;

    let response = Client::new().get(&uri).send()?;

    serde_json::from_reader::<HyperResponse, Match>(response).map_err(From::from)
}

pub fn get_scores<F>(key: &str, beatmap_id: u64, f: F) -> Result<Vec<GameScore>>
    where F: FnOnce(GetScoreRequest) -> GetScoreRequest {
    let mut uri = String::new();
    uri.push_str(API_URL);
    uri.push_str("/get_scores?k=");
    uri.push_str(key);
    write!(uri, "&b={}", beatmap_id)?;

    mutate_uri(&mut uri, f(GetScoreRequest::default()).0);

    let response = Client::new().get(&uri).send()?;

    serde_json::from_reader::<HyperResponse, Vec<GameScore>>(response).map_err(From::from)
}

pub fn get_user<F, U>(key: &str, user: U, f: F) -> Result<Vec<User>>
    where F: FnOnce(GetUserRequest) -> GetUserRequest, U: Into<GetBeatmapUser> {
    let mut uri = String::new();
    uri.push_str(API_URL);
    uri.push_str("/get_user?k=");
    uri.push_str(key);

    mutate_uri(&mut uri, f(GetUserRequest::default()).user(user.into()).0);

    let response = Client::new().get(&uri).send()?;

    serde_json::from_reader::<HyperResponse, Vec<User>>(response).map_err(From::from)
}

pub fn get_user_best<F, U>(key: &str, user: U, f: F) -> Result<Vec<Performance>>
    where F: FnOnce(GetUserBestRequest) -> GetUserBestRequest, U: Into<GetBeatmapUser> {
    let mut uri = String::new();
    uri.push_str(API_URL);
    uri.push_str("/get_user_best?k=");
    uri.push_str(key);

    mutate_uri(&mut uri, f(GetUserBestRequest::default()).user(user.into()).0);

    let response = Client::new().get(&uri).send()?;

    serde_json::from_reader::<HyperResponse, Vec<Performance>>(response).map_err(From::from)
}

pub fn get_user_recent<F, U>(key: &str, user: U, f: F) -> Result<Vec<RecentPlay>>
    where F: FnOnce(GetUserRecentRequest) -> GetUserRecentRequest, U: Into<GetBeatmapUser> {
    let mut uri = String::new();
    uri.push_str(API_URL);
    uri.push_str("/get_user_recent?k=");
    uri.push_str(key);

    mutate_uri(&mut uri, f(GetUserRecentRequest::default()).user(user.into()).0);

    let response = Client::new().get(&uri).send()?;

    serde_json::from_reader::<HyperResponse, Vec<RecentPlay>>(response).map_err(From::from)
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

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;

    #[test]
    fn tests() {
        let key = env::var("OSU_KEY").unwrap();
        let u = "Cookiezi";

        let _ = get_beatmaps(&key, |f| f).expect("get beatmaps");
        let _ = get_scores(&key, 191904, |f| f).expect("get scores");
        let _ = get_user(&key, u, |f| f).expect("get user");
        let _ = get_user_best(&key, u, |f| f).expect("get user best");
        let _ = get_user_recent(&key, u, |f| f).expect("get user recent");
    }
}
