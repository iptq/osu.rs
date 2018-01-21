[![ci-badge][]][ci] [![license-badge][]][license] [![docs-badge][]][docs]

# osu.rs

Unofficial Rust crate for the osu! API.

[Documentation][docs]

### Installation

Add the following dependency to your Cargo.toml:

```toml
osu = "0.2"
```

And include it in your project:

```rust
extern crate osu;
```

### Examples

Using `hyper` with the `hyper-tls` HTTPS connector, retrieve the start time of a
match by ID:

```rust
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate osu;
extern crate tokio_core;

use futures::Future;
use hyper::client::{Client, HttpConnector};
use hyper_tls::HttpsConnector;
use osu::bridge::hyper::OsuHyperRequester;
use std::error::Error;
use std::env;
use tokio_core::reactor::Core;

fn try_main() -> Result<(), Box<Error>> {
    let mut core = Core::new()?;
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &core.handle())?)
        .build(&core.handle());
    let key = env::var("OSU_KEY")?;

    let done = client.get_match(&key, 71641).map(|match| {
        println!("Match start time: {}", match.start_time);

        ()
    }).map_err(|_| ());

    core.run(done).expect("Error running core");

    Ok(())
}

fn main() {
    try_main().unwrap();
}
```

Using reqwest, retrieve a match's start time by ID:

```rust
extern crate osu;
extern crate reqwest;

use osu::bridge::reqwest::OsuReqwestRequester;
use reqwest::Client;
use std::error::Error;

fn try_main() -> Result<(), Box<Error>> {
    let key = env::var("OSU_KEY")?;
    let client = Client::new();
    let match = client.get_match(&key, 71641)?;

    println!("Match start time: {}", match.start_time);

    Ok(())
}

fn main() {
    try_main().unwrap();
}
```

### License

License info in [LICENSE.md]. Long story short, ISC.

[ci]: https://travis-ci.org/zeyla/osu.rs
[ci-badge]: https://travis-ci.org/zeyla/osu.rs.svg?branch=master
[docs]: https://docs.rs/crate/osu
[docs-badge]: https://img.shields.io/badge/docs-online-2020ff.svg
[LICENSE.md]: https://github.com/zeyla/osu.rs/blob/master/LICENSE.md
[license]: https://opensource.org/licenses/ISC
[license-badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=flat-square
