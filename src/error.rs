//! The error and result types returned by the library.
//!
//! All library functions return a [`Result`], which has an `Err` type of
//! [`Error`].
//!
//! [`Error`]: enum.Error.html
//! [`Result`]: type.Result.html

use serde_json::Error as JsonError;
use std::error::Error as StdError;
use std::fmt::{Display, Error as FmtError, Formatter, Result as FmtResult};
use std::io::Error as IoError;
use std::result::Result as StdResult;

#[cfg(feature = "hyper")]
use hyper::Error as HyperError;

/// The result type used throughout the library.
pub type Result<T> = StdResult<T, Error>;

/// The error type used throughout the library.
///
/// This wraps all of the depended crates' error types and stdlib error types
/// that can return from functions.
#[derive(Debug)]
pub enum Error {
    /// An error from `std::fmt`
    Format(FmtError),
    /// A `hyper` crate error
    #[cfg(feature = "hyper")]
    Hyper(HyperError),
    /// A `serde_json` crate error
    Json(JsonError),
    /// A `std::io` module error
    Io(IoError),
}

impl From<FmtError> for Error {
    fn from(err: FmtError) -> Error {
        Error::Format(err)
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Io(err)
    }
}

#[cfg(feature = "hyper")]
impl From<HyperError> for Error {
    fn from(err: HyperError) -> Error {
        Error::Hyper(err)
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Error {
        Error::Json(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Format(ref inner) => inner.description(),
            Error::Json(ref inner) => inner.description(),
            Error::Io(ref inner) => inner.description(),
            #[cfg(feature = "hyper")]
            Error::Hyper(ref inner) => inner.description(),
        }
    }
}
