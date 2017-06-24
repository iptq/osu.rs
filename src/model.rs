#![allow(missing_docs)]

use serde::de::{Deserialize, Deserializer, Error as DeError};
use std::fmt::Display;
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum Approval {
    #[serde(rename="2")]
    Approved,
    #[serde(rename="-2")]
    Graveyard,
    #[serde(rename="4")]
    Loved,
    #[serde(rename="0")]
    Pending,
    #[serde(rename="3")]
    Qualified,
    #[serde(rename="1")]
    Ranked,
    #[serde(rename="-1")]
    WIP,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum Genre {
    #[serde(rename="3")]
    Anime,
    #[serde(rename="0")]
    Any,
    #[serde(rename="10")]
    Electronic,
    #[serde(rename="9")]
    HipHop,
    #[serde(rename="7")]
    Novelty,
    #[serde(rename="6")]
    Other,
    #[serde(rename="5")]
    Pop,
    #[serde(rename="4")]
    Rock,
    #[serde(rename="1")]
    Unspecified,
    #[serde(rename="2")]
    VideoGame,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum Language {
    #[serde(rename="0")]
    Any,
    #[serde(rename="4")]
    Chinese,
    #[serde(rename="2")]
    English,
    #[serde(rename="7")]
    French,
    #[serde(rename="5")]
    Instrumental,
    #[serde(rename="8")]
    German,
    #[serde(rename="11")]
    Italian,
    #[serde(rename="3")]
    Japanese,
    #[serde(rename="6")]
    Korean,
    #[serde(rename="10")]
    Spanish,
    #[serde(rename="9")]
    Swedish,
    #[serde(rename="1")]
    Other,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum PlayMode {
    #[serde(rename="2")]
    CatchTheBeat,
    #[serde(rename="3")]
    Mania,
    #[serde(rename="0")]
    Standard,
    #[serde(rename="1")]
    Taiko,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum ScoringType {
    #[serde(rename="1")]
    Accuracy,
    #[serde(rename="2")]
    Combo,
    #[serde(rename="0")]
    Score,
    #[serde(rename="3")]
    ScoreV2,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum TeamType {
    #[serde(rename="0")]
    HeadToHead,
    #[serde(rename="1")]
    TagCoOp,
    #[serde(rename="3")]
    TagTeamVs,
    #[serde(rename="2")]
    TeamVs,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Beatmap {
    pub approved: Approval,
    pub approved_date: Option<String>,
    pub artist: String,
    #[serde(deserialize_with="from_str")]
    pub beatmap_id: i64,
    #[serde(deserialize_with="from_str")]
    pub beatmapset_id: i64,
    #[serde(deserialize_with="from_str")]
    pub bpm: f64,
    pub creator: String,
    #[serde(deserialize_with="from_str", rename="difficultyrating")]
    pub difficulty_rating: f64,
    #[serde(deserialize_with="from_str")]
    pub diff_approach: f64,
    #[serde(deserialize_with="from_str")]
    pub diff_drain: f64,
    #[serde(deserialize_with="from_str")]
    pub diff_overall: f64,
    #[serde(deserialize_with="from_str")]
    pub diff_size: f64,
    #[serde(deserialize_with="from_str")]
    pub favourite_count: i64,
    pub file_md5: String,
    pub genre_id: Genre,
    #[serde(deserialize_with="from_str")]
    pub hit_length: i64,
    pub language_id: Language,
    pub last_update: String,
    #[serde(deserialize_with="from_optional_str")]
    pub max_combo: Option<i64>,
    #[serde(deserialize_with="from_str")]
    pub mode: i64,
    #[serde(deserialize_with="from_str", rename="passcount")]
    pub pass_count: i64,
    #[serde(deserialize_with="from_str", rename="playcount")]
    pub play_count: i64,
    pub source: String,
    /// A list of tags, separated by spaces.
    pub tags: String,
    pub title: String,
    #[serde(deserialize_with="from_str")]
    pub total_length: i64,
    pub version: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Game {
    #[serde(deserialize_with="from_str")]
    pub beatmap_id: i64,
    pub end_time: String,
    #[serde(deserialize_with="from_str")]
    pub game_id: i64,
    #[serde(deserialize_with="from_str")]
    pub match_type: i64,
    pub mods: Mods,
    pub play_mode: PlayMode,
    pub scores: Vec<GameScore>,
    pub scoring_type: ScoringType,
    pub start_time: String,
    pub team_type: TeamType,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GameScore {
    #[serde(deserialize_with="from_str", rename="count100")]
    pub count_100: i64,
    #[serde(deserialize_with="from_str", rename="count300")]
    pub count_300: i64,
    #[serde(deserialize_with="from_str", rename="count50")]
    pub count_50: i64,
    #[serde(deserialize_with="from_str", rename="countgeki")]
    pub count_geki: i64,
    #[serde(deserialize_with="from_str", rename="countkatu")]
    pub count_katu: i64,
    #[serde(deserialize_with="from_str", rename="countmiss")]
    pub count_miss: i64,
    pub date: String,
    pub enabled_mods: Mods,
    #[serde(deserialize_with="from_str", rename="maxcombo")]
    pub max_combo: i64,
    #[serde(deserialize_with="from_str")]
    pub perfect: i64,
    #[serde(deserialize_with="from_str")]
    pub pp: f64,
    pub rank: String,
    #[serde(deserialize_with="from_str")]
    pub score: i64,
    #[serde(deserialize_with="from_str")]
    pub user_id: i64,
    pub username: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Match {
    pub end_time: Option<String>,
    #[serde(deserialize_with="from_str")]
    pub match_id: i64,
    pub name: String,
    pub start_time: String,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct MatchScore {
    #[serde(deserialize_with="from_str", rename="count100")]
    pub count_100: i64,
    #[serde(deserialize_with="from_str", rename="count300")]
    pub count_300: i64,
    #[serde(deserialize_with="from_str", rename="count50")]
    pub count_50: i64,
    #[serde(deserialize_with="from_str", rename="countgeki")]
    pub count_geki: i64,
    #[serde(deserialize_with="from_str", rename="countkatu")]
    pub count_katu: i64,
    #[serde(deserialize_with="from_str", rename="countmiss")]
    pub count_miss: i64,
    #[serde(deserialize_with="from_str", rename="maxcombo")]
    pub max_combo: i64,
    #[serde(deserialize_with="from_str")]
    pub pass: i64,
    #[serde(deserialize_with="from_str")]
    pub perfect: i64,
    // Not used. Always 0.
    #[serde(deserialize_with="from_str")]
    pub rank: i64,
    #[serde(deserialize_with="from_str")]
    pub score: i64,
    #[serde(deserialize_with="from_str")]
    pub slot: i64,
    #[serde(deserialize_with="from_str")]
    pub team: i64,
    #[serde(deserialize_with="from_str")]
    pub user_id: i64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Performance {
    #[serde(deserialize_with="from_str")]
    pub beatmap_id: i64,
    #[serde(deserialize_with="from_str", rename="count100")]
    pub count_100: i64,
    #[serde(deserialize_with="from_str", rename="count300")]
    pub count_300: i64,
    #[serde(deserialize_with="from_str", rename="count50")]
    pub count_50: i64,
    #[serde(deserialize_with="from_str", rename="countgeki")]
    pub count_geki: i64,
    #[serde(deserialize_with="from_str", rename="countkatu")]
    pub count_katu: i64,
    #[serde(deserialize_with="from_str", rename="countmiss")]
    pub count_miss: i64,
    pub date: String,
    pub enabled_mods: Mods,
    #[serde(deserialize_with="from_str", rename="maxcombo")]
    pub max_combo: i64,
    #[serde(deserialize_with="from_str_bool")]
    pub perfect: bool,
    #[serde(deserialize_with="from_str")]
    pub pp: f64,
    pub rank: String,
    #[serde(deserialize_with="from_str")]
    pub score: i64,
    #[serde(deserialize_with="from_str")]
    pub user_id: i64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RecentPlay {
    #[serde(deserialize_with="from_str")]
    pub beatmap_id: i64,
    #[serde(deserialize_with="from_str", rename="count100")]
    pub count_100: i64,
    #[serde(deserialize_with="from_str", rename="count300")]
    pub count_300: i64,
    #[serde(deserialize_with="from_str", rename="count50")]
    pub count_50: i64,
    #[serde(deserialize_with="from_str", rename="countgeki")]
    pub count_geki: i64,
    #[serde(deserialize_with="from_str", rename="countkatu")]
    pub count_katu: i64,
    #[serde(deserialize_with="from_str", rename="countmiss")]
    pub count_miss: i64,
    pub date: String,
    pub enabled_mods: Mods,
    #[serde(deserialize_with="from_str", rename="maxcombo")]
    pub max_combo: i64,
    #[serde(deserialize_with="from_str_bool")]
    pub perfect: bool,
    pub rank: String,
    #[serde(deserialize_with="from_str")]
    pub score: i64,
    #[serde(deserialize_with="from_str")]
    pub user_id: i64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct User {
    #[serde(deserialize_with="from_str", rename="user_id")]
    pub id: i64,
    #[serde(deserialize_with="from_str")]
    pub accuracy: f64,
    #[serde(deserialize_with="from_str", rename="count100")]
    pub count_100: i64,
    #[serde(deserialize_with="from_str", rename="count300")]
    pub count_300: i64,
    #[serde(deserialize_with="from_str", rename="count50")]
    pub count_50: i64,
    #[serde(deserialize_with="from_str")]
    pub count_rank_a: i64,
    #[serde(deserialize_with="from_str")]
    pub count_rank_s: i64,
    #[serde(deserialize_with="from_str")]
    pub count_rank_ss: i64,
    pub country: String,
    pub events: Vec<UserEvent>,
    #[serde(deserialize_with="from_str")]
    pub level: f64,
    #[serde(deserialize_with="from_str", rename="playcount")]
    pub play_count: i64,
    #[serde(deserialize_with="from_str")]
    pub pp_country_rank: i64,
    #[serde(deserialize_with="from_str")]
    pub pp_rank: i64,
    #[serde(deserialize_with="from_str")]
    pub pp_raw: f64,
    #[serde(deserialize_with="from_str")]
    pub ranked_score: i64,
    #[serde(deserialize_with="from_str")]
    pub total_score: i64,
    pub username: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UserEvent {
    #[serde(deserialize_with="from_str")]
    pub beatmap_id: i64,
    #[serde(deserialize_with="from_str")]
    pub beatmapset_id: i64,
    pub date: String,
    pub display_html: String,
    #[serde(deserialize_with="from_str", rename="epicfactor")]
    pub epic_factor: i64,
}

bitflags! {
    pub struct Mods: i64 {
        const NONE = 0;
        const NO_FAIL = 1;
        const EASY = 1 << 1;
        const NO_VIDEO = 1 << 2;
        const HIDDEN = 1 << 3;
        const HARD_ROCK = 1 << 4;
        const SUDDEN_DEATH = 1 << 5;
        const DOUBLE_TIME = 1 << 6;
        const RELAX = 1 << 7;
        const HALF_TIME = 1 << 8;
        /// Only set along with [`DOUBLE_TIME`]. i.e.: NC only gives 576
        ///
        /// [`DOUBLE_TIME`]: constant.DOUBLE_TIME.html
        const NIGHTCORE = 1 << 9;
        const FLASHLIGHT = 1 << 10;
        const AUTOPLAY = 1 << 11;
        const SPUN_OUT = 1 << 12;
        /// Could be "Autopilot".
        const RELAX2 = 1 << 13;
        const PERFECT = 1 << 14;
        const KEY4 = 1 << 15;
        const KEY5 = 1 << 16;
        const KEY6 = 1 << 17;
        const KEY7 = 1 << 18;
        const KEY8 = 1 << 19;
        const KEY_MOD = 1015808;
        const FADE_IN = 1 << 20;
        const RANDOM = 1 << 21;
        const LAST_MOD = 1 << 22;
        const FREE_MOD_ALLOWED = 2069691;
        const KEY9 = 1 << 24;
        const KEY10 = 1 << 25;
        const KEY1 = 1 << 26;
        const KEY2 = 1 << 27;
        const KEY3 = 1 << 28;
    }
}

impl<'de> Deserialize<'de> for Mods {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> StdResult<Self, D::Error> {
        let num: i64 = from_str(deserializer)?;

        Ok(Mods::from_bits_truncate(num))
    }
}

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where T: FromStr,
          T::Err: Display,
          D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;

    T::from_str(&s).map_err(DeError::custom)
}

fn from_optional_str<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where T: FromStr,
          T::Err: Display,
          D: Deserializer<'de> {
    Ok(String::deserialize(deserializer).ok().and_then(|x| T::from_str(&x).ok()))
}

fn from_str_bool<'de, D>(deserializer: D) -> Result<bool, D::Error> where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;

    match &*s {
        "0" => Ok(false),
        "1" => Ok(true),
        other => Err(DeError::custom(other)),
    }
}
