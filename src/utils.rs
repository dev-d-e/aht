use crate::error::*;
pub(crate) use log::{debug, error, info, trace, warn};
use std::str::FromStr;
use std::time::{Duration, Instant};

pub(crate) mod ascii {
    pub(crate) const NULL: char = '\0';
    ///' '
    pub(crate) const SPACE: char = ' ';
    ///!
    pub(crate) const EXCLAMATION: char = '!';
    ///"
    pub(crate) const QUOTATION: char = '"';
    ///#
    pub(crate) const NUMBER_SIGN: char = '#';
    ///%
    pub(crate) const PER_CENT: char = '%';
    ///&
    pub(crate) const AMPERSAND: char = '&';
    ///'
    pub(crate) const APOSTROPHE: char = '\'';
    ///(
    pub(crate) const LEFT_PARENTHESIS: char = '(';
    ///)
    pub(crate) const RIGHT_PARENTHESIS: char = ')';
    ///*
    pub(crate) const ASTERISK: char = '*';
    ///+
    pub(crate) const PLUS: char = '+';
    ///,
    pub(crate) const COMMA: char = ',';
    ///-
    pub(crate) const HYPHEN: char = '-';
    ///.
    pub(crate) const FULL_STOP: char = '.';
    ///\/
    pub(crate) const SLASH: char = '/';
    ///\n
    pub(crate) const LF: char = '\n';
    ///\r
    pub(crate) const CR: char = '\r';
    ///:
    pub(crate) const COLON: char = ':';
    ///;
    pub(crate) const SEMICOLON: char = ';';
    ///<
    pub(crate) const LT: char = '<';
    ///>
    pub(crate) const GT: char = '>';
    ///=
    pub(crate) const EQUAL: char = '=';
    ///?
    pub(crate) const QUESTION: char = '?';
    ///@
    pub(crate) const AT: char = '@';
    ///[
    pub(crate) const LEFT_SQUARE_BRACKET: char = '[';
    ///]
    pub(crate) const RIGHT_SQUARE_BRACKET: char = ']';
    ///\
    pub(crate) const BACKSLASH: char = '\\';
    ///^
    pub(crate) const CIRCUMFLEX_ACCENT: char = '^';
    ///_
    pub(crate) const LOW_LINE: char = '_';
    ///{
    pub(crate) const LEFT_CURLY_BRACKET: char = '{';
    ///}
    pub(crate) const RIGHT_CURLY_BRACKET: char = '}';
    ///|
    pub(crate) const VERTICAL_LINE: char = '|';

    use crate::error::*;

    #[derive(Debug, Default)]
    pub(crate) struct CharCounter {
        total: usize,
        row: usize,
        column: usize,
        cr: bool,
    }

    impl CharCounter {
        pub(crate) fn count(&mut self, c: char) {
            self.total += 1;
            match c {
                CR => {
                    self.row += 1;
                    self.column = 0;
                    self.cr = true;
                }
                LF => {
                    if !self.cr {
                        self.row += 1;
                        self.column = 0;
                    }
                    self.cr = false;
                }
                _ => {
                    self.column += 1;
                    self.cr = false;
                }
            }
        }

        pub(crate) fn to_error(&self, k: ErrorKind, s: impl ToString) -> Error {
            (k, self.row, self.column, s).into()
        }
    }
}

#[inline(always)]
pub(crate) fn to_bool(s: &str) -> Result<bool> {
    if s.is_empty() {
        Ok(true)
    } else {
        bool::from_str(s).map_err(|e| (ErrorKind::StrErr, e).into())
    }
}

#[inline(always)]
pub(crate) fn to_usize(s: &str) -> Result<usize> {
    usize::from_str_radix(s, 10).map_err(|e| (ErrorKind::StrErr, e).into())
}

#[inline(always)]
pub(crate) fn to_isize(s: &str) -> Result<isize> {
    isize::from_str_radix(s, 10).map_err(|e| (ErrorKind::StrErr, e).into())
}

#[inline(always)]
pub(crate) fn to_f32(s: &str) -> Result<f32> {
    f32::from_str(s).map_err(|e| (ErrorKind::StrErr, e).into())
}

#[inline(always)]
pub(crate) fn between<T: PartialOrd>(o: T, min: T, max: T) -> bool {
    o >= min && o <= max
}

#[derive(Debug)]
pub(crate) struct Chronograph {
    t: Instant,
    n: u64,
}

impl Chronograph {
    pub(crate) fn new(n: u64) -> Self {
        Self {
            t: Instant::now(),
            n,
        }
    }

    pub(crate) fn elapsed(&self) -> bool {
        self.t.elapsed() >= Duration::from_millis(self.n)
    }

    pub(crate) fn refresh(&mut self) {
        self.t = Instant::now();
    }
}

#[derive(Debug)]
pub(crate) struct FpsCtrl {
    target: (u32, u32),
    duration: Duration,
    t: Instant,
}

impl FpsCtrl {
    pub(crate) fn new(target: (u32, u32)) -> Self {
        let duration = Duration::from_micros(target.1 as u64 * 1000000 / target.0 as u64);
        Self {
            target,
            duration,
            t: Instant::now(),
        }
    }

    pub(crate) fn need_to_wait(&mut self) -> Option<Instant> {
        let next_frame_time = self.t + self.duration;
        let now = Instant::now();
        if now >= next_frame_time {
            self.t = now;
            None
        } else {
            Some(next_frame_time)
        }
    }

    pub(crate) fn is_next(&mut self) -> bool {
        self.need_to_wait().is_none()
    }
}

impl From<(u32, u32)> for FpsCtrl {
    fn from(o: (u32, u32)) -> Self {
        Self::new(o)
    }
}

#[derive(Debug)]
pub(crate) struct FpsCounter {
    n: u64,
    t: Instant,
    r: u64,
}

impl Default for FpsCounter {
    fn default() -> Self {
        Self {
            n: 0,
            t: Instant::now(),
            r: 0,
        }
    }
}

impl FpsCounter {
    pub(crate) fn reset(&mut self) {
        self.n = 0;
        self.t = Instant::now();
    }

    pub(crate) fn count(&mut self) {
        self.n += 1;
        let o = self.t.elapsed().as_millis() as u64;
        if o >= 1000 {
            self.r = self.n * 1000 / o;
            self.reset();
        }
    }

    pub(crate) fn fps(&self) -> Option<u64> {
        if self.r > 0 { Some(self.r) } else { None }
    }
}
