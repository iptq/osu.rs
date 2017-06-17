extern crate osu;
extern crate serde_json;

use osu::*;
use std::fs::File;

#[test]
fn test_beatmaps() {
    let f = File::open("./tests/resources/beatmaps_01.json").unwrap();
    let _ = serde_json::from_reader::<File, Vec<Beatmap>>(f).unwrap();
}

#[test]
fn test_performances() {
    let f = File::open("./tests/resources/performances_01.json").unwrap();
    let _ = serde_json::from_reader::<File, Vec<Performance>>(f).unwrap();
}

#[test]
fn test_user() {
    let f = File::open("./tests/resources/user_01.json").unwrap();
    let _ = serde_json::from_reader::<File, User>(f).unwrap();
}

#[test]
fn test_user_best() {
    let f = File::open("./tests/resources/user_best_01.json").unwrap();
    let _ = serde_json::from_reader::<File, Vec<Performance>>(f).unwrap();
}

#[test]
fn test_user_recent() {
    let f = File::open("./tests/resources/user_recent_01.json").unwrap();
    let _ = serde_json::from_reader::<File, Vec<RecentPlay>>(f).unwrap();
}
