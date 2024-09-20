use crate::grid::Grid;
use crate::markup::{Page, TypeEntity};
use crate::utils::ascii::*;
use crate::utils::color::*;
use crate::utils::{between, get_font, to_isize, to_usize};
use skia_safe::utils::text_utils::Align;
use skia_safe::{scalar, Canvas, Color, Font, IRect, ISize, Paint, Point, RRect, Rect};
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
        Self {
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
                    $o.zero.from_2d(&t.pos);
                    $o.side.value_with(&t.side);
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

    pub(crate) fn right_bottom(&self) -> Coord2D {
        let mut x = isize::MIN;
        let mut y = isize::MIN;
        macro_rules! size {
            ($o:ident) => {{
                if !$o.hidden {
                    let n = $o.zero.x + $o.side.effect.width;
                    if x < n {
                        x = n;
                    }
                    let n = $o.zero.y + $o.side.effect.height;
                    if y < n {
                        y = n;
                    }
                }
            }};
        }
        macro_rules! size1 {
            ($o:ident) => {{
                if !$o.hidden {
                    let f = $o.outside.final_position();
                    if x < f.x {
                        x = f.x;
                    }
                    if y < f.y {
                        y = f.y;
                    }
                }
            }};
        }
        macro_rules! size2 {
            ($o:ident) => {{
                let f = $o.outside.final_position();
                if x < f.x {
                    x = f.x;
                }
                if y < f.y {
                    y = f.y;
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
                TypeEntity::OPTION(o) => size2!(o),
                TypeEntity::PT(o) => size1!(o),
                TypeEntity::SELECT(o) => size1!(o),
                TypeEntity::TIME(o) => size1!(o),
                TypeEntity::VIDEO(o) => size!(o),
                _ => {}
            }
        }
        Coord2D::xy(x, y)
    }

    pub(crate) fn draw(&mut self, canvas: &Canvas, page: &mut Page) {
        let subset = &mut self.vec;
        for e in subset {
            match e {
                TypeEntity::AREA(o) => o.draw(canvas, page),
                TypeEntity::AUDIO(o) => o.draw(canvas, page),
                TypeEntity::BUTTON(o) => o.draw(canvas, page),
                TypeEntity::CANVAS(o) => o.draw(canvas, page),
                TypeEntity::IFRAME(o) => o.draw(canvas, page),
                TypeEntity::IMG(o) => o.draw(canvas, page),
                TypeEntity::INP(o) => o.draw(canvas, page),
                TypeEntity::OPTION(o) => o.draw(canvas, page),
                TypeEntity::PT(o) => o.draw(canvas, page),
                TypeEntity::SELECT(o) => o.draw(canvas, page),
                TypeEntity::TIME(o) => o.draw(canvas, page),
                TypeEntity::VIDEO(o) => o.draw(canvas, page),
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
        Self {
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
        let (c, a) = align_pattern.font_xy(rect, font.size() as isize);
        let mut paint = Paint::default();
        paint.set_color(self.color);
        paint.set_anti_alias(true);
        canvas.draw_str_align(text, c, font, &paint, a);
    }
}

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

///Sides.
#[derive(Debug)]
pub struct Sides {
    pub width: Distance,
    pub height: Distance,
    pub(crate) effect: RectSide,
}

impl Sides {
    pub(crate) fn new(width: Distance, height: Distance) -> Self {
        Self {
            width,
            height,
            effect: RectSide::empty(),
        }
    }

    pub(crate) fn pixel(w: isize, h: isize) -> Self {
        Self::new(Distance::Pixel(w), Distance::Pixel(h))
    }

    pub(crate) fn percentage(w: usize, h: usize) -> Self {
        Self::new(Distance::Percentage(w), Distance::Percentage(h))
    }

    pub(crate) fn full_horizontal(n: isize) -> Self {
        Self::new(Distance::Percentage(100), Distance::Pixel(n))
    }

    pub(crate) fn full_vertical(n: isize) -> Self {
        Self::new(Distance::Pixel(n), Distance::Percentage(100))
    }

    pub fn is_empty(&self) -> bool {
        self.width.is_empty() || self.height.is_empty()
    }

    pub(crate) fn value(&mut self, w: isize, h: isize) -> &RectSide {
        self.effect.width = self.width.get(w);
        self.effect.height = self.height.get(h);
        &self.effect
    }

    pub(crate) fn value_with(&mut self, r: &RectSide) -> &RectSide {
        self.value(r.width, r.height)
    }

    pub(crate) fn to_rect(&mut self, zero: &Coord) -> FixedRect {
        if self.effect.width == 0 {
            self.effect.width = self.width.get(0);
        }
        if self.effect.height == 0 {
            self.effect.height = self.height.get(0);
        }
        FixedRect {
            pos: zero.to_2d(),
            side: self.effect.clone(),
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
        Self {
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

    pub fn font_xy(&self, rect: &FixedRect, size: isize) -> (Coord2D, Align) {
        let mut x = rect.pos.x;
        let mut font_align = Align::Left;
        match self.horizontal {
            HorizontalAlign::Left => {}
            HorizontalAlign::Center => {
                font_align = Align::Center;
                x = x + rect.side.width / 2;
            }
            HorizontalAlign::Right => {
                font_align = Align::Right;
                x = x + rect.side.width;
            }
        }

        let mut y = rect.pos.y;
        match self.vertical {
            VerticalAlign::Top => {
                y += size;
            }
            VerticalAlign::Middle => {
                y += rect.side.height / 2 + size / 2;
            }
            VerticalAlign::Bottom => {
                y += rect.side.height;
            }
        }
        (Coord2D::xy(x, y), font_align)
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
        Self {
            color: BG_COLOR,
            x_rad: 0,
            y_rad: 0,
        }
    }

    pub(crate) fn to_rrect(&self, rect: &FixedRect) -> RRect {
        RRect::new_rect_xy(rect.to_rect(), self.x_rad as scalar, self.y_rad as scalar)
    }
}

impl Painter for Range {
    fn act(&mut self, rect: &FixedRect, canvas: &Canvas) {
        let mut paint = Paint::default();
        paint.set_color(self.color);
        paint.set_anti_alias(true);
        canvas.draw_rrect(self.to_rrect(rect), &paint);
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

///ScrollBarType.
#[derive(Debug)]
pub enum ScrollBarType {
    Both,
    Horizontal,
    Vertical,
}

///"ScrollBar" include horizontal scroll bar & vertical scroll bar.
#[derive(Debug)]
pub(crate) struct ScrollBar {
    scroll_bar_type: ScrollBarType,
    pub(crate) background: Box<dyn Painter>,
    pub(crate) foreground: Box<dyn Painter>,
    hor_show: bool,
    hor_fr: FixedRect,
    hor_fore: LineSegment,
    ver_show: bool,
    ver_fr: FixedRect,
    ver_fore: LineSegment,
}

impl ScrollBar {
    pub(crate) fn new() -> Self {
        Self {
            scroll_bar_type: ScrollBarType::Both,
            background: Box::new(range!(Color::CYAN)),
            foreground: Box::new(range!(Color::YELLOW)),
            hor_show: false,
            hor_fr: FixedRect::with_side(0, 10),
            hor_fore: LineSegment::new(0, 0),
            ver_show: false,
            ver_fr: FixedRect::with_side(10, 0),
            ver_fore: LineSegment::new(0, 0),
        }
    }

    pub(crate) fn horizontal() -> Self {
        let mut s = Self::new();
        s.scroll_bar_type = ScrollBarType::Horizontal;
        s
    }

    pub(crate) fn vertical() -> Self {
        let mut s = Self::new();
        s.scroll_bar_type = ScrollBarType::Vertical;
        s
    }

    pub(crate) fn cursor_move(&mut self, point: &Coord2D, displacement: &RectSide) {
        match self.scroll_bar_type {
            ScrollBarType::Both => {
                if self.hor_fr.within(point) {
                    if displacement.width == 0 {
                        return;
                    }
                    let max = self.hor_fr.side.width;
                    self.hor_fore.finite_move(displacement.width, 0, max);
                } else if self.ver_fr.within(point) {
                    if displacement.height == 0 {
                        return;
                    }
                    let max = self.ver_fr.side.height;
                    self.ver_fore.finite_move(displacement.height, 0, max);
                }
            }
            ScrollBarType::Horizontal => {
                if self.hor_fr.within(point) {
                    if displacement.width == 0 {
                        return;
                    }
                    let max = self.hor_fr.side.width;
                    self.hor_fore.finite_move(displacement.width, 0, max);
                }
            }
            ScrollBarType::Vertical => {
                if self.ver_fr.within(point) {
                    if displacement.height == 0 {
                        return;
                    }
                    let max = self.ver_fr.side.height;
                    self.ver_fore.finite_move(displacement.height, 0, max);
                }
            }
        }
    }

    pub(crate) fn resize(&mut self, r: &FixedRect, max: &RectSide) -> RectSide {
        let vision = &r.side;
        if vision.is_empty() {
            return RectSide::new(0, 0);
        }

        match self.scroll_bar_type {
            ScrollBarType::Both => {
                if vision.height < max.height {
                    self.ver_show = true;
                    self.ver_fr.vertical_inset(r, r.width_sub(&self.ver_fr));
                    self.ver_fore.length = vision.height * vision.height / max.height;
                    let h = self.ver_fr.side.height - self.ver_fore.length;
                    self.ver_fore.max_begin(h);
                } else {
                    self.ver_show = false;
                    self.ver_fore.begin = 0;
                }

                if vision.width < max.width {
                    self.hor_show = true;
                    self.hor_fr.horizontal_inset(r, r.height_sub(&self.hor_fr));
                    self.hor_fore.length = vision.width * vision.width / max.width;
                    //deal with coincidence
                    if self.ver_show && !self.ver_fr.is_empty() {
                        self.hor_fr.side.width -= self.ver_fr.side.width;
                    }
                    let w = self.hor_fr.side.width - self.hor_fore.length;
                    self.hor_fore.max_begin(w);
                } else {
                    self.hor_show = false;
                    self.hor_fore.begin = 0;
                }
                RectSide::new(
                    self.hor_fore.begin * max.width / vision.width,
                    self.ver_fore.begin * max.height / vision.height,
                )
            }
            ScrollBarType::Horizontal => {
                if vision.width < max.width {
                    self.hor_show = true;
                    self.hor_fr.horizontal_inset(r, r.height_sub(&self.hor_fr));
                    self.hor_fore.length = vision.width * vision.width / max.width;
                    let w = self.hor_fr.side.width - self.hor_fore.length;
                    self.hor_fore.max_begin(w);
                } else {
                    self.hor_show = false;
                    self.hor_fore.begin = 0;
                }
                RectSide::new(self.hor_fore.begin * max.width / vision.width, 0)
            }
            ScrollBarType::Vertical => {
                if vision.height < max.height {
                    self.ver_show = true;
                    self.ver_fr.vertical_inset(r, r.width_sub(&self.ver_fr));
                    self.ver_fore.length = vision.height * vision.height / max.height;
                    let h = self.ver_fr.side.height - self.ver_fore.length;
                    self.ver_fore.max_begin(h);
                } else {
                    self.ver_show = false;
                    self.ver_fore.begin = 0;
                }
                RectSide::new(0, self.ver_fore.begin * max.height / vision.height)
            }
        }
    }

    fn hor_draw(&mut self, canvas: &Canvas) {
        if self.hor_show && !self.hor_fr.is_empty() {
            self.background.act(&self.hor_fr, canvas);
            let fg = self.hor_fr.horizontal_move(&self.hor_fore);
            self.foreground.act(&fg, canvas);
        }
    }

    fn ver_draw(&mut self, canvas: &Canvas) {
        if self.ver_show && !self.ver_fr.is_empty() {
            self.background.act(&self.ver_fr, canvas);
            let fg = self.ver_fr.vertical_move(&self.ver_fore);
            self.foreground.act(&fg, canvas);
        }
    }

    pub(crate) fn draw(&mut self, canvas: &Canvas) {
        match self.scroll_bar_type {
            ScrollBarType::Both => {
                self.ver_draw(canvas);
                self.hor_draw(canvas);
            }
            ScrollBarType::Horizontal => {
                self.hor_draw(canvas);
            }
            ScrollBarType::Vertical => {
                self.ver_draw(canvas);
            }
        }
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
        let v = p.effect(1000);
        assert_eq!((v[0].begin, v[0].length), (100, 100));
        assert_eq!((v[1].begin, v[1].length), (200, 400));
        assert_eq!((v[2].begin, v[2].length), (600, 400));

        let s = "[100,20%] ,3";
        let p = Points::from_str(s);
        let v = p.effect(1000);
        assert_eq!((v[0].begin, v[0].length), (100, 100));
        assert_eq!((v[1].begin, v[1].length), (200, 400));
        assert_eq!((v[2].begin, v[2].length), (600, 400));

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
    }
}
