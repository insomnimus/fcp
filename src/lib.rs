#![no_std]

#[cfg(test)]
mod tests;

use core::{fmt, str};
use Error::*;

type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    UnknownRequestType,
    InvalidValue,
    MissingValue,
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

#[derive(Debug, PartialEq)]
pub enum Request {
    Get(GetRequest),
    Set(SetRequest),
    Adj(AdjRequest),
}

#[derive(Debug, PartialEq)]
pub enum GetRequest {
    Config,
    Percentage,
    Temperature,
    Voltage,
}

impl GetRequest {
    fn parse(val: &[u8]) -> Result<Self> {
        use GetRequest::*;
        Ok(match val {
            b"%" => Percentage,
            b"cfg" => Config,
            b"volt" => Voltage,
            b"temp" => Temperature,
            _ => return Err(InvalidValue),
        })
    }

    pub fn val_str(&self) -> &'static str {
        match self {
            Self::Voltage => "volt",
            Self::Config => "cfg",
            Self::Temperature => "temp",
            Self::Percentage => "%",
        }
    }
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

impl Request {
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

    pub fn method(&self) -> &'static str {
        match self {
            Self::Get(_) => "GET",
            Self::Set(_) => "SET",
            Self::Adj(_) => "ADJ",
        }
    }
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
