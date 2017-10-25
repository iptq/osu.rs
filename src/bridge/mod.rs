#[cfg(feature = "hyper")]
pub mod hyper;

#[cfg(feature = "hyper")]
pub use self::hyper::OsuHyperRequester;
