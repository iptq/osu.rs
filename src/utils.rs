use serde_json::Value;
use std::collections::BTreeMap;
use ::error::{Error, Result};

#[macro_escape]
macro_rules! field {
    ($map:expr, R, [], $key:expr, $decode:path) => {
        try!(remove(&mut $map, $key).and_then(|v| decode_array(v, $decode)))
    };
    ($map:expr, float, $key:expr) => {
        try!(match remove(&mut $map, $key) {
            Ok(Value::F64(v)) => Ok(v),
            Ok(Value::String(v)) => v.parse::<f64>().map_err(Error::from),
            _ => Err(Error::Other("Expected valid f64")),
        })
    };
    ($map:expr, O, int, $key:expr) => {
        try!(opt(&mut $map, $key, into_i64))
    };
    ($map:expr, int, $key:expr) => {
        try!(remove(&mut $map, $key).and_then(into_i64))
    };
    ($map:expr, O, $key:expr, $decode:path) => {
        try!(opt(&mut $map, $key, $decode))
    };
    ($map:expr, R, $key:expr, $decode:path) => {
        try!(remove(&mut $map, $key).and_then($decode))
    };
}

#[macro_escape]
macro_rules! map_names {
    ($typ:ident; $($entry:ident $value:expr,)*) => {
        impl $typ {
            pub fn name(&self) -> &str {
                match *self {
                    $($typ::$entry => $value,)*
                }
            }

            #[doc(hidden)]
            pub fn from_str(name: &str) -> Option<Self> {
                match name {
                    $($value => Some($typ::$entry),)*
                    _ => None,
                }
            }

            #[doc(hiddden)]
            pub fn decode(value: Value) -> Result<Self> {
                let name = try!(into_string(value));
                Self::from_str(&name).ok_or(Error::Decode(
                    concat!("Expected valid ", stringify!($typ)),
                    Value::String(name)
                ))
            }
        }
    }
}

pub fn into_map(value: Value) -> Result<BTreeMap<String, Value>> {
    match value {
        Value::Object(m) => Ok(m),
        value => Err(Error::Decode("Expected object", value)),
    }
}

pub fn decode_array<T, F: Fn(Value) -> Result<T>>(value: Value,
                                                  f: F)
                                                  -> Result<Vec<T>> {
    into_array(value)
        .and_then(|x| x.into_iter().map(f).collect())
}

pub fn into_array(value: Value) -> Result<Vec<Value>> {
    match value {
        Value::Array(v) => Ok(v),
        value => Err(Error::Decode("Expected array", value)),
    }
}

pub fn into_string(value: Value) -> Result<String> {
    match value {
        Value::String(s) => Ok(s),
        value => Err(Error::Decode("Expected string", value)),
    }
}

pub fn into_i64(value: Value) -> Result<i64> {
    match value {
        Value::I64(v) => Ok(v),
        Value::U64(v) => Ok(v as i64),
        Value::String(v) => v.parse::<i64>().map_err(Error::from),
        other => Err(Error::Decode("Expected valid i64", other)),
    }
}

pub fn remove(map: &mut BTreeMap<String, Value>, key: &str) -> Result<Value> {
    map.remove(key)
        .ok_or_else(|| Error::Decode("Unexpected absent key",
                                     Value::String(key.into())))
}

pub fn opt<T, F: FnOnce(Value) -> Result<T>>(map: &mut BTreeMap<String, Value>,
                                            key: &str, f: F)
                                            -> Result<Option<T>> {
    match map.remove(key) {
        None | Some(Value::Null) => Ok(None),
        Some(val) => f(val).map(Some),
    }
}
