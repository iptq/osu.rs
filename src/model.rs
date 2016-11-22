use serde_json::Value;
use ::error::{Error, Result};
use ::utils::{decode_array, into_map, into_string, into_i64, opt, remove};

pub enum Approval {
    Approved,
    Graveyard,
    Loved,
    Pending,
    Qualified,
    Ranked,
    WIP,
}

map_names! { Approval;
    Graveyard "-2",
    WIP "-1",
    Pending "0",
    Ranked "1",
    Approved "2",
    Qualified "3",
    Loved "4",
}

pub enum Genre {
    Anime,
    Any,
    Electronic,
    HipHop,
    Novelty,
    Other,
    Pop,
    Rock,
    Unspecified,
    VideoGame,
}

map_names! { Genre;
    Any "0",
    Unspecified "1",
    VideoGame "2",
    Anime "3",
    Rock "4",
    Pop "5",
    Other "6",
    Novelty "7",
    HipHop "9",
    Electronic "10",
}

pub enum Language {
    Any,
    Chinese,
    English,
    French,
    Instrumental,
    German,
    Italian,
    Japanese,
    Korean,
    Spanish,
    Swedish,
    Other,
}

map_names! { Language;
    Any "0",
    Other "1",
    English "2",
    Japanese "3",
    Chinese "4",
    Instrumental "5",
    Korean "6",
    French "7",
    German "8",
    Swedish "9",
    Spanish "10",
    Italian "11",
}

pub enum PlayMode {
    CatchTheBeat,
    Mania,
    Standard,
    Taiko,
}

map_names! { PlayMode;
    Standard "0",
    Taiko "1",
    CatchTheBeat "2",
    Mania "3",
}

pub enum ScoringType {
    Accuracy,
    Combo,
    Score,
    ScoreV2,
}

map_names! { ScoringType;
    Score "0",
    Accuracy "1",
    Combo "2",
    ScoreV2 "3",
}

pub enum TeamType {
    HeadToHead,
    TagCoOp,
    TagTeamVs,
    TeamVs,
}

map_names! { TeamType;
    HeadToHead "0",
    TagCoOp "1",
    TeamVs "2",
    TagTeamVs "3",
}

pub struct Beatmap {
    pub approved: Approval,
    pub approved_date: Option<String>,
    pub artist: String,
    pub beatmap_id: i64,
    pub beatmapset_id: i64,
    pub bpm: f64,
    pub creator: String,
    pub difficulty_rating: f64,
    pub diff_size: f64,
    pub diff_overall: f64,
    pub diff_approach: f64,
    pub diff_drain: f64,
    pub favourite_count: i64,
    pub file_md5: String,
    pub genre_id: Genre,
    pub hit_length: i64,
    pub language_id: Language,
    pub last_update: String,
    pub max_combo: Option<i64>,
    pub mode: i64,
    pub pass_count: i64,
    pub play_count: i64,
    pub source: String,
    /// A list of tags, separated by spaces.
    pub tags: String,
    pub title: String,
    pub total_length: i64,
    pub version: String,
}

impl Beatmap {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Beatmap> {
        let mut map = try!(into_map(value));

        Ok(Beatmap {
            approved: field!(map, R, "approved", Approval::decode),
            approved_date: field!(map, O, "approved_date", into_string),
            artist: field!(map, R, "artist", into_string),
            beatmap_id: field!(map, int, "beatmap_id"),
            beatmapset_id: field!(map, int, "beatmapset_id"),
            bpm: field!(map, float, "bpm"),
            creator: field!(map, R, "creator", into_string),
            difficulty_rating: field!(map, float, "difficultyrating"),
            diff_approach: field!(map, float, "diff_approach"),
            diff_drain: field!(map, float, "diff_drain"),
            diff_overall: field!(map, float, "diff_overall"),
            diff_size: field!(map, float, "diff_size"),
            favourite_count: field!(map, int, "favourite_count"),
            file_md5: field!(map, R, "file_md5", into_string),
            genre_id: field!(map, R, "genre_id", Genre::decode),
            hit_length: field!(map, int, "hit_length"),
            language_id: field!(map, R, "language_id", Language::decode),
            last_update: field!(map, R, "last_update", into_string),
            max_combo: field!(map, O, int, "max_combo"),
            mode: field!(map, int, "mode"),
            pass_count: field!(map, int, "passcount"),
            play_count: field!(map, int, "playcount"),
            source: field!(map, R, "source", into_string),
            tags: field!(map, R, "tags", into_string),
            title: field!(map, R, "title", into_string),
            total_length: field!(map, int, "total_length"),
            version: field!(map, R, "version", into_string),
        })
    }
}

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

impl Game {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Game> {
        let mut map = try!(into_map(value));

        Ok(Game {
            beatmap_id: field!(map, int, "beatmap_id"),
            end_time: field!(map, R, "end_time", into_string),
            game_id: field!(map, int, "game_id"),
            match_type: field!(map, int, "match_type"),
            mods: field!(map, R, "mods", Mods::decode),
            play_mode: field!(map, R, "play_mode", PlayMode::decode),
            scores: field!(map, R, [], "scores", GameScore::decode),
            scoring_type: field!(map, R, "scoring_type", ScoringType::decode),
            start_time: field!(map, R, "start_time", into_string),
            team_type: field!(map, R, "team_type", TeamType::decode),
        })
    }
}

pub struct GameScore {
    pub count_100: i64,
    pub count_300: i64,
    pub count_50: i64,
    pub count_geki: i64,
    pub count_katu: i64,
    pub count_miss: i64,
    pub date: String,
    pub enabled_mods: Mods,
    pub max_combo: i64,
    pub perfect: i64,
    pub pp: f64,
    pub rank: String,
    pub score: i64,
    pub user_id: i64,
    pub username: String,
}

impl GameScore {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<GameScore> {
        let mut map = try!(into_map(value));

        Ok(GameScore {
            count_100: field!(map, int, "count100"),
            count_300: field!(map, int, "count300"),
            count_50: field!(map, int, "count50"),
            count_geki: field!(map, int, "countgeki"),
            count_katu: field!(map, int, "countkatu"),
            count_miss: field!(map, int, "countmiss"),
            date: field!(map, R, "date", into_string),
            enabled_mods: field!(map, R, "enabled_mods", Mods::decode),
            max_combo: field!(map, int, "maxcombo"),
            perfect: field!(map, int, "perfect"),
            pp: field!(map, float, "pp"),
            rank: field!(map, R, "rank", into_string),
            score: field!(map, int, "score"),
            user_id: field!(map, int, "user_id"),
            username: field!(map, R, "username", into_string),
        })
    }
}

pub struct Match {
    pub end_time: Option<String>,
    pub match_id: i64,
    pub name: String,
    pub start_time: String,
}

impl Match {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Match> {
        let mut map = try!(into_map(value));

        Ok(Match {
            end_time: field!(map, O, "end_time", into_string),
            match_id: field!(map, int, "match_id"),
            name: field!(map, R, "name", into_string),
            start_time: field!(map, R, "start_time", into_string),
        })
    }
}

pub struct MatchScore {
    pub count_100: i64,
    pub count_300: i64,
    pub count_50: i64,
    pub count_geki: i64,
    pub count_katu: i64,
    pub count_miss: i64,
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

impl MatchScore {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<MatchScore> {
        let mut map = try!(into_map(value));

        Ok(MatchScore {
            count_100: field!(map, int, "count100"),
            count_300: field!(map, int, "count300"),
            count_50: field!(map, int, "count50"),
            count_geki: field!(map, int, "countgeki"),
            count_katu: field!(map, int, "countkatu"),
            count_miss: field!(map, int, "countmiss"),
            max_combo: field!(map, int, "maxcombo"),
            pass: field!(map, int, "pass"),
            perfect: field!(map, int, "perfect"),
            rank: field!(map, int, "rank"),
            score: field!(map, int, "score"),
            slot: field!(map, int, "slot"),
            team: field!(map, int, "team"),
            user_id: field!(map, int, "user_id"),
        })
    }
}

pub struct Performance {
    pub beatmap_id: i64,
    pub count_100: i64,
    pub count_300: i64,
    pub count_50: i64,
    pub count_geki: i64,
    pub count_katu: i64,
    pub count_miss: i64,
    pub date: String,
    pub enabled_mods: Mods,
    pub max_combo: i64,
    pub perfect: bool,
    pub pp: f64,
    pub rank: String,
    pub score: i64,
    pub user_id: i64,
}

impl Performance {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Performance> {
        let mut map = try!(into_map(value));

        Ok(Performance {
            beatmap_id: field!(map, int, "beatmap_id"),
            count_100: field!(map, int, "count100"),
            count_300: field!(map, int, "count300"),
            count_50: field!(map, int, "count50"),
            count_geki: field!(map, int, "countgeki"),
            count_katu: field!(map, int, "countkatu"),
            count_miss: field!(map, int, "countmiss"),
            date: field!(map, R, "date", into_string),
            enabled_mods: field!(map, R, "enabled_mods", Mods::decode),
            max_combo: field!(map, int, "maxcombo"),
            perfect: field!(map, int, "perfect") > 0,
            pp: field!(map, float, "pp"),
            rank: field!(map, R, "rank", into_string),
            score: field!(map, int, "score"),
            user_id: field!(map, int, "user_id"),
        })
    }
}

pub struct RecentPlay {
    pub beatmap_id: i64,
    pub count_100: i64,
    pub count_300: i64,
    pub count_50: i64,
    pub count_geki: i64,
    pub count_katu: i64,
    pub count_miss: i64,
    pub date: String,
    pub enabled_mods: Mods,
    pub max_combo: i64,
    pub perfect: bool,
    pub rank: String,
    pub score: i64,
    pub user_id: i64,
}

impl RecentPlay {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<RecentPlay> {
        let mut map = try!(into_map(value));

        Ok(RecentPlay {
            beatmap_id: field!(map, int, "beatmap_id"),
            count_100: field!(map, int, "count100"),
            count_300: field!(map, int, "count300"),
            count_50: field!(map, int, "count50"),
            count_geki: field!(map, int, "countgeki"),
            count_katu: field!(map, int, "countkatu"),
            count_miss: field!(map, int, "countmiss"),
            date: field!(map, R, "date", into_string),
            enabled_mods: field!(map, R, "enabled_mods", Mods::decode),
            max_combo: field!(map, int, "maxcombo"),
            perfect: field!(map, int, "perfect") > 0,
            rank: field!(map, R, "rank", into_string),
            score: field!(map, int, "score"),
            user_id: field!(map, int, "user_id"),
        })
    }
}

pub struct User {
    pub id: i64,
    pub accuracy: f64,
    pub count_100: i64,
    pub count_300: i64,
    pub count_50: i64,
    pub count_rank_a: i64,
    pub count_rank_s: i64,
    pub count_rank_ss: i64,
    pub country: String,
    pub events: Vec<UserEvent>,
    pub level: f64,
    pub play_count: i64,
    pub pp_country_rank: i64,
    pub pp_rank: i64,
    pub pp_raw: f64,
    pub ranked_score: i64,
    pub total_score: i64,
    pub username: String,
}

impl User {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<User> {
        let mut map = try!(into_map(value));

        Ok(User {
            id: field!(map, int, "user_id"),
            accuracy: field!(map, float, "accuracy"),
            count_100: field!(map, int, "count100"),
            count_300: field!(map, int, "count300"),
            count_50: field!(map, int, "count50"),
            count_rank_a: field!(map, int, "count_rank_a"),
            count_rank_s: field!(map, int, "count_rank_s"),
            count_rank_ss: field!(map, int, "count_rank_ss"),
            country: field!(map, R, "country", into_string),
            events: field!(map, R, [], "events", UserEvent::decode),
            level: field!(map, float, "level"),
            play_count: field!(map, int, "playcount"),
            pp_country_rank: field!(map, int, "pp_country_rank"),
            pp_rank: field!(map, int, "pp_rank"),
            pp_raw: field!(map, float, "pp_raw"),
            ranked_score: field!(map, int, "ranked_score"),
            total_score: field!(map, int, "total_score"),
            username: field!(map, R, "username", into_string),
        })
    }
}

pub struct UserEvent {
    pub beatmap_id: i64,
    pub beatmapset_id: i64,
    pub date: String,
    pub display_html: String,
    pub epic_factor: i64,
}

impl UserEvent {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<UserEvent> {
        let mut map = try!(into_map(value));

        Ok(UserEvent {
            beatmap_id: field!(map, int, "beatmap_id"),
            beatmapset_id: field!(map, int, "beatmapset_id"),
            date: field!(map, R, "date", into_string),
            display_html: field!(map, R, "display_html", into_string),
            epic_factor: field!(map, int, "epicfactor"),
        })
    }
}

bitflags! {
    pub flags Mods: i64 {
        const NONE = 0,
        const NO_FAIL = 1 << 0,
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

impl Mods {
    #[doc(hidden)]
    pub fn decode(value: Value) -> Result<Mods> {
        let num = match value {
            Value::I64(v) => v,
            Value::U64(v) => v as i64,
            Value::String(v) => try!(v.parse::<i64>().map_err(Error::from)),
            other => panic!("Unexpected mods: {:?}", other),
        };

        Ok(Self::from_bits_truncate(num))
    }
}
