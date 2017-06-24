//! A set of builders for use with some request functions that have multiple
//! optional parameters.

use serde_json;
use std::collections::BTreeMap;
use super::GetBeatmapUser;
use ::model::*;

enum GetBeatmapType {
    /// Use for user_ids.
    Id,
    /// Use for usernames.
    String,
}

impl GetBeatmapType {
    pub fn name(&self) -> &str {
        match *self {
            GetBeatmapType::Id => "id",
            GetBeatmapType::String => "string",
        }
    }
}

/// A builder used in conjunction with [`OsuRequester::get_beatmaps`] for
/// optional parameters.
///
/// [`OsuRequester::get_beatmaps`]: trait.OsuRequester.html#method.get_beatmaps
#[derive(Default)]
pub struct GetBeatmapsRequest<'a>(pub BTreeMap<&'a str, String>);

impl<'a> GetBeatmapsRequest<'a> {
    /// Specify the beatmap ID to filter by.
    pub fn beatmap_id(mut self, beatmap_id: u64) -> Self {
        self.0.insert("b", beatmap_id.to_string());

        GetBeatmapsRequest(self.0)
    }

    /// Specify the beatmap set ID to filter by.
    pub fn beatmap_set_id(mut self, beatmap_set_id: u64) -> Self {
        self.0.insert("s", beatmap_set_id.to_string());

        GetBeatmapsRequest(self.0)
    }

    /// Specify the hash to filter by.
    pub fn hash(mut self, hash: &str) -> Self {
        self.0.insert("h", hash.to_owned());

        GetBeatmapsRequest(self.0)
    }

    /// Specify whether converted beatmaps are included.
    ///
    /// Only has an effect if the [`mode`] is chosen and not [`Standard`].
    ///
    /// [`Standard`]: ../model/enum.PlayMode.html#variant.Standard
    /// [`mode`]: #method.mode
    pub fn include_converted(mut self, include_converted: bool) -> Self {
        self.0.insert("a", include_converted.to_string());

        GetBeatmapsRequest(self.0)
    }

    /// The amount of results to return.
    ///
    /// Defaults to `500`. Maximum is `500`.
    pub fn limit(mut self, limit: u16) -> Self {
        self.0.insert("limit", limit.to_string());

        GetBeatmapsRequest(self.0)
    }

    /// Specify the mode to filter by.
    ///
    /// Refer to [`GetUserRequest::mode`] for examples.
    ///
    /// [`GetUserRequest::mode`]: struct.GetUserRequest.html#method.mode
    pub fn mode(mut self, mode: PlayMode) -> Self {
        if let Ok(mode) = serde_json::to_string(&mode) {
            self.0.insert("m", mode);
        }

        GetBeatmapsRequest(self.0)
    }

    /// Return all beatmaps ranked since the given date. Must be a MySQL date.
    pub fn since(mut self, since: &str) -> Self {
        self.0.insert("since", since.to_owned());

        GetBeatmapsRequest(self.0)
    }

    /// Specify the user to retrieve a beatmap for. Pass in a u64 to search by
    /// Id, or a string to search by username.
    ///
    /// This will automatically set the 'type' based on what is passed.
    pub fn user<U: Into<GetBeatmapUser>>(mut self, user: U) -> Self {
        match user.into() {
            GetBeatmapUser::UserId(id) => {
                self.0.insert("u", id.to_string());
                self.0.insert("type", GetBeatmapType::Id.name().to_owned());
            },
            GetBeatmapUser::Username(name) => {
                self.0.insert("u", name);
                self.0.insert("type", GetBeatmapType::String.name().to_owned());
            }
        }

        GetBeatmapsRequest(self.0)
    }
}

/// A builder used in conjunction with [`OsuRequester::get_scores`] for
/// optional parameters.
///
/// [`OsuRequester::get_scores`]: trait.OsuRequester.html#method.get_scores
#[derive(Default)]
pub struct GetScoreRequest<'a>(pub BTreeMap<&'a str, String>);

impl<'a> GetScoreRequest<'a> {
    /// Specify the number of performances to retrieve at maximum.
    ///
    /// Refer to [`GetUserRequest::limit`] for examples.
    ///
    /// [`GetUserRequest::limit`]: struct.GetUserRequest.html#method.limit
    pub fn limit(mut self, limit: u16) -> Self {
        self.0.insert("limit", limit.to_string());

        GetScoreRequest(self.0)
    }

    /// Specify the mode to filter by.
    ///
    /// Refer to [`GetUserRequest::mode`] for examples.
    ///
    /// [`GetUserRequest::mode`]: struct.GetUserRequest.html#method.mode
    pub fn mode(mut self, mode: PlayMode) -> Self {
        if let Ok(mode) = serde_json::to_string(&mode) {
            self.0.insert("m", mode);
        }

        GetScoreRequest(self.0)
    }

    /// Filters results by scores with certain Mods enabled.
    ///
    /// # Examples
    ///
    /// Filter beatmap score results by , limiting by 5 results:
    ///
    /// ```rust,no_run
    /// # extern crate hyper;
    /// # extern crate osu;
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// use hyper::Client;
    /// use osu::{Mods, OsuRequester};
    /// use std::env;
    ///
    /// let osu_key = env::var("OSU_KEY")?;
    /// let client = Client::new();
    /// let beatmap_id = 15412;
    ///
    /// let _ = client.get_scores(osu_key, beatmap_id, |f| f
    ///     .limit(5)
    ///     .mods(1))?;
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn mods(mut self, mods: Mods) -> Self {
        self.0.insert("mods", mods.bits().to_string());

        GetScoreRequest(self.0)
    }

    /// Specify the user to retrieve a beatmap's top scores for. Pass in a u64
    /// to search by Id, or a string to search by username.
    ///
    /// This will automatically set the 'type' based on what is passed.
    pub fn user<U: Into<GetBeatmapUser>>(mut self, user: U) -> Self {
        match user.into() {
            GetBeatmapUser::UserId(id) => {
                self.0.insert("u", id.to_string());
                self.0.insert("type", GetBeatmapType::Id.name().to_owned());
            },
            GetBeatmapUser::Username(name) => {
                self.0.insert("u", name);
                self.0.insert("type", GetBeatmapType::String.name().to_owned());
            }
        }

        GetScoreRequest(self.0)
    }
}

/// A builder used in conjunction with [`OsuRequester::get_user_best`] for
/// optional parameters.
///
/// [`OsuRequester::get_user_best`]: trait.OsuRequester.html#method.get_user_best
#[derive(Default)]
pub struct GetUserBestRequest<'a>(pub BTreeMap<&'a str, String>);

impl<'a> GetUserBestRequest<'a> {
    /// Specify the number of best performances to retrieve at maximum.
    ///
    /// # Examples
    ///
    /// Retrieve `10` of the best performances - at most - for the user with an
    /// ID of `8`:
    ///
    /// ```rust,no_run
    /// # extern crate hyper;
    /// # extern crate osu;
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// use hyper::Client;
    /// use osu::OsuRequester;
    /// use std::env;
    ///
    /// let osu_key = env::var("OSU_KEY")?;
    /// let client = Client::new();
    ///
    /// let _ = client.get_user_best(osu_key, 8, |f| f.limit(10))?;
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn limit(mut self, limit: u16) -> Self {
        self.0.insert("limit", limit.to_string());

        GetUserBestRequest(self.0)
    }

    /// Specify the mode to filter a user's performances by.
    ///
    /// # Examples
    ///
    /// Retrieve a user's best performances, filtered by [`PlayMode::Mania`]:
    ///
    /// ```rust,no_run
    /// # extern crate hyper;
    /// # extern crate osu;
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// use hyper::Client;
    /// use osu::{OsuRequester, PlayMode};
    /// use std::env;
    ///
    /// let osu_key = env::var("OSU_KEY")?;
    /// let client = Client::new();
    ///
    /// let _ = client.get_user_best(osu_key, 1, |f| f.mode(PlayMode::Mania))?;
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn mode(mut self, mode: PlayMode) -> Self {
        if let Ok(mode) = serde_json::to_string(&mode) {
            self.0.insert("m", mode);
        }

        GetUserBestRequest(self.0)
    }

    /// Specify the user to retrieve top scores for. Pass in a u64 to search by
    /// Id, or a string to search by username.
    ///
    /// This will automatically set the 'type' based on what is passed.
    #[doc(hidden)]
    pub fn user<U: Into<GetBeatmapUser>>(mut self, user: U) -> Self {
        match user.into() {
            GetBeatmapUser::UserId(id) => {
                self.0.insert("u", id.to_string());
                self.0.insert("type", GetBeatmapType::Id.name().to_owned());
            },
            GetBeatmapUser::Username(name) => {
                self.0.insert("u", name);
                self.0.insert("type", GetBeatmapType::String.name().to_owned());
            }
        }

        GetUserBestRequest(self.0)
    }
}

/// A builder used in conjunction with [`OsuRequester::get_user_recent`] for
/// optional parameters.
///
/// [`OsuRequester::get_user_recent`]: trait.OsuRequester.html#method.get_user_recent
#[derive(Default)]
pub struct GetUserRecentRequest<'a>(pub BTreeMap<&'a str, String>);

impl<'a> GetUserRecentRequest<'a> {
    /// Specify the number of recent plays to retrieve at maximum.
    ///
    /// # Examples
    ///
    /// Retrieve `10` of the recent plays - at most - for the user with an ID of
    /// `8`:
    ///
    /// ```rust,no_run
    /// # extern crate hyper;
    /// # extern crate osu;
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// use hyper::Client;
    /// use osu::OsuRequester;
    /// use std::env;
    ///
    /// let osu_key = env::var("OSU_KEY")?;
    /// let client = Client::new();
    ///
    /// let _ = client.get_user_recent(osu_key, 8, |f| f.limit(10))?;
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn limit(mut self, limit: u16) -> Self {
        self.0.insert("limit", limit.to_string());

        GetUserRecentRequest(self.0)
    }

    /// Specify the mode to filter by.
    ///
    /// Refer to [`GetUserRequest::mode`] for examples.
    ///
    /// [`GetUserRequest::mode`]: struct.GetUserRequest.html#method.mode
    pub fn mode(mut self, mode: PlayMode) -> Self {
        if let Ok(mode) = serde_json::to_string(&mode) {
            self.0.insert("m", mode);
        }

        GetUserRecentRequest(self.0)
    }

    /// Specify the user to retrieve data for. Pass in a u64 to search by
    /// Id, or a string to search by username.
    ///
    /// This will automatically set the 'type' based on what is passed.
    #[doc(hidden)]
    pub fn user<U: Into<GetBeatmapUser>>(mut self, user: U) -> Self {
        match user.into() {
            GetBeatmapUser::UserId(id) => {
                self.0.insert("u", id.to_string());
                self.0.insert("type", GetBeatmapType::Id.name().to_owned());
            },
            GetBeatmapUser::Username(name) => {
                self.0.insert("u", name);
                self.0.insert("type", GetBeatmapType::String.name().to_owned());
            }
        }

        GetUserRecentRequest(self.0)
    }
}

/// A builder used in conjunction with [`OsuRequester::get_user`] for optional
/// parameters.
///
/// [`OsuRequester::get_user`]: trait.OsuRequester.html#method.get_user
#[derive(Default)]
pub struct GetUserRequest<'a>(pub BTreeMap<&'a str, String>);

impl<'a> GetUserRequest<'a> {
    /// Specify the number of event days to filter by.
    ///
    /// # Examples
    ///
    /// Retrieve a user, filtering by 3 event days:
    ///
    /// ```rust,no_run
    /// # extern crate hyper;
    /// # extern crate osu;
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// use hyper::Client;
    /// use osu::OsuRequester;
    /// use std::env;
    ///
    /// let osu_key = env::var("OSU_KEY")?;
    /// let client = Client::new();
    /// let user_id = 17;
    ///
    /// let _ = client.get_user(osu_key, user_id, 3)?;
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn event_days(mut self, event_days: u8) -> Self {
        self.0.insert("event_days", event_days.to_string());

        GetUserRequest(self.0)
    }

    /// Specify the mode to filter results by.
    ///
    /// # Examples
    ///
    /// Retrieve a user, filtered by [`PlayMode::Mania`]:
    ///
    /// ```rust,no_run
    /// # extern crate hyper;
    /// # extern crate osu;
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// use hyper::Client;
    /// use osu::{OsuRequester, PlayMode};
    /// use std::env;
    ///
    /// let osu_key = env::var("OSU_KEY")?;
    /// let client = Client::new();
    ///
    /// let _ = client.get_user(osu_key, 1, |f| f.mode(PlayMode::Mania))?;
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn mode(mut self, mode: PlayMode) -> Self {
        if let Ok(mode) = serde_json::to_string(&mode) {
            self.0.insert("m", mode);
        }

        GetUserRequest(self.0)
    }

    /// Specify the user to retrieve data for. Pass in a u64 to search by
    /// Id, or a string to search by username.
    ///
    /// This will automatically set the 'type' based on what is passed.
    ///
    /// # Examples
    ///
    /// Retrieve a user by their ID:
    ///
    /// ```rust,no_run
    /// # extern crate hyper;
    /// # extern crate osu;
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// use hyper::Client;
    /// use osu::OsuRequester;
    /// use std::env;
    ///
    /// let osu_key = env::var("OSU_KEY")?;
    /// let client = Client::new();
    /// let user = "Cookiezi";
    ///
    /// let _ = client.get_user(osu_key, user, |f| f.user(user))?;
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    #[doc(hidden)]
    pub fn user<U: Into<GetBeatmapUser>>(mut self, user: U) -> Self {
        match user.into() {
            GetBeatmapUser::UserId(id) => {
                self.0.insert("u", id.to_string());
                self.0.insert("type", GetBeatmapType::Id.name().to_owned());
            },
            GetBeatmapUser::Username(name) => {
                self.0.insert("u", name);
                self.0.insert("type", GetBeatmapType::String.name().to_owned());
            }
        }

        GetUserRequest(self.0)
    }
}
