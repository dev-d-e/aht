use crate::utils::ascii::*;
use crate::utils::*;
use skia_safe::{scalar, IRect, ISize, Point, Rect};
use std::time::{Duration, Instant};

///three-dimensional coordinate.
#[derive(Debug, Default)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Coord {
    pub(crate) fn new() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    pub(crate) fn xyz(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    pub(crate) fn xy(x: isize, y: isize) -> Self {
        Self { x, y, z: 0 }
    }

    pub(crate) fn from_2d(&mut self, c: &Coord2D) {
        self.x = c.x;
        self.y = c.y
    }

    pub(crate) fn to_2d(&self) -> Coord2D {
        Coord2D {
            x: self.x,
            y: self.y,
        }
    }

    pub(crate) fn move_rect_to_2d(&self, r: &RectSide) -> Coord2D {
        Coord2D {
            x: self.x + r.width,
            y: self.y + r.height,
        }
    }
}

///two-dimensional coordinate.
#[derive(Clone, Debug, Default)]
pub struct Coord2D {
    pub x: isize,
    pub y: isize,
}

impl Coord2D {
    pub(crate) fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    pub(crate) fn xy(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn move_xy(&self, dx: isize, dy: isize) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    pub(crate) fn from_xy(&mut self, c: &Self) {
        self.x = c.x;
        self.y = c.y;
    }

    pub(crate) fn away_from(&self, c: &Self) -> RectSide {
        RectSide::new(self.x - c.x, self.y - c.y)
    }
}

impl Into<ISize> for Coord2D {
    fn into(self) -> ISize {
        ISize::new(self.x as i32, self.y as i32)
    }
}

impl Into<Point> for Coord2D {
    fn into(self) -> Point {
        Point::new(self.x as scalar, self.y as scalar)
    }
}

///Distance.
#[derive(Debug)]
pub enum Distance {
    Pixel(isize),
    Percentage(usize),
}

impl Distance {
    pub fn from_str(s: &str) -> Option<Self> {
        if let Some((s, _)) = s.split_once(PER_CENT) {
            to_usize(s).map(|i| Self::Percentage(i))
        } else {
            to_isize(s).map(|i| Self::Pixel(i))
        }
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        match self {
            Distance::Pixel(i) => {
                s.push_str(&i.to_string());
            }
            Distance::Percentage(i) => {
                s.push_str(&i.to_string());
                s.push(PER_CENT);
            }
        }
        s
    }

    pub fn pixel(&self) -> Option<isize> {
        match self {
            Self::Pixel(i) => Some(*i),
            Self::Percentage(_) => None,
        }
    }

    pub fn get(&self, n: isize) -> isize {
        match self {
            Self::Pixel(i) => *i,
            Self::Percentage(i) => n * ((*i) as isize) / 100,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Pixel(i) => *i == 0,
            Self::Percentage(i) => *i == 0,
        }
    }
}

///LineSegment.
#[derive(Debug)]
pub(crate) struct LineSegment {
    pub begin: isize,
    pub length: isize,
}

impl LineSegment {
    pub(crate) fn new(begin: isize, length: isize) -> Self {
        Self { begin, length }
    }

    pub(crate) fn end(&self) -> isize {
        self.begin + self.length
    }

    pub(crate) fn max_begin(&mut self, max: isize) {
        if self.begin > max {
            self.begin = max
        }
    }

    pub(crate) fn move_seg(&mut self, n: isize) {
        self.begin += n;
    }

    pub(crate) fn finite_move(&mut self, n: isize, min: isize, max: isize) {
        let p = self.begin + n;
        if p >= min {
            if p <= max {
                self.begin = p;
            } else {
                self.begin = max;
            }
        } else {
            self.begin = min;
        }
    }
}

///Ordinal.
#[derive(Debug)]
pub enum Ordinal {
    Number(usize),
    X(usize),
    Y(usize),
    XY(usize, usize),
    None,
}

impl Ordinal {
    pub fn from_str(s: &str) -> Self {
        let s = s.trim();
        if s.is_empty() {
            return Self::None;
        }
        if let Some(s) = s.split_once(COMMA) {
            match to_usize(s.0) {
                Some(x) => match to_usize(s.1) {
                    Some(y) => Self::XY(x, y),
                    None => Self::X(x),
                },
                None => match to_usize(s.1) {
                    Some(y) => Self::Y(y),
                    None => Self::None,
                },
            }
        } else {
            match to_usize(s) {
                Some(n) => Self::Number(n),
                None => Self::None,
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        match self {
            Ordinal::Number(n) => {
                s.push_str(&n.to_string());
            }
            Ordinal::X(x) => {
                s.push_str(&x.to_string());
                s.push(COMMA);
            }
            Ordinal::Y(y) => {
                s.push(COMMA);
                s.push_str(&y.to_string());
            }
            Ordinal::XY(x, y) => {
                s.push_str(&x.to_string());
                s.push(COMMA);
                s.push_str(&y.to_string());
            }
            Ordinal::None => {}
        }
        s
    }
}

///Points.
#[derive(Debug)]
pub struct Points {
    pub data: Vec<Distance>,
    pub count: usize,
}

impl Points {
    pub(crate) fn new() -> Self {
        Self {
            data: Vec::new(),
            count: 0,
        }
    }

    pub fn from_str(mut s: &str) -> Self {
        if s.is_empty() {
            return Self::new();
        }

        if let Some(t) = s.split_once(LEFT_SQUARE_BRACKET) {
            s = t.1;
        }
        let mut data_str = "";
        let mut count_str = s;
        if let Some(t) = s.split_once(RIGHT_SQUARE_BRACKET) {
            (data_str, count_str) = t;
        }
        let data = data_str
            .split(COMMA)
            .filter_map(|k| Distance::from_str(k))
            .collect();
        if let Some(k) = count_str.split(COMMA).find(|k| !k.trim().is_empty()) {
            count_str = k;
        }
        let mut count = 0;
        if let Some(i) = to_usize(count_str) {
            count = i;
        }
        Self { data, count }
    }

    ///Returns a string.
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        if self.data.len() > 0 {
            s.push(LEFT_SQUARE_BRACKET);
            let v: Vec<String> = self.data.iter().map(|o| o.to_string()).collect();
            s.push_str(&v.join(&COMMA.to_string()));
            s.push(RIGHT_SQUARE_BRACKET);
            s.push(COMMA);
        }
        s.push_str(&self.count.to_string());
        s
    }

    fn effect(&self, sum: isize) -> Vec<LineSegment> {
        let mut v = Vec::new();
        let mut o = 0;
        for i in &self.data {
            o = i.get(sum);
            v.push(o);
        }
        //one point at least.
        if v.is_empty() {
            v.push(o);
        }
        let mut k = 0;
        if o < sum {
            k = sum - o;
        }
        let count = self.count;
        let n = v.len();
        if n < count {
            let n = (count - n + 1) as isize;
            k = k / n;
            for _ in 0..n - 1 {
                o = o + k;
                v.push(o);
            }
        }
        v.reverse();
        let mut r = Vec::new();
        let mut k = sum;
        for i in v {
            if i < k {
                r.push(LineSegment::new(i, k - i));
                k = i;
            } else {
                r.push(LineSegment::new(i, 0));
            }
        }
        r.reverse();
        r
    }

    pub(crate) fn coord(&self, sum: isize, zero: isize) -> Vec<LineSegment> {
        let mut v = self.effect(sum);
        v.iter_mut().for_each(|i| i.move_seg(zero));
        v
    }
}

///RectSide.
#[derive(Clone, Debug)]
pub struct RectSide {
    pub width: isize,
    pub height: isize,
}

impl RectSide {
    pub(crate) fn new(width: isize, height: isize) -> Self {
        Self { width, height }
    }

    pub(crate) fn empty() -> Self {
        Self::new(0, 0)
    }

    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    pub(crate) fn width_add(&mut self, n: isize) {
        self.width += n;
    }

    pub(crate) fn height_add(&mut self, n: isize) {
        self.height += n;
    }

    pub(crate) fn width_sub(&self, r: &Self) -> isize {
        self.width - r.width
    }

    pub(crate) fn height_sub(&self, r: &Self) -> isize {
        self.height - r.height
    }

    pub(crate) fn add(&self, w: isize, h: isize) -> Self {
        Self {
            width: self.width + w,
            height: self.height + h,
        }
    }

    pub(crate) fn same_ratio(&self, r: &Self, r2: &Self) -> Self {
        Self {
            width: self.width * r.width / r2.width,
            height: self.height * r.height / r2.height,
        }
    }

    pub(crate) fn replace(&mut self, r: &Self) {
        self.width = r.width;
        self.height = r.height;
    }

    pub(crate) fn to_rect(&self, zero: &Coord) -> FixedRect {
        FixedRect {
            pos: zero.to_2d(),
            side: self.clone(),
        }
    }

    pub(crate) fn to_isize(&self) -> ISize {
        ISize::new(self.width as i32, self.height as i32)
    }
}

///FixedRect.
#[derive(Debug)]
pub struct FixedRect {
    pub pos: Coord2D,
    pub side: RectSide,
}

impl FixedRect {
    pub(crate) fn new() -> Self {
        Self {
            pos: Coord2D::new(),
            side: RectSide::empty(),
        }
    }

    pub(crate) fn with_side(width: isize, height: isize) -> Self {
        Self {
            pos: Coord2D::new(),
            side: RectSide::new(width, height),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.side.is_empty()
    }

    pub(crate) fn add(&self, left: isize, top: isize, right: isize, bottom: isize) -> Self {
        Self {
            pos: Coord2D::xy(self.pos.x - left, self.pos.y - top),
            side: self.side.add(left + right, top + bottom),
        }
    }

    pub fn move_xy(&self, dx: isize, dy: isize) -> Self {
        Self {
            pos: self.pos.move_xy(dx, dy),
            side: self.side.clone(),
        }
    }

    pub(crate) fn horizontal_move(&self, n: &LineSegment) -> Self {
        Self {
            pos: self.pos.move_xy(n.begin, 0),
            side: RectSide {
                width: n.length,
                height: self.side.height,
            },
        }
    }

    pub(crate) fn vertical_move(&mut self, n: &LineSegment) -> Self {
        Self {
            pos: self.pos.move_xy(0, n.begin),
            side: RectSide {
                width: self.side.width,
                height: n.length,
            },
        }
    }

    fn right(&self) -> isize {
        self.pos.x + self.side.width
    }

    fn bottom(&self) -> isize {
        self.pos.y + self.side.height
    }

    pub fn right_bottom(&self) -> Coord2D {
        Coord2D::xy(self.right(), self.bottom())
    }

    pub fn within(&self, c: &Coord2D) -> bool {
        between(c.x, self.pos.x, self.right()) && between(c.y, self.pos.y, self.bottom())
    }

    pub(crate) fn width_sub(&self, r: &Self) -> isize {
        self.side.width_sub(&r.side)
    }

    pub(crate) fn height_sub(&self, r: &Self) -> isize {
        self.side.height_sub(&r.side)
    }

    pub(crate) fn from_xy(&mut self, t: &Self) {
        self.pos.from_xy(&t.pos);
    }

    pub(crate) fn horizontal_inset(&mut self, t: &Self, dy: isize) {
        self.pos.x = t.pos.x;
        self.pos.y = t.pos.y + dy;
        self.side.width = t.side.width
    }

    pub(crate) fn vertical_inset(&mut self, t: &Self, dx: isize) {
        self.pos.x = t.pos.x + dx;
        self.pos.y = t.pos.y;
        self.side.height = t.side.height
    }

    pub fn to_irect(&self) -> IRect {
        IRect::from_xywh(
            self.pos.x as i32,
            self.pos.y as i32,
            self.side.width as i32,
            self.side.height as i32,
        )
    }

    pub fn to_rect(&self) -> Rect {
        Rect::from_xywh(
            self.pos.x as f32,
            self.pos.y as f32,
            self.side.width as f32,
            self.side.height as f32,
        )
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance() {
        let s = "100";
        if let Some(d) = Distance::from_str(s) {
            assert_eq!(s, d.to_string());
        }

        let s = "20%";
        if let Some(d) = Distance::from_str(s) {
            assert_eq!(s, d.to_string());
        }
    }

    #[test]
    fn ordinal() {
        let s = "1";
        let o = Ordinal::from_str(s);
        assert_eq!(s, o.to_string());

        let s = "";
        let o = Ordinal::from_str(s);
        assert_eq!(s, o.to_string());

        let s = "1,2";
        let o = Ordinal::from_str(s);
        assert_eq!(s, o.to_string());
    }

    #[test]
    fn points() {
        let p = Points {
            data: vec![Distance::Pixel(100), Distance::Percentage(20)],
            count: 3,
        };
        let v = p.effect(1000);
        assert_eq!((v[0].begin, v[0].length), (100, 100));
        assert_eq!((v[1].begin, v[1].length), (200, 400));
        assert_eq!((v[2].begin, v[2].length), (600, 400));

        let s = "[100,20%],3";
        let p = Points::from_str(s);
        let v = p.effect(1000);
        assert_eq!((v[0].begin, v[0].length), (100, 100));
        assert_eq!((v[1].begin, v[1].length), (200, 400));
        assert_eq!((v[2].begin, v[2].length), (600, 400));
        assert_eq!(s, p.to_string());

        let s = " [ 100 ,, ,] , 3 ,";
        let p = Points::from_str(s);
        let v = p.effect(1000);
        assert_eq!((v[0].begin, v[0].length), (100, 300));
        assert_eq!((v[1].begin, v[1].length), (400, 300));
        assert_eq!((v[2].begin, v[2].length), (700, 300));

        let s = "2";
        let p = Points::from_str(s);
        let v = p.effect(1000);
        assert_eq!((v[0].begin, v[0].length), (0, 500));
        assert_eq!((v[1].begin, v[1].length), (500, 500));
        assert_eq!(s, p.to_string());
    }
}
