use super::*;
use crate::error::*;
use crate::utils::ascii::*;
use crate::utils::*;
use getset::{CopyGetters, Setters};
use skia_safe::{scalar, ISize, Point, Size};
use std::ops::Add;
use std::str::FromStr;

pub type Color = skia_safe::Color;

///Represents two-dimensional coordinate.
#[derive(Clone, Debug, Default, CopyGetters, Setters)]
#[getset(get_copy = "pub", set = "pub")]
pub struct Coord2D {
    x: f32,
    y: f32,
}

impl std::fmt::Display for Coord2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Coord2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub(crate) fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }
}

impl Add for &Coord2D {
    type Output = Coord2D;

    fn add(self, other: Self) -> Self::Output {
        Coord2D::new(self.x + other.x, self.y + other.y)
    }
}

impl Add<(f32, f32)> for &Coord2D {
    type Output = Coord2D;

    fn add(self, other: (f32, f32)) -> Self::Output {
        Coord2D::new(self.x + other.0, self.y + other.1)
    }
}

impl From<(f32, f32)> for Coord2D {
    fn from(o: (f32, f32)) -> Self {
        Self::new(o.0, o.1)
    }
}

impl From<&Coord> for Coord2D {
    fn from(c: &Coord) -> Self {
        Self::new(c.x, c.y)
    }
}

impl From<Coord2D> for Size {
    fn from(c: Coord2D) -> Self {
        Self::new(c.x, c.y)
    }
}

impl From<Coord2D> for ISize {
    fn from(c: Coord2D) -> Self {
        Self::new(c.x as i32, c.y as i32)
    }
}

impl From<Coord2D> for Point {
    fn from(c: Coord2D) -> Self {
        Self::new(c.x as scalar, c.y as scalar)
    }
}

impl From<&Coord2D> for Point {
    fn from(c: &Coord2D) -> Self {
        Self::new(c.x as scalar, c.y as scalar)
    }
}

///Represents three-dimensional coordinate.
#[derive(Clone, Debug, Default, CopyGetters, Setters)]
pub struct Coord {
    xy: Coord2D,
    #[getset(get_copy = "pub", set = "pub")]
    z: f32,
}

deref!(Coord, Coord2D, xy);

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl Coord {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            xy: Coord2D::new(x, y),
            z,
        }
    }

    pub(crate) fn is_finite(&self) -> bool {
        self.xy.is_finite() && self.z.is_finite()
    }

    pub(crate) fn is_empty(&self) -> bool {
        !self.is_finite()
    }
}

impl Add for &Coord {
    type Output = Coord;

    fn add(self, other: Self) -> Self::Output {
        Coord::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<(f32, f32)> for &Coord {
    type Output = Coord;

    fn add(self, other: (f32, f32)) -> Self::Output {
        Coord::new(self.x + other.0, self.y + other.1, self.z)
    }
}

impl Add<(f32, f32, f32)> for &Coord {
    type Output = Coord;

    fn add(self, other: (f32, f32, f32)) -> Self::Output {
        Coord::new(self.x + other.0, self.y + other.1, self.z + other.2)
    }
}

impl FromStr for Coord {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut v = Vec::new();
        for o in s.split(COMMA) {
            let o = o.trim();
            if o.len() > 0 {
                v.push(to_f32(o)?)
            }
        }

        let c = match v.len() {
            0 => Self::default(),
            1 => Self::new(v[0], 0.0, 0.0),
            2 => Self::new(v[0], v[1], 0.0),
            _ => Self::new(v[0], v[1], v[2]),
        };
        Ok(c)
    }
}

impl TryFrom<&str> for Coord {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

impl TryFrom<&String> for Coord {
    type Error = Error;

    fn try_from(s: &String) -> Result<Self> {
        Self::from_str(s.as_str())
    }
}

impl From<(f32, f32, f32)> for Coord {
    fn from(o: (f32, f32, f32)) -> Self {
        Self::new(o.0, o.1, o.2)
    }
}

impl From<(f32, f32)> for Coord {
    fn from(o: (f32, f32)) -> Self {
        Self::new(o.0, o.1, 0.0)
    }
}

impl From<&Coord2D> for Coord {
    fn from(o: &Coord2D) -> Self {
        Self::new(o.x, o.y, 0.0)
    }
}

///Represents distance.
#[derive(Clone, Debug)]
pub enum Distance {
    Pixel(f32),
    Percentage(f32),
}

impl std::fmt::Display for Distance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Distance::Pixel(i) => {
                write!(f, "{}", i)
            }
            Distance::Percentage(i) => {
                write!(f, "{}{}", i, PER_CENT)
            }
        }
    }
}

impl Distance {
    pub fn pixel(&self) -> Option<f32> {
        match self {
            Self::Pixel(i) => Some(*i),
            Self::Percentage(_) => None,
        }
    }

    pub fn get(&self, n: f32) -> f32 {
        match self {
            Self::Pixel(i) => *i,
            Self::Percentage(i) => n * (*i) / 100.0,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Pixel(i) => *i == 0.0,
            Self::Percentage(i) => *i == 0.0,
        }
    }
}

impl FromStr for Distance {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((s, _)) = s.split_once(PER_CENT) {
            to_f32(s).map(|i| Self::Percentage(i))
        } else {
            to_f32(s).map(|i| Self::Pixel(i))
        }
    }
}

impl TryFrom<&str> for Distance {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

impl TryFrom<&String> for Distance {
    type Error = Error;

    fn try_from(s: &String) -> Result<Self> {
        Self::from_str(s.as_str())
    }
}

///Represents ordinal.
#[derive(Clone, Debug)]
pub enum Ordinal {
    Number(usize),
    X(usize),
    Y(usize),
    XY(usize, usize),
    None,
}

impl std::fmt::Display for Ordinal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ordinal::Number(n) => {
                write!(f, "{}", n)
            }
            Ordinal::X(x) => {
                write!(f, "{}{}", x, COMMA)
            }
            Ordinal::Y(y) => {
                write!(f, "{}{}", COMMA, y)
            }
            Ordinal::XY(x, y) => {
                write!(f, "{}{}{}", x, COMMA, y)
            }
            Ordinal::None => Ok(()),
        }
    }
}

impl FromStr for Ordinal {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.is_empty() {
            Ok(Self::None)
        } else if let Some(s) = s.split_once(COMMA) {
            let a = s.0.trim();
            let b = s.1.trim();
            if a.is_empty() {
                if b.is_empty() {
                    Ok(Self::None)
                } else {
                    to_usize(b).map(|y| Self::Y(y))
                }
            } else {
                if b.is_empty() {
                    to_usize(a).map(|x| Self::X(x))
                } else {
                    to_usize(a).and_then(|x| to_usize(b).map(|y| Self::XY(x, y)))
                }
            }
        } else {
            to_usize(s).map(|n| Self::Number(n))
        }
    }
}

impl TryFrom<&str> for Ordinal {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

impl TryFrom<&String> for Ordinal {
    type Error = Error;

    fn try_from(s: &String) -> Result<Self> {
        Self::from_str(s.as_str())
    }
}

///Represents points.
#[derive(Clone, Debug, Default, CopyGetters, Setters)]
pub struct Points {
    data: Vec<Distance>,
    #[getset(get_copy = "pub", set = "pub")]
    count: usize,
}

deref!(Points, Vec<Distance>, data);

impl std::fmt::Display for Points {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        if self.data.len() > 0 {
            s.push(LEFT_SQUARE_BRACKET);
            let v: Vec<String> = self.data.iter().map(|o| o.to_string()).collect();
            s.push_str(&v.join(&COMMA.to_string()));
            s.push(RIGHT_SQUARE_BRACKET);
            s.push(COMMA);
        }
        write!(f, "{}{}", s, self.count)
    }
}

impl Points {
    fn effect(&self, sum: f32) -> Vec<f32> {
        let mut v = Vec::new();
        let mut o = 0.0;
        for i in &self.data {
            o = i.get(sum);
            v.push(o);
        }
        //one point at least.
        if v.is_empty() {
            v.push(o);
        }
        let mut k = 0.0;
        if o < sum {
            k = sum - o;
        }
        let count = self.count;
        let n = v.len();
        if n < count {
            let n = count - n + 1;
            k = k / n as f32;
            for _ in 0..n - 1 {
                o = o + k;
                v.push(o);
            }
        }
        v
    }

    pub(crate) fn coord(&self, sum: f32, zero: f32) -> Vec<f32> {
        let mut v = self.effect(sum);
        v.iter_mut().for_each(|i| *i += zero);
        v
    }
}

impl FromStr for Points {
    type Err = Error;

    fn from_str(mut s: &str) -> Result<Self> {
        if s.is_empty() {
            return Ok(Self::default());
        }

        if let Some(t) = s.split_once(LEFT_SQUARE_BRACKET) {
            s = t.1;
        }
        let mut data_str = "";
        let mut count_str = s;
        if let Some(t) = s.split_once(RIGHT_SQUARE_BRACKET) {
            (data_str, count_str) = t;
        }

        let mut data = Vec::new();
        for o in data_str.split(COMMA) {
            let o = o.trim();
            if o.len() > 0 {
                data.push(Distance::try_from(o)?);
            }
        }

        for o in count_str.split(COMMA) {
            let o = o.trim();
            if o.len() > 0 {
                count_str = o;
                break;
            }
        }

        if count_str.is_empty() {
            Ok(Self { data, count: 0 })
        } else {
            to_usize(count_str).map(|count| Self { data, count })
        }
    }
}

impl TryFrom<&str> for Points {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

impl TryFrom<&String> for Points {
    type Error = Error;

    fn try_from(s: &String) -> Result<Self> {
        Self::from_str(s.as_str())
    }
}

const JS: &str = "text/javascript";

///Represents script type.
#[derive(Clone, Debug)]
pub enum ScriptType {
    JS,
}

impl std::fmt::Display for ScriptType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JS => f.write_str(JS),
        }
    }
}

impl FromStr for ScriptType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            JS => Ok(Self::JS),
            _ => Err((ErrorKind::Script, "unsupported").into()),
        }
    }
}

impl TryFrom<&str> for ScriptType {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

impl TryFrom<&String> for ScriptType {
    type Error = Error;

    fn try_from(s: &String) -> Result<Self> {
        Self::from_str(s.as_str())
    }
}

#[derive(Clone, Debug, Default, CopyGetters, Setters)]
#[getset(get_copy = "pub(crate)", set = "pub(crate)")]
pub(crate) struct RectSide {
    width: f32,
    height: f32,
}

impl std::fmt::Display for RectSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.width, self.height)
    }
}

impl RectSide {
    pub(crate) fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub(crate) fn away_from(a: &Coord2D, b: &Coord2D) -> Self {
        Self::new(a.x() - b.x(), a.y() - b.y())
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.width == 0.0 || self.height == 0.0
    }

    pub(crate) fn same_ratio(&self, r: &Self, r2: &Self) -> Self {
        Self::new(
            self.width * r.width / r2.width,
            self.height * r.height / r2.height,
        )
    }

    pub(crate) fn get_attr(&mut self, e: &Element, r: &Self) {
        if let Some(a) = e.attribute().width() {
            self.width = a.get(r.width);
        }
        if let Some(a) = e.attribute().height() {
            self.height = a.get(r.height);
        }
    }
}

impl Add<(f32, f32)> for &RectSide {
    type Output = RectSide;

    fn add(self, other: (f32, f32)) -> Self::Output {
        RectSide::new(self.width + other.0, self.height + other.1)
    }
}

impl From<&RectSide> for Size {
    fn from(o: &RectSide) -> Self {
        Self::new(o.width, o.height)
    }
}

impl From<&RectSide> for ISize {
    fn from(o: &RectSide) -> Self {
        Self::new(o.width as i32, o.height as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance() {
        let s = "100";
        let d = Distance::try_from(s).unwrap();
        assert_eq!(s, d.to_string());

        let s = "20%";
        let d = Distance::try_from(s).unwrap();
        assert_eq!(s, d.to_string());
    }

    #[test]
    fn ordinal() {
        let s = "1";
        let o = Ordinal::try_from(s).unwrap();
        assert_eq!(s, o.to_string());

        let s = "";
        let o = Ordinal::try_from(s).unwrap();
        assert_eq!(s, o.to_string());

        let s = "1,2";
        let o = Ordinal::try_from(s).unwrap();
        assert_eq!(s, o.to_string());
    }

    #[test]
    fn points() {
        let p = Points {
            data: vec![Distance::Pixel(100.0), Distance::Percentage(20.0)],
            count: 3,
        };
        let v = p.effect(1000.0);
        assert_eq!((v[0], v[1], v[2]), (100.0, 200.0, 600.0));

        let s = "[100,20%],3";
        let p = Points::try_from(s).unwrap();
        let v = p.effect(1000.0);
        assert_eq!((v[0], v[1], v[2]), (100.0, 200.0, 600.0));
        assert_eq!(s, p.to_string());

        let s = " [ 100 ,, ,] , 3 ,";
        let p = Points::try_from(s).unwrap();
        let v = p.effect(1000.0);
        assert_eq!((v[0], v[1], v[2]), (100.0, 400.0, 700.0));

        let s = "2";
        let p = Points::try_from(s).unwrap();
        let v = p.effect(1000.0);
        assert_eq!((v[0], v[1]), (0.0, 500.0));
        assert_eq!(s, p.to_string());
    }
}
