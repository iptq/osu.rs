//! Bridged implementation support for various HTTP clients.

#[cfg(feature = "hyper")]
pub mod hyper;
#[cfg(feature = "reqwest")]
pub mod reqwest;

#[cfg(feature = "hyper")]
pub use self::hyper::OsuHyperRequester;
#[cfg(feature = "reqwest")]
pub use self::reqwest::OsuReqwestRequester;
