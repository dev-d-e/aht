/*!
A module for error.
*/

use getset::{Getters, MutGetters};
use std::num::{ParseFloatError, ParseIntError};
use std::str::ParseBoolError;

pub type Result<T, E = Error> = std::result::Result<T, E>;

///Represents error kind.
#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    BrokenEnd,
    UnexpectedChar,
    InvalidMark,
    InvalidAttribute,
    Interrupted,
    AttributeParse,
    ParseBool,
    ParseInt,
    ParseFloat,
    None,
    NotFound,
    MediaError,
    GpuError,
}

///Represents error.
#[derive(Debug, Getters)]
pub struct Error {
    #[getset(get = "pub")]
    kind: ErrorKind,
    p: Option<(usize, usize)>,
    e: Option<Box<dyn std::error::Error>>,
}

impl Error {
    fn format(kind: ErrorKind, a: usize, b: usize) -> Self {
        Self {
            kind,
            p: Some((a, b)),
            e: None,
        }
    }

    fn kind_err(kind: ErrorKind, e: impl std::error::Error + 'static) -> Self {
        Self {
            kind,
            p: None,
            e: Some(Box::new(e)),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = format!("{:?}", self.kind);
        if let Some(p) = self.p {
            s.push_str(&format!("{p:?}"));
        }
        if let Some(e) = &self.e {
            s.push_str(&format!("{e}"));
        }
        f.write_str(&s)
    }
}

impl std::error::Error for Error {}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self {
            kind,
            p: None,
            e: None,
        }
    }
}

impl From<(ErrorKind, usize, usize)> for Error {
    fn from(o: (ErrorKind, usize, usize)) -> Self {
        Self::format(o.0, o.1, o.2)
    }
}

impl<T> From<(ErrorKind, T)> for Error
where
    T: std::error::Error + 'static,
{
    fn from(o: (ErrorKind, T)) -> Self {
        Self::kind_err(o.0, o.1)
    }
}

impl From<ParseBoolError> for Error {
    fn from(e: ParseBoolError) -> Self {
        Self::kind_err(ErrorKind::ParseBool, e)
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Self::kind_err(ErrorKind::ParseInt, e)
    }
}

impl From<ParseFloatError> for Error {
    fn from(e: ParseFloatError) -> Self {
        Self::kind_err(ErrorKind::ParseFloat, e)
    }
}

///Collection of `Error`.
#[derive(Debug, Default, Getters, MutGetters)]
pub struct ErrorHolder {
    v: Vec<Error>,
}

deref!(ErrorHolder, Vec<Error>, v);

impl ErrorHolder {}
