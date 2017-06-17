extern crate hyper;
extern crate hyper_native_tls;
extern crate osu;

use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use std::env;
use osu::*;

const USER: &'static str = "cookiezi";

fn client() -> Client {
    let tls = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(tls);

    Client::with_connector(connector)
}

#[ignore]
#[test]
fn test_get_beatmaps() {
    let key = env::var("OSU_KEY").unwrap();
    let client = client();
    let _ = client.get_beatmaps(&key, |f| f).expect("get beatmaps");
}

#[ignore]
#[test]
fn test_get_scores() {
    let key = env::var("OSU_KEY").unwrap();
    let client = client();
    let _ = client.get_scores(&key, 191904, |f| f).expect("get scores");
}

#[ignore]
#[test]
fn test_get_user() {
    let key = env::var("OSU_KEY").unwrap();
    let client = client();
    let _ = client.get_user(&key, USER, |f| f).expect("get user");
}

#[ignore]
#[test]
fn test_get_user_best() {
    let key = env::var("OSU_KEY").unwrap();
    let client = client();
    let _ = client.get_user_best(&key, USER, |f| f).expect("get user best");
}

#[ignore]
#[test]
fn test_get_user_recent() {
    let key = env::var("OSU_KEY").unwrap();
    let client = client();
    let _ = client.get_user_recent(&key, USER, |f| f).expect("get user recent");
}
