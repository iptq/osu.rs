#![allow(unreachable_code)]
#![cfg(feature = "hyper")]

extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate osu;
extern crate tokio_core;

use futures::Future;
use hyper::client::{Client, HttpConnector};
use hyper::Body;
use hyper_tls::HttpsConnector;
use osu::bridge::hyper::OsuHyperRequester;
use std::env;
use tokio_core::reactor::Core;

const USER: &'static str = "cookiezi";

fn setup() -> (Core, Client<HttpsConnector<HttpConnector>, Body>, String) {
    let core = Core::new().unwrap();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &core.handle()).unwrap())
        .build(&core.handle());
    let key = env::var("OSU_KEY").expect("no key");

    (core, client, key)
}

#[ignore]
#[test]
fn test_get_beatmaps() {
    let (mut core, client, key) = setup();

    let done = client.get_beatmaps(&key, |f| f)
        .map(|_| {
            assert!(true);

            ()
        }).map_err(|why| {
            panic!("{:?}", why);

            ()
        });

    core.run(done).expect("core err");
}

#[ignore]
#[test]
fn test_get_scores() {
    let (mut core, client, key) = setup();
    let done = client.get_scores(&key, 191904, |f| f)
        .map(|_| {
            assert!(true);

            ()
        }).map_err(|why| {
            panic!("{:?}", why);

            ()
        });

    core.run(done).expect("core err");
}

#[ignore]
#[test]
fn test_get_user() {
    let (mut core, client, key) = setup();
    let done = client.get_user(&key, USER, |f| f)
        .map(|_| {
            assert!(true);

            ()
        }).map_err(|why| {
            panic!("{:?}", why);

            ()
        });

    core.run(done).expect("core err");
}

#[ignore]
#[test]
fn test_get_user_best() {
    let (mut core, client, key) = setup();
    let done = client.get_user_best(&key, USER, |f| f)
        .map(|_| {
            assert!(true);

            ()
        }).map_err(|why| {
            panic!("{:?}", why);

            ()
        });

    core.run(done).expect("core err");
}

#[ignore]
#[test]
fn test_get_user_recent() {
    let (mut core, client, key) = setup();
    let done = client.get_user_recent(&key, USER, |f| f)
        .map(|_| {
            assert!(true);

            ()
        }).map_err(|why| {
            panic!("{:?}", why);

            ()
        });

    core.run(done).expect("core err");
}
