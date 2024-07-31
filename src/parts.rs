use crate::grid::Grid;
use crate::markup::TypeEntity;
use crate::utils::ascii::*;
use crate::utils::color::*;
use crate::utils::{get_font, to_isize, to_usize};
use skia_safe::utils::text_utils::Align;
use skia_safe::{Canvas, Color, Font, IRect, Paint, RRect, Rect};
use std::cell::OnceCell;
use std::collections::VecDeque;

pub trait Painter: std::fmt::Debug {
    fn act(&mut self, rect: &FixedRect, canvas: &Canvas);
}

///Subset.
#[derive(Debug)]
pub struct Subset {
    pub(crate) vec: VecDeque<TypeEntity>,
}

impl Subset {
    pub(crate) fn new() -> Self {
        Subset {
            vec: VecDeque::new(),
        }
    }

    pub fn push_back(&mut self, e: TypeEntity) {
        self.vec.push_back(e);
    }

    pub(crate) fn set_parent(&mut self, parent_ptr: &mut TypeEntity) {
        let subset = &mut self.vec;
        for e in subset {
            unsafe {
                let self_ptr = e as *mut TypeEntity;
                match e {
                    TypeEntity::AREA(o) => o.set_parent(parent_ptr, &mut *self_ptr),
                    TypeEntity::AUDIO(o) => o.set_parent(parent_ptr, &mut *self_ptr),
                    TypeEntity::BUTTON(o) => o.set_parent(parent_ptr, &mut *self_ptr),
                    TypeEntity::CANVAS(o) => o.set_parent(parent_ptr, &mut *self_ptr),
                    TypeEntity::IFRAME(o) => o.set_parent(parent_ptr, &mut *self_ptr),
                    TypeEntity::IMG(o) => o.set_parent(parent_ptr, &mut *self_ptr),
                    TypeEntity::INP(o) => o.set_parent(parent_ptr, &mut *self_ptr),
                    TypeEntity::OPTION(o) => o.set_parent(parent_ptr),
                    TypeEntity::PT(o) => o.set_parent(parent_ptr, &mut *self_ptr),
                    TypeEntity::SELECT(o) => o.set_parent(parent_ptr, &mut *self_ptr),
                    TypeEntity::TIME(o) => o.set_parent(parent_ptr, &mut *self_ptr),
                    TypeEntity::VIDEO(o) => o.set_parent(parent_ptr, &mut *self_ptr),
                    _ => {}
                }
            }
        }
    }

    pub(crate) fn resize(&mut self, xy: &mut Grid) {
        macro_rules! resize {
            ($o:ident) => {
                if let Some(t) = xy.next(&$o.ordinal) {
                    $o.zero.x = t.x;
                    $o.zero.y = t.y;
                    $o.side.width(t.width);
                    $o.side.height(t.height);
                } else {
                    $o.hidden = true;
                }
            };
        }

        let subset = &mut self.vec;
        for e in subset {
            match e {
                TypeEntity::AREA(o) => resize!(o),
                TypeEntity::AUDIO(o) => resize!(o),
                TypeEntity::BUTTON(o) => resize!(o),
                TypeEntity::CANVAS(o) => resize!(o),
                TypeEntity::IFRAME(o) => resize!(o),
                TypeEntity::IMG(o) => resize!(o),
                TypeEntity::INP(o) => resize!(o),
                //TypeEntity::OPTION(o) => o.resize(),
                TypeEntity::PT(o) => resize!(o),
                TypeEntity::SELECT(o) => resize!(o),
                TypeEntity::TIME(o) => resize!(o),
                TypeEntity::VIDEO(o) => resize!(o),
                _ => {}
            }
        }
    }

    pub(crate) fn size(&self) -> (isize, isize) {
        let mut x = isize::MIN;
        let mut y = isize::MIN;
        macro_rules! size {
            ($o:ident) => {{
                println!("zero:{:?}, {:?}", $o.zero, $o.side);
                let n = $o.zero.x + $o.side.effective_w;
                if x < n {
                    x = n;
                }
                let n = $o.zero.y + $o.side.effective_h;
                if y < n {
                    y = n;
                }
            }};
        }
        macro_rules! size1 {
            ($o:ident) => {{
                let (m, n) = $o.outside.final_position();
                if x < m {
                    x = m;
                }
                if y < n {
                    y = n;
                }
            }};
        }

        let subset = &self.vec;
        for e in subset {
            match e {
                TypeEntity::AREA(o) => size!(o),
                TypeEntity::AUDIO(o) => size!(o),
                TypeEntity::BUTTON(o) => size!(o),
                TypeEntity::CANVAS(o) => size!(o),
                TypeEntity::IFRAME(o) => size!(o),
                TypeEntity::IMG(o) => size1!(o),
                TypeEntity::INP(o) => size1!(o),
                TypeEntity::OPTION(o) => size!(o),
                TypeEntity::PT(o) => size1!(o),
                TypeEntity::SELECT(o) => size1!(o),
                TypeEntity::TIME(o) => size1!(o),
                TypeEntity::VIDEO(o) => size!(o),
                _ => {}
            }
        }
        (x, y)
    }

    pub(crate) fn draw(&mut self, canvas: &Canvas) {
        let subset = &mut self.vec;
        for e in subset {
            match e {
                TypeEntity::AREA(o) => o.draw(canvas),
                TypeEntity::AUDIO(o) => o.draw(canvas),
                TypeEntity::BUTTON(o) => o.draw(canvas),
                TypeEntity::CANVAS(o) => o.draw(canvas),
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

///ApplyFont.
#[derive(Debug)]
pub struct ApplyFont {
    name: String,
    cell: OnceCell<Font>,
    pub color: Color,
}

impl ApplyFont {
    pub(crate) fn new() -> Self {
        ApplyFont {
            name: String::new(),
            cell: OnceCell::new(),
            color: FONT_COLOR,
        }
    }

    pub fn font(&self) -> &Font {
        let s = self.name.as_str();
        &self.cell.get_or_init(|| get_font(s).unwrap())
    }

    pub fn name(&mut self, s: &str) {
        if s.is_empty() {
            return;
        }
        self.name.replace_range(.., s);
    }

    pub(crate) fn draw(
        &self,
        rect: &FixedRect,
        align_pattern: &AlignPattern,
        text: &String,
        canvas: &Canvas,
    ) {
        let font = self.font();
        let t = align_pattern.font_xy(rect, font.size() as isize);
        let mut paint = Paint::default();
        paint.set_color(self.color);
        paint.set_anti_alias(true);
        canvas.draw_str_align(text, (t.0 as i32, t.1 as i32), font, &paint, t.2);
    }
}

///Coord.
#[derive(Debug)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Coord {
    pub(crate) fn new() -> Self {
        Coord { x: 0, y: 0, z: 0 }
    }
}

///FixedRect.
#[derive(Debug)]
pub struct FixedRect {
    pub x: isize,
    pub y: isize,
    pub width: isize,
    pub height: isize,
}

impl FixedRect {
    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    pub fn add(&self, left: isize, top: isize, right: isize, bottom: isize) -> Self {
        FixedRect {
            x: self.x - left,
            y: self.y - top,
            width: self.width + left + right,
            height: self.height + top + bottom,
        }
    }

    pub fn to_irect(&self) -> IRect {
        IRect::from_xywh(
            self.x as i32,
            self.y as i32,
            self.width as i32,
            self.height as i32,
        )
    }

    pub fn to_rect(&self) -> Rect {
        Rect::from_xywh(
            self.x as f32,
            self.y as f32,
            self.width as f32,
            self.height as f32,
        )
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

    pub fn pixel(&self) -> Option<isize> {
        match self {
            Distance::Pixel(i) => Some(*i),
            Distance::Percentage(_) => None,
        }
    }

    pub fn get(&self, n: isize) -> isize {
        match self {
            Distance::Pixel(i) => *i,
            Distance::Percentage(i) => n * ((*i) as isize) / 100,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Distance::Pixel(i) => *i == 0,
            Distance::Percentage(i) => *i == 0,
        }
    }
}

///Sides.
#[derive(Debug)]
pub struct Sides {
    pub width: Distance,
    pub height: Distance,
    pub(crate) effective_w: isize,
    pub(crate) effective_h: isize,
}

impl Sides {
    pub(crate) fn new(width: Distance, height: Distance) -> Self {
        Sides {
            width,
            height,
            effective_w: 0,
            effective_h: 0,
        }
    }

    pub(crate) fn pixel(w: isize, h: isize) -> Self {
        Self::new(Distance::Pixel(w), Distance::Pixel(h))
    }

    pub(crate) fn percentage(w: usize, h: usize) -> Self {
        Self::new(Distance::Percentage(w), Distance::Percentage(h))
    }

    pub fn is_empty(&self) -> bool {
        self.width.is_empty() || self.height.is_empty()
    }

    pub(crate) fn width_pixel(&self) -> Option<isize> {
        self.width.pixel()
    }

    pub(crate) fn width(&mut self, n: isize) -> isize {
        self.effective_w = self.width.get(n);
        self.effective_w
    }

    pub(crate) fn height_pixel(&self) -> Option<isize> {
        self.height.pixel()
    }

    pub(crate) fn height(&mut self, n: isize) -> isize {
        self.effective_h = self.height.get(n);
        self.effective_h
    }

    pub(crate) fn to_rect(&mut self, zero: &Coord) -> FixedRect {
        if self.effective_w == 0 {
            self.width(0);
        }
        if self.effective_h == 0 {
            self.height(0);
        }
        FixedRect {
            x: zero.x,
            y: zero.y,
            width: self.effective_w,
            height: self.effective_h,
        }
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

///AlignPattern.
#[derive(Debug)]
pub struct AlignPattern {
    pub horizontal: HorizontalAlign,
    pub vertical: VerticalAlign,
}

impl AlignPattern {
    pub(crate) fn new(horizontal: HorizontalAlign, vertical: VerticalAlign) -> Self {
        AlignPattern {
            horizontal,
            vertical,
        }
    }

    pub(crate) fn center_middle() -> Self {
        Self::new(HorizontalAlign::Center, VerticalAlign::Middle)
    }

    pub(crate) fn left_middle() -> Self {
        Self::new(HorizontalAlign::Left, VerticalAlign::Middle)
    }

    pub fn font_xy(&self, rect: &FixedRect, size: isize) -> (isize, isize, Align) {
        let mut x = rect.x;
        let mut font_align = Align::Left;
        match self.horizontal {
            HorizontalAlign::Left => {}
            HorizontalAlign::Center => {
                font_align = Align::Center;
                x = x + rect.width / 2;
            }
            HorizontalAlign::Right => {
                font_align = Align::Right;
                x = x + rect.width;
            }
        }

        let mut y = rect.y;
        match self.vertical {
            VerticalAlign::Top => {
                y += size;
            }
            VerticalAlign::Middle => {
                y += rect.height / 2 + size / 2;
            }
            VerticalAlign::Bottom => {
                y += rect.height;
            }
        }
        (x, y, font_align)
    }
}

#[derive(Debug)]
pub(crate) struct Range {
    pub(crate) color: Color,
    pub(crate) x_rad: isize,
    pub(crate) y_rad: isize,
}

impl Range {
    pub(crate) fn new() -> Self {
        Range {
            color: BG_COLOR,
            x_rad: 0,
            y_rad: 0,
        }
    }
}

impl Painter for Range {
    fn act(&mut self, rect: &FixedRect, canvas: &Canvas) {
        let mut paint = Paint::default();
        paint.set_color(self.color);
        paint.set_anti_alias(true);
        let r = RRect::new_rect_xy(rect.to_rect(), self.x_rad as f32, self.y_rad as f32);
        canvas.draw_rrect(r, &paint);
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

///Points.
#[derive(Debug)]
pub struct Points {
    pub data: Vec<Distance>,
    pub count: usize,
}

impl Points {
    pub(crate) fn new() -> Self {
        Points {
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

    pub(crate) fn effect(&self, sum: isize) -> Vec<(isize, isize)> {
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
                r.push((i, k - i));
                k = i;
            } else {
                r.push((i, 0));
            }
        }
        r.reverse();
        r
    }

    pub(crate) fn coord(&self, sum: isize, zero: isize) -> Vec<(isize, isize)> {
        let mut v = self.effect(sum);
        v.iter_mut().for_each(|i| i.0 += zero);
        v
    }
}

///"HorizontalScrollBar" is horizontal scroll bar
#[derive(Debug)]
pub(crate) struct HorizontalScrollBar {
    pub(crate) hidden: bool,
    pub(crate) zero: Coord,
    pub(crate) side: Sides,
    pub(crate) background: Box<dyn Painter>,
    pub(crate) align_pattern: AlignPattern,
    pub(crate) cursor_x: isize,
    pub(crate) cursor_width: isize,
}

impl HorizontalScrollBar {
    pub(crate) fn new() -> Self {
        HorizontalScrollBar {
            hidden: false,
            zero: Coord::new(),
            side: Sides::percentage(100, 1),
            background: Box::new(Range::new()),
            align_pattern: AlignPattern::center_middle(),
            cursor_x: 0,
            cursor_width: 0,
        }
    }

    zero!();

    pub(crate) fn draw(&mut self, canvas: &Canvas) {
        if self.hidden {
            return;
        }

        let r = self.side.to_rect(&self.zero);
        if r.is_empty() {
            return;
        }
        self.background.as_mut().act(&r, canvas);
    }
}

///"VerticalScrollBar" is vertical scroll bar
#[derive(Debug)]
pub(crate) struct VerticalScrollBar {
    pub(crate) hidden: bool,
    pub(crate) zero: Coord,
    pub(crate) side: Sides,
    pub(crate) background: Box<dyn Painter>,
    pub(crate) align_pattern: AlignPattern,
    pub(crate) cursor_y: isize,
    pub(crate) cursor_height: isize,
}

impl VerticalScrollBar {
    pub(crate) fn new() -> Self {
        VerticalScrollBar {
            hidden: false,
            zero: Coord::new(),
            side: Sides::percentage(1, 100),
            background: Box::new(Range::new()),
            align_pattern: AlignPattern::center_middle(),
            cursor_y: 0,
            cursor_height: 0,
        }
    }

    zero!();

    pub(crate) fn draw(&mut self, canvas: &Canvas) {
        if self.hidden {
            return;
        }

        let r = self.side.to_rect(&self.zero);
        if r.is_empty() {
            return;
        }
        self.background.as_mut().act(&r, canvas);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points() {
        let p = Points {
            data: vec![Distance::Pixel(100), Distance::Percentage(20)],
            count: 3,
        };
        assert_eq!(p.effect(1000), [(100, 100), (200, 400), (600, 400)]);
        let s = "[100,20%] ,3";
        let p = Points::from_str(s);
        assert_eq!(p.effect(1000), [(100, 100), (200, 400), (600, 400)]);
        let s = " [ 100 ,, ,] , 3 ,";
        let p = Points::from_str(s);
        assert_eq!(p.effect(1000), [(100, 300), (400, 300), (700, 300)]);
        let s = "2";
        let p = Points::from_str(s);
        assert_eq!(p.effect(1000), [(0, 500), (500, 500)]);
    }
}
