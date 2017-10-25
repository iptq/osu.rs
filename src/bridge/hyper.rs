use hyper::Client;
use serde_json;
use std::collections::BTreeMap;
use std::fmt::Write;
use ::builder::*;
use ::*;

/// A trait used for implementation on various HTTP clients.
pub trait OsuHyperRequester {
    /// Retrieves filtered beatmap results.
    ///
    /// # Examples
    ///
    /// Retrieve 25 beatmaps, including convert beatmaps:
    ///
    /// ```rust,no_run
    /// # extern crate hyper;
    /// # extern crate osu;
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// use hyper::Client;
    /// use osu::OsuHyperRequester;
    /// use std::env;
    ///
    /// let osu_key = env::var("OSU_KEY")?;
    /// let client = Client::new();
    ///
    /// let _ = client.get_beatmaps(osu_key, |f| f
    ///     .include_converted(true)
    ///     .limit(25))?;
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    fn get_beatmaps<F>(&self, key: &str, f: F) -> Result<Vec<Beatmap>>
        where F: FnOnce(GetBeatmapsRequest) -> GetBeatmapsRequest;

    /// Retrieves information about a match.
    ///
    /// # Examples
    ///
    /// Retrieve the match with an ID of `71654`:
    ///
    /// ```rust,no_run
    /// # extern crate hyper;
    /// # extern crate osu;
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// use hyper::Client;
    /// use osu::OsuHyperRequester;
    /// use std::env;
    ///
    /// let osu_key = env::var("OSU_KEY")?;
    /// let cient = Client::new();
    ///
    /// let _ = client.get_beatmaps(osu_key, 71654)?;
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    fn get_match(&self, key: &str, match_id: u64) -> Result<Match>;

    /// Retrieves scores for a beatmap.
    ///
    /// # Examples
    ///
    /// Retrieve the scores for the beatmap with an ID of `774965`:
    ///
    /// ```rust,no_run
    /// # extern crate hyper;
    /// # extern crate osu;
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// use hyper::Client;
    /// use osu::OsuHyperRequester;
    /// use std::env;
    ///
    /// let osu_key = env::var("OSU_KEY")?;
    /// let client = Client::new();
    ///
    /// let scores = client.get_scores(osu_key, 774965, |f| f.limit(10))?;
    /// println!("There are {} scores", scores.len());
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    fn get_scores<F>(&self, key: &str, beatmap_id: u64, f: F) -> Result<Vec<GameScore>>
        where F: FnOnce(GetScoreRequest) -> GetScoreRequest;

    /// Retrieves information about a user.
    ///
    /// # Examples
    ///
    /// Retrieve information about the user with a username of `"Cookiezi"`:
    ///
    /// ```rust,no_run
    /// # extern crate hyper;
    /// # extern crate osu;
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// use hyper::Client;
    /// use osu::OsuHyperRequester;
    /// use std::env;
    ///
    /// let osu_key = env::var("OSU_KEY")?;
    /// let client = Client::new();
    ///
    /// let user = client.get_user(osu_key, "Cookiezi", |f| f)?;
    /// println!("{}'s accuracy is {}", user.username, user.accuracy);
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    fn get_user<F, U>(&self, key: &str, user: U, f: F) -> Result<Vec<User>>
        where F: FnOnce(GetUserRequest) -> GetUserRequest, U: Into<GetBeatmapUser>;

    /// Retrieves the user's best performances.
    ///
    /// # Examples
    ///
    /// Retrieve the user's 5 best performances:
    ///
    /// ```rust,no_run
    /// # extern crate hyper;
    /// # extern crate osu;
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// use hyper::Client;
    /// use osu::OsuHyperRequester;
    /// use std::env;
    ///
    /// let osu_key = env::var("OSU_KEY")?;
    /// let client = Client::new();
    /// let user = "Cookiezi";
    ///
    /// let performances = client.get_user_best(osu_key, user, |f| f
    ///     .limit(5))?;
    ///
    /// if let Some(performance) = performances.first() {
    ///     println!("{}'s best performance was on beatmap {}", user, performance.beatmap_id);
    /// } else {
    ///     println!("{} does not have a best performance", user);
    /// }
    /// ```
    fn get_user_best<F, U>(&self, key: &str, user: U, f: F) -> Result<Vec<Performance>>
        where F: FnOnce(GetUserBestRequest) -> GetUserBestRequest, U: Into<GetBeatmapUser>;

    /// Retrieves information about a user's recent plays.
    ///
    /// # Examples
    ///
    /// Retrieve a user's 10 most recent plays:
    ///
    /// ```rust,no_run
    /// # extern crate hyper;
    /// # extern crate osu;
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// use hyper::Client;
    /// use osu::OsuHyperRequester;
    /// use std::env;
    ///
    /// let osu_key = env::var("OSU_KEY")?;
    /// let client = Client::new();
    ///
    /// let recent_plays = client.get_user_recent(osu_key, "Cookiezi", |f| f
    ///     .limit(10));
    /// println!("We got {} results", recent_plays.len());
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    fn get_user_recent<F, U>(&self, key: &str, user: U, f: F) -> Result<Vec<RecentPlay>>
        where F: FnOnce(GetUserRecentRequest) -> GetUserRecentRequest, U: Into<GetBeatmapUser>;
}

impl OsuHyperRequester for Client {
    fn get_beatmaps<F>(&self, key: &str, f: F) -> Result<Vec<Beatmap>>
        where F: FnOnce(GetBeatmapsRequest) -> GetBeatmapsRequest {
        let mut uri = String::new();
        uri.push_str(API_URL);
        uri.push_str("/get_beatmaps?k=");
        uri.push_str(key);

        mutate_uri(&mut uri, f(GetBeatmapsRequest::default()).0);

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

        mutate_uri(&mut uri, f(GetScoreRequest::default()).0);

        let response = self.get(&uri).send()?;

        serde_json::from_reader(response).map_err(From::from)
    }

    fn get_user<F, U>(&self, key: &str, user: U, f: F) -> Result<Vec<User>>
        where F: FnOnce(GetUserRequest) -> GetUserRequest, U: Into<GetBeatmapUser> {
        let mut uri = String::new();
        uri.push_str(API_URL);
        uri.push_str("/get_user?k=");
        uri.push_str(key);

        mutate_uri(&mut uri, f(GetUserRequest::default()).user(user.into()).0);

        let response = self.get(&uri).send()?;

        serde_json::from_reader(response).map_err(From::from)
    }

    fn get_user_best<F, U>(&self, key: &str, user: U, f: F) -> Result<Vec<Performance>>
        where F: FnOnce(GetUserBestRequest) -> GetUserBestRequest, U: Into<GetBeatmapUser> {
        let mut uri = String::new();
        uri.push_str(API_URL);
        uri.push_str("/get_user_best?k=");
        uri.push_str(key);

        mutate_uri(&mut uri, f(GetUserBestRequest::default()).user(user.into()).0);

        let response = self.get(&uri).send()?;

        serde_json::from_reader(response).map_err(From::from)
    }

    fn get_user_recent<F, U>(&self, key: &str, user: U, f: F) -> Result<Vec<RecentPlay>>
        where F: FnOnce(GetUserRecentRequest) -> GetUserRecentRequest, U: Into<GetBeatmapUser> {
        let mut uri = String::new();
        uri.push_str(API_URL);
        uri.push_str("/get_user_recent?k=");
        uri.push_str(key);

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
