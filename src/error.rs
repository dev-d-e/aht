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
    Markup,
    Style,
    Script,
    StrErr,
    Media,
    Gpu,
    Window,
}

///Represents error.
#[derive(Getters)]
pub struct Error {
    #[getset(get = "pub")]
    kind: ErrorKind,
    p: Option<(usize, usize)>,
    s: String,
}

impl Error {
    fn format(kind: ErrorKind, a: usize, b: usize, s: impl ToString) -> Self {
        Self {
            kind,
            p: Some((a, b)),
            s: s.to_string(),
        }
    }

    fn new(kind: ErrorKind, s: impl ToString) -> Self {
        Self {
            kind,
            p: None,
            s: s.to_string(),
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = format!("{:?} Error.", self.kind);
        if let Some(p) = self.p {
            s.push_str(&format!(" {p:?}"));
        }
        if self.s.len() > 0 {
            s.push(' ');
            s.push_str(&self.s);
        }
        f.write_str(&s)
    }
}

impl std::error::Error for Error {}

impl<T> From<(ErrorKind, usize, usize, T)> for Error
where
    T: ToString,
{
    fn from(o: (ErrorKind, usize, usize, T)) -> Self {
        Self::format(o.0, o.1, o.2, o.3)
    }
}

impl<T> From<(ErrorKind, T)> for Error
where
    T: ToString,
{
    fn from(o: (ErrorKind, T)) -> Self {
        Self::new(o.0, o.1)
    }
}

#[inline(always)]
pub(crate) fn to_err(kind: ErrorKind, e: impl std::error::Error) -> Error {
    (kind, e).into()
}

///Collection of `Error`.
#[derive(Debug, Default, Getters, MutGetters)]
pub struct ErrorHolder {
    v: Vec<Error>,
}

deref!(ErrorHolder, Vec<Error>, v);

impl ErrorHolder {}
