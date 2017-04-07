use serde::de::{Deserialize, Deserializer};
use std::result::Result as StdResult;

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
    pub beatmap_id: i64,
    pub beatmapset_id: i64,
    pub bpm: f64,
    pub creator: String,
    #[serde(rename="difficultyrating")]
    pub difficulty_rating: f64,
    pub diff_approach: f64,
    pub diff_drain: f64,
    pub diff_overall: f64,
    pub diff_size: f64,
    pub favourite_count: i64,
    pub file_md5: String,
    pub genre_id: Genre,
    pub hit_length: i64,
    pub language_id: Language,
    pub last_update: String,
    pub max_combo: Option<i64>,
    pub mode: i64,
    #[serde(rename="passcount")]
    pub pass_count: i64,
    #[serde(rename="playcount")]
    pub play_count: i64,
    pub source: String,
    /// A list of tags, separated by spaces.
    pub tags: String,
    pub title: String,
    pub total_length: i64,
    pub version: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Game {
    pub beatmap_id: i64,
    pub end_time: String,
    pub game_id: i64,
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
    #[serde(rename="count100")]
    pub count_100: i64,
    #[serde(rename="count300")]
    pub count_300: i64,
    #[serde(rename="count50")]
    pub count_50: i64,
    #[serde(rename="countgeki")]
    pub count_geki: i64,
    #[serde(rename="countkatu")]
    pub count_katu: i64,
    #[serde(rename="countmiss")]
    pub count_miss: i64,
    pub date: String,
    pub enabled_mods: Mods,
    #[serde(rename="maxcombo")]
    pub max_combo: i64,
    pub perfect: i64,
    pub pp: f64,
    pub rank: String,
    pub score: i64,
    pub user_id: i64,
    pub username: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Match {
    pub end_time: Option<String>,
    pub match_id: i64,
    pub name: String,
    pub start_time: String,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct MatchScore {
    #[serde(rename="count100")]
    pub count_100: i64,
    #[serde(rename="count300")]
    pub count_300: i64,
    #[serde(rename="count50")]
    pub count_50: i64,
    #[serde(rename="countgeki")]
    pub count_geki: i64,
    #[serde(rename="countkatu")]
    pub count_katu: i64,
    #[serde(rename="countmiss")]
    pub count_miss: i64,
    #[serde(rename="maxcombo")]
    pub max_combo: i64,
    pub pass: i64,
    pub perfect: i64,
    // Not used. Always 0.
    pub rank: i64,
    pub score: i64,
    pub slot: i64,
    pub team: i64,
    pub user_id: i64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Performance {
    pub beatmap_id: i64,
    #[serde(rename="count100")]
    pub count_100: i64,
    #[serde(rename="count300")]
    pub count_300: i64,
    #[serde(rename="count50")]
    pub count_50: i64,
    #[serde(rename="countgeki")]
    pub count_geki: i64,
    #[serde(rename="countkatu")]
    pub count_katu: i64,
    #[serde(rename="countmiss")]
    pub count_miss: i64,
    pub date: String,
    pub enabled_mods: Mods,
    #[serde(rename="maxcombo")]
    pub max_combo: i64,
    pub perfect: bool,
    pub pp: f64,
    pub rank: String,
    pub score: i64,
    pub user_id: i64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RecentPlay {
    pub beatmap_id: i64,
    #[serde(rename="count100")]
    pub count_100: i64,
    #[serde(rename="count300")]
    pub count_300: i64,
    #[serde(rename="count50")]
    pub count_50: i64,
    #[serde(rename="countgeki")]
    pub count_geki: i64,
    #[serde(rename="countkatu")]
    pub count_katu: i64,
    #[serde(rename="countmiss")]
    pub count_miss: i64,
    pub date: String,
    pub enabled_mods: Mods,
    #[serde(rename="maxcombo")]
    pub max_combo: i64,
    pub perfect: bool,
    pub rank: String,
    pub score: i64,
    pub user_id: i64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct User {
    pub id: i64,
    pub accuracy: f64,
    #[serde(rename="count100")]
    pub count_100: i64,
    #[serde(rename="count300")]
    pub count_300: i64,
    #[serde(rename="count50")]
    pub count_50: i64,
    pub count_rank_a: i64,
    pub count_rank_s: i64,
    pub count_rank_ss: i64,
    pub country: String,
    pub events: Vec<UserEvent>,
    pub level: f64,
    #[serde(rename="playcount")]
    pub play_count: i64,
    pub pp_country_rank: i64,
    pub pp_rank: i64,
    pub pp_raw: f64,
    pub ranked_score: i64,
    pub total_score: i64,
    pub username: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UserEvent {
    pub beatmap_id: i64,
    pub beatmapset_id: i64,
    pub date: String,
    pub display_html: String,
    pub epic_factor: i64,
}

bitflags! {
    pub flags Mods: i64 {
        const NONE = 0,
        const NO_FAIL = 1,
        const EASY = 1 << 1,
        const NO_VIDEO = 1 << 2,
        const HIDDEN = 1 << 3,
        const HARD_ROCK = 1 << 4,
        const SUDDEN_DEATH = 1 << 5,
        const DOUBLE_TIME = 1 << 6,
        const RELAX = 1 << 7,
        const HALF_TIME = 1 << 8,
        /// Only set along with [`DOUBLE_TIME`]. i.e.: NC only gives 576
        ///
        /// [`DOUBLE_TIME`]: constant.DOUBLE_TIME.html
        const NIGHTCORE = 1 << 9,
        const FLASHLIGHT = 1 << 10,
        const AUTOPLAY = 1 << 11,
        const SPUN_OUT = 1 << 12,
        /// Could be "Autopilot".
        const RELAX2 = 1 << 13,
        const PERFECT = 1 << 14,
        const KEY4 = 1 << 15,
        const KEY5 = 1 << 16,
        const KEY6 = 1 << 17,
        const KEY7 = 1 << 18,
        const KEY8 = 1 << 19,
        const KEY_MOD = 1015808,
        const FADE_IN = 1 << 20,
        const RANDOM = 1 << 21,
        const LAST_MOD = 1 << 22,
        const FREE_MOD_ALLOWED = 2069691,
        const KEY9 = 1 << 24,
        const KEY10 = 1 << 25,
        const KEY1 = 1 << 26,
        const KEY2 = 1 << 27,
        const KEY3 = 1 << 28,
    }
}

impl Deserialize for Mods {
    fn deserialize<D: Deserializer>(deserializer: D) -> StdResult<Self, D::Error> {
        Ok(Mods::from_bits_truncate(i64::deserialize(deserializer)?))
    }
}
