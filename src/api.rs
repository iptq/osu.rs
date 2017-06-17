use super::GetBeatmapUser;
use ::builder::*;
use ::error::Result;
use ::model::*;

#[cfg(any(feature="hyper"))]
use std::collections::BTreeMap;

pub trait OsuRequester {
    fn get_beatmaps<F>(&self, key: &str, f: F) -> Result<Vec<Beatmap>>
        where F: FnOnce(GetBeatmapsRequest) -> GetBeatmapsRequest;

    fn get_match(&self, key: &str, match_id: u64) -> Result<Match>;

    fn get_scores<F>(&self, key: &str, beatmap_id: u64, f: F) -> Result<Vec<GameScore>>
        where F: FnOnce(GetScoreRequest) -> GetScoreRequest;

    fn get_user<F, U>(&self, key: &str, user: U, f: F) -> Result<Vec<User>>
        where F: FnOnce(GetUserRequest) -> GetUserRequest, U: Into<GetBeatmapUser>;

    fn get_user_best<F, U>(&self, key: &str, user: U, f: F) -> Result<Vec<Performance>>
        where F: FnOnce(GetUserBestRequest) -> GetUserBestRequest, U: Into<GetBeatmapUser>;

    fn get_user_recent<F, U>(&self, key: &str, user: U, f: F) -> Result<Vec<RecentPlay>>
        where F: FnOnce(GetUserRecentRequest) -> GetUserRecentRequest, U: Into<GetBeatmapUser>;
}

#[cfg(feature="hyper")]
mod hyper_support {
    use hyper::Client;
    use serde_json;
    use std::fmt::Write;
    use super::OsuRequester;
    use ::builder::*;
    use ::*;

    impl OsuRequester for Client {
        fn get_beatmaps<F>(&self, key: &str, f: F) -> Result<Vec<Beatmap>>
            where F: FnOnce(GetBeatmapsRequest) -> GetBeatmapsRequest {
            let mut uri = String::new();
            uri.push_str(API_URL);
            uri.push_str("/get_beatmaps?k=");
            uri.push_str(key);

            super::mutate_uri(&mut uri, f(GetBeatmapsRequest::default()).0);

            let response = self.get(&uri).send()?;

            serde_json::from_reader(response).map_err(From::from)
        }

        fn get_match(&self, key: &str, match_id: u64) -> Result<Match> {
            let mut uri = String::new();
            uri.push_str(API_URL);
            uri.push_str("/get_match?k=");
            uri.push_str(key);
            write!(uri, "&mp={}", match_id)?;

            let response = self.get(&uri).send()?;

            serde_json::from_reader(response).map_err(From::from)
        }

        fn get_scores<F>(&self, key: &str, beatmap_id: u64, f: F) -> Result<Vec<GameScore>>
            where F: FnOnce(GetScoreRequest) -> GetScoreRequest {
            let mut uri = String::new();
            uri.push_str(API_URL);
            uri.push_str("/get_scores?k=");
            uri.push_str(key);
            write!(uri, "&b={}", beatmap_id)?;

            super::mutate_uri(&mut uri, f(GetScoreRequest::default()).0);

            let response = self.get(&uri).send()?;

            serde_json::from_reader(response).map_err(From::from)
        }

        fn get_user<F, U>(&self, key: &str, user: U, f: F) -> Result<Vec<User>>
            where F: FnOnce(GetUserRequest) -> GetUserRequest, U: Into<GetBeatmapUser> {
            let mut uri = String::new();
            uri.push_str(API_URL);
            uri.push_str("/get_user?k=");
            uri.push_str(key);

            super::mutate_uri(&mut uri, f(GetUserRequest::default()).user(user.into()).0);

            let response = self.get(&uri).send()?;

            serde_json::from_reader(response).map_err(From::from)
        }

        fn get_user_best<F, U>(&self, key: &str, user: U, f: F) -> Result<Vec<Performance>>
            where F: FnOnce(GetUserBestRequest) -> GetUserBestRequest, U: Into<GetBeatmapUser> {
            let mut uri = String::new();
            uri.push_str(API_URL);
            uri.push_str("/get_user_best?k=");
            uri.push_str(key);

            super::mutate_uri(&mut uri, f(GetUserBestRequest::default()).user(user.into()).0);

            let response = self.get(&uri).send()?;

            serde_json::from_reader(response).map_err(From::from)
        }

        fn get_user_recent<F, U>(&self, key: &str, user: U, f: F) -> Result<Vec<RecentPlay>>
            where F: FnOnce(GetUserRecentRequest) -> GetUserRecentRequest, U: Into<GetBeatmapUser> {
            let mut uri = String::new();
            uri.push_str(API_URL);
            uri.push_str("/get_user_recent?k=");
            uri.push_str(key);

            super::mutate_uri(&mut uri, f(GetUserRecentRequest::default()).user(user.into()).0);

            let response = self.get(&uri).send()?;

            serde_json::from_reader(response).map_err(From::from)
        }
    }
}

#[cfg(any(feature="hyper"))]
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
