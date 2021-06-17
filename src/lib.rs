//! This crate provides types and methods for parsing/ creating [FCP](https://github.com/insomnimus/fcp) requests.
//!
//! # Examples
//!
//! ```
//! use fcp::{Request, AdjRequest, GetRequest, SetRequest};
//!
//! // Imagine we have an incoming fcp request
//! // over some connection. We can parse the request like this.
//! let req= b"GET all"; // the bytes we received over the connection
//! let req = Request::parse(req).unwrap();
//! assert_eq!(Request::Get(GetRequest::All), req);
//!
//! // We can also use the string extension method:
//! let req: Request = "SET v500".parse().unwrap();
//! assert_eq!(Request::Set(SetRequest::Voltage(500)), req);
//!
//! // On the client side, we may form requests like this:
//! let req = Request::Adj(AdjRequest::Voltage(-25));
//! let req_string= format!("{};", &req); // fcp requests are ';' terminated
//! assert_eq!("ADJ v-25;", req_string.as_str());

#![cfg_attr(not(test), no_std)]

#[cfg(test)]
mod tests;

use core::{
    fmt,
    str::{self, FromStr},
};
use Error::*;

type Result<T> = core::result::Result<T, Error>;

/// Crate specific (not FCP bound) errors that may be returned trying to parse Requests.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// The request method word is unknown. For example "FETCH x" (FETCH is invalid).
    UnknownRequestType,
    /// The request has an invalid value. For example "GET shoesize".
    InvalidValue,
    /// The request consists of just the method. For example "SET".
    MissingValue,
    /// The request is empty. For example "".
    Empty,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Empty => "empty request",
                Self::UnknownRequestType => "unknown request type",
                Self::InvalidValue => "invalid value",
                Self::MissingValue => "missing value",
            }
        )
    }
}

/// Types of FCP requests.
#[derive(Debug, PartialEq, Eq)]
pub enum Request {
    /// The `GET`request.
    Get(GetRequest),
    /// The `SET` request.
    Set(SetRequest),
    /// The `ADJ` (adjust) request.
    Adj(AdjRequest),
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Get(r) => write!(f, "GET {}", r.val_str()),
            Self::Set(r) => {
                use SetRequest::*;
                match r {
                    Voltage(v) => write!(f, "SET v{}", v),
                    Percentage(v) => write!(f, "SET %{}", v),
                    Auto => write!(f, "SET a"),
                }
            }
            Self::Adj(r) => {
                use AdjRequest::*;
                match r {
                    Voltage(v) => write!(f, "ADJ v{}", v),
                    Percentage(v) => write!(f, "ADJ %{}", v),
                }
            }
        }
    }
}

impl Request {
    /// Tries to parse a slice of bytes into a Request.
    ///
    /// # Errors
    /// `parse` will error if the given byte slice doesn't contain valid ASCII or
    /// the byte slice is not a valid FCP request.
    ///
    /// # Examples
    ///
    /// ```
    /// use fcp::{Request, SetRequest};
    /// let req = b"SET %25";
    /// assert_eq!(Ok(Request::Set(SetRequest::Percentage(25))), Request::parse(req));
    /// ```
    pub fn parse(s: &[u8]) -> Result<Self> {
        let mut split = s.splitn(2, |b| *b == b' ');
        let method = match split.next() {
            None => return Err(Empty),
            Some(x) => x,
        };
        if let Some(val) = split.next() {
            match method {
                b"GET" => GetRequest::parse(val).map(Self::Get),
                b"SET" => SetRequest::parse(val).map(Self::Set),
                b"ADJ" => AdjRequest::parse(val).map(Self::Adj),
                _ => Err(UnknownRequestType),
            }
        } else {
            match method {
                b"GET" | b"SET" | b"ADJ" => Err(MissingValue),
                b"" => Err(Empty),
                _ => Err(UnknownRequestType),
            }
        }
    }

    /// Returns the method name, as a static &str.
    ///
    /// # Examples
    ///
    /// ```
    /// use fcp::{Request, GetRequest};
    /// let req = Request::Get(GetRequest::Config);
    /// assert_eq!("GET", req.method());
    /// ```
    pub fn method(&self) -> &'static str {
        match self {
            Self::Get(_) => "GET",
            Self::Set(_) => "SET",
            Self::Adj(_) => "ADJ",
        }
    }
}

impl FromStr for Request {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Self::parse(s.as_bytes())
    }
}

/// Types of `GET` FCP requests.
#[derive(Debug, PartialEq, Eq)]
pub enum GetRequest {
    All,
    Config,
    Percentage,
    Temperature,
    Voltage,
}

impl GetRequest {
    fn parse(val: &[u8]) -> Result<Self> {
        use GetRequest::*;
        Ok(match val {
            b"all" => All,
            b"%" => Percentage,
            b"cfg" => Config,
            b"volt" => Voltage,
            b"temp" => Temperature,
            _ => return Err(InvalidValue),
        })
    }

    pub fn val_str(&self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Voltage => "volt",
            Self::Config => "cfg",
            Self::Temperature => "temp",
            Self::Percentage => "%",
        }
    }
}

/// Types of `SET` FCP requests.
#[derive(Debug, PartialEq, Eq)]
pub enum SetRequest {
    Auto,
    Voltage(u16),
    Percentage(u8),
}

impl SetRequest {
    fn parse(val: &[u8]) -> Result<Self> {
        use SetRequest::*;
        if val.is_empty() {
            return Err(MissingValue);
        }
        unsafe {
            Ok(match val[0] {
                b'a' if val.len() == 1 => Auto,
                b'v' => {
                    if let Ok(n) = str::from_utf8_unchecked(&val[1..]).parse::<u16>() {
                        Voltage(n)
                    } else {
                        return Err(InvalidValue);
                    }
                }
                b'%' => {
                    if let Ok(n) = str::from_utf8_unchecked(&val[1..]).parse::<u8>() {
                        Percentage(n)
                    } else {
                        return Err(InvalidValue);
                    }
                }
                _ => return Err(InvalidValue),
            })
        }
    }
}

/// Types of `ADJ` (adjust) FCP requests.
#[derive(Debug, PartialEq, Eq)]
pub enum AdjRequest {
    Voltage(i16),
    Percentage(i8),
}

impl AdjRequest {
    fn parse(val: &[u8]) -> Result<Self> {
        use AdjRequest::*;
        if val.is_empty() {
            return Err(MissingValue);
        }
        unsafe {
            match val[0] {
                b'v' => {
                    if let Ok(n) = str::from_utf8_unchecked(&val[1..]).parse::<i16>() {
                        Ok(Voltage(n))
                    } else {
                        Err(InvalidValue)
                    }
                }
                b'%' => {
                    if let Ok(n) = str::from_utf8_unchecked(&val[1..]).parse::<i8>() {
                        Ok(Percentage(n))
                    } else {
                        Err(InvalidValue)
                    }
                }
                _ => Err(InvalidValue),
            }
        }
    }
}
