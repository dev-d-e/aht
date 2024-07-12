use crate::markup::TypeEntity;
use crate::utils::ascii::*;
use crate::utils::{get_font, to_isize, to_usize};
use skia_safe::{Canvas, Color, Font, IRect, Paint, Rect};
use std::cell::OnceCell;
use std::collections::VecDeque;

///Subset.
#[derive(Debug)]
pub struct Subset {
    vec: VecDeque<TypeEntity>,
}

impl Subset {
    pub fn new() -> Self {
        Subset {
            vec: VecDeque::new(),
        }
    }

    pub fn vec(&mut self) -> &mut VecDeque<TypeEntity> {
        &mut self.vec
    }

    pub fn draw(&mut self, canvas: &Canvas) {
        let subset = &mut self.vec;
        for o in subset {
            match o {
                TypeEntity::AREA(o) => o.draw(canvas),
                TypeEntity::AUDIO(o) => o.draw(canvas),
                TypeEntity::BUTTON(o) => o.draw(canvas),
                TypeEntity::CANVAS(o) => o.draw(canvas),
                TypeEntity::DIALOG(o) => o.draw(canvas),
                TypeEntity::IFRAME(o) => o.draw(canvas),
                TypeEntity::IMG(o) => o.draw(canvas),
                TypeEntity::INP(o) => o.draw(canvas),
                TypeEntity::OPTION(o) => o.draw(canvas),
                TypeEntity::PT(o) => o.draw(canvas),
                TypeEntity::SELECT(o) => o.draw(canvas),
                TypeEntity::TIME(o) => o.draw(canvas),
                TypeEntity::VIDEO(o) => o.draw(canvas),
                _ => {}
            }
        }
    }
}

///RectangleRange.
#[derive(Clone, Debug)]
pub struct RectangleRange {
    pub x: isize,
    pub y: isize,
    pub width: isize,
    pub height: isize,
}

impl RectangleRange {
    pub fn new() -> Self {
        RectangleRange {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    pub(crate) fn to_irect(&self) -> IRect {
        IRect::from_xywh(
            self.x as i32,
            self.y as i32,
            self.width as i32,
            self.height as i32,
        )
    }

    pub(crate) fn to_rect(&self) -> Rect {
        Rect::from_xywh(
            self.x as f32,
            self.y as f32,
            self.width as f32,
            self.height as f32,
        )
    }
}

///HorizontalAlign.
#[derive(Debug)]
pub enum HorizontalAlign {
    Left,
    Center,
    Right,
}

///VerticalAlign.
#[derive(Debug)]
pub enum VerticalAlign {
    Top,
    Middle,
    Bottom,
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
            return Ordinal::None;
        }
        if let Some(s) = s.split_once(COMMA) {
            match to_usize(s.0) {
                Some(i) => match to_usize(s.1) {
                    Some(n) => Self::XY(i, n),
                    None => Self::X(i),
                },
                None => match to_usize(s.1) {
                    Some(n) => Self::Y(n),
                    None => Self::None,
                },
            }
        } else {
            match to_usize(s) {
                Some(i) => Self::Number(i),
                None => Self::None,
            }
        }
    }
}

///Shape.
#[derive(Debug)]
pub enum Shape {
    Rectangle(RectangleRange),
    Circle(isize, isize, isize),
    Polygon(Vec<(isize, isize)>),
    Default,
}

///CoordType.
#[derive(Debug)]
pub enum CoordType {
    Pixel(isize),
    Percentage(isize),
}

impl CoordType {
    fn from_str(s: &str) -> Option<Self> {
        if let Some((s, _)) = s.split_once(PER_CENT) {
            to_isize(s).map(|i| Self::Percentage(i))
        } else {
            to_isize(s).map(|i| Self::Pixel(i))
        }
    }
}

///Points.
#[derive(Debug)]
pub struct Points {
    data: Vec<CoordType>,
    count: usize,
}

impl Points {
    pub fn new(data: Vec<CoordType>, count: usize) -> Self {
        Points { data, count }
    }

    pub fn empty() -> Self {
        Self::new(Vec::new(), 0)
    }

    pub fn from_str(mut s: &str) -> Self {
        if s.is_empty() {
            return Points::empty();
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
            .filter_map(|k| CoordType::from_str(k))
            .collect();
        if let Some(k) = count_str.split(COMMA).find(|k| !k.trim().is_empty()) {
            count_str = k;
        }
        let mut count = 0;
        if let Some(i) = to_usize(count_str) {
            count = i;
        }
        Points::new(data, count)
    }

    pub fn effect(&self, sum: isize) -> Vec<isize> {
        let mut v = Vec::new();
        let mut o = 0;
        for i in &self.data {
            match i {
                CoordType::Pixel(i) => {
                    o = *i;
                }
                CoordType::Percentage(i) => {
                    o = sum * (*i) / 100;
                }
            }
            v.push(o);
        }
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
        v
    }
}

///ApplyFont.
#[derive(Debug)]
pub struct ApplyFont {
    name: String,
    cell: OnceCell<Font>,
    color: Color,
}

impl ApplyFont {
    pub fn new() -> Self {
        ApplyFont {
            name: String::new(),
            cell: OnceCell::new(),
            color: Color::BLACK,
        }
    }

    pub fn font(&self) -> &Font {
        let s = &self.name;
        &self.cell.get_or_init(|| get_font(s))
    }

    pub fn name(&mut self, s: &str) {
        self.name.replace_range(.., s);
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

///"HorizontalScrollBar" is horizontal scroll bar
#[derive(Debug)]
pub(crate) struct HorizontalScrollBar {
    range: RectangleRange,
    background: Color,
    cursor_x: isize,
    cursor_width: isize,
}

impl HorizontalScrollBar {
    pub fn new() -> Self {
        HorizontalScrollBar {
            range: RectangleRange::new(),
            background: Color::WHITE,
            cursor_x: 0,
            cursor_width: 0,
        }
    }

    range_background!();

    cursor_x!();

    pub fn draw(&mut self, canvas: &Canvas) {}
}

///"VerticalScrollBar" is vertical scroll bar
#[derive(Debug)]
pub(crate) struct VerticalScrollBar {
    range: RectangleRange,
    background: Color,
    cursor_y: isize,
    cursor_height: isize,
}

impl VerticalScrollBar {
    pub fn new() -> Self {
        VerticalScrollBar {
            range: RectangleRange::new(),
            background: Color::WHITE,
            cursor_y: 0,
            cursor_height: 0,
        }
    }

    range_background!();

    cursor_y!();

    pub fn draw(&mut self, canvas: &Canvas) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points() {
        let p = Points::new(vec![CoordType::Pixel(100), CoordType::Percentage(20)], 3);
        assert_eq!(p.effect(1000), [100, 200, 600]);
        let s = "[100,20%] ,3";
        let p = Points::from_str(s);
        assert_eq!(p.effect(1000), [100, 200, 600]);
        let s = " [ 100 ,, ,] , 3 ,";
        let p = Points::from_str(s);
        assert_eq!(p.effect(1000), [100, 400, 700]);
        let s = "2";
        let p = Points::from_str(s);
        assert_eq!(p.effect(1000), [0, 500]);
    }
}
