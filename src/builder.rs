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

#[derive(Default)]
pub struct GetBeatmapsRequest<'a>(pub BTreeMap<&'a str, String>);

impl<'a> GetBeatmapsRequest<'a> {
    pub fn beatmap_id(mut self, beatmap_id: u64) -> Self {
        self.0.insert("b", beatmap_id.to_string());

        GetBeatmapsRequest(self.0)
    }

    pub fn beatmap_set_id(mut self, beatmap_set_id: u64) -> Self {
        self.0.insert("s", beatmap_set_id.to_string());

        GetBeatmapsRequest(self.0)
    }

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

    pub fn mode(mut self, mode: PlayMode) -> Self {
        self.0.insert("m", mode.name().to_string());

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

#[derive(Default)]
pub struct GetScoreRequest<'a>(pub BTreeMap<&'a str, String>);

impl<'a> GetScoreRequest<'a> {
    pub fn limit(mut self, limit: u16) -> Self {
        self.0.insert("limit", limit.to_string());

        GetScoreRequest(self.0)
    }

    pub fn mode(mut self, mode: PlayMode) -> Self {
        self.0.insert("m", mode.name().to_string());

        GetScoreRequest(self.0)
    }

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

#[derive(Default)]
pub struct GetUserBestRequest<'a>(pub BTreeMap<&'a str, String>);

impl<'a> GetUserBestRequest<'a> {
    pub fn limit(mut self, limit: u16) -> Self {
        self.0.insert("limit", limit.to_string());

        GetUserBestRequest(self.0)
    }

    pub fn mode(mut self, mode: PlayMode) -> Self {
        self.0.insert("m", mode.name().to_string());

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

#[derive(Default)]
pub struct GetUserRecentRequest<'a>(pub BTreeMap<&'a str, String>);

impl<'a> GetUserRecentRequest<'a> {
    pub fn limit(mut self, limit: u16) -> Self {
        self.0.insert("limit", limit.to_string());

        GetUserRecentRequest(self.0)
    }

    pub fn mode(mut self, mode: PlayMode) -> Self {
        self.0.insert("m", mode.name().to_string());

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

#[derive(Default)]
pub struct GetUserRequest<'a>(pub BTreeMap<&'a str, String>);

impl<'a> GetUserRequest<'a> {
    pub fn event_days(mut self, event_days: u8) -> Self {
        self.0.insert("event_days", event_days.to_string());

        GetUserRequest(self.0)
    }

    pub fn mode(mut self, mode: PlayMode) -> Self {
        self.0.insert("m", mode.name().to_string());

        GetUserRequest(self.0)
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

        GetUserRequest(self.0)
    }
}
