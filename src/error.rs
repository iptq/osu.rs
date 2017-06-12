use serde_json::{Error as JsonError, Value};
use std::io::Error as IoError;
use std::error::Error as StdError;
use std::fmt::{Display, Error as FmtError, Formatter, Result as FmtResult};
use std::num::{ParseFloatError, ParseIntError};
use std::result::Result as StdResult;

#[cfg(feature="hyper")]
use hyper::Error as HyperError;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// An error from `std::fmt`
    Format(FmtError),
    /// A `hyper` crate error
    #[cfg(feature="hyper")]
    Hyper(HyperError),
    /// A `serde_json` crate error
    Json(JsonError),
    /// A `std::io` module error
    Io(IoError),
    ParseFloat(ParseFloatError),
    ParseInt(ParseIntError),
    /// A json decoding error, with a description and the offending value
    Decode(&'static str, Value),
    /// A miscellaneous error, with a description
    Other(&'static str),
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

#[cfg(feature="hyper")]
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

impl From<ParseFloatError> for Error {
    fn from(err: ParseFloatError) -> Error {
        Error::ParseFloat(err)
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Error {
        Error::ParseInt(err)
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
            Error::ParseFloat(ref inner) => inner.description(),
            Error::ParseInt(ref inner) => inner.description(),
            Error::Decode(msg, _) | Error::Other(msg) => msg,
            #[cfg(feature="hyper")]
            Error::Hyper(ref inner) => inner.description(),
        }
    }
}
