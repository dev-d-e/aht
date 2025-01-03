use crate::content::{
    Area, Audio, Button, Canv, Form, Iframe, Img, Inp, Opt, Pt, Select, Time, Video,
};
use crate::global::*;
use crate::markup::Mark;
use crate::parts::{
    Chronograph, Coord, Coord2D, Distance, FixedRect, LineSegment, Ordinal, Points, RectSide,
};
use skia_safe::utils::text_utils::Align;
use skia_safe::{scalar, Canvas, Color, Font, Paint, RRect};
use std::fmt::Debug;
use std::sync::Arc;

///Grid.
#[derive(Debug)]
pub struct Grid {
    x: Vec<LineSegment>,
    x_n: usize,
    y: Vec<LineSegment>,
    y_n: usize,
}

impl Grid {
    pub(crate) fn new() -> Self {
        Self {
            x: Vec::new(),
            x_n: 0,
            y: Vec::new(),
            y_n: 0,
        }
    }

    pub(crate) fn x(&mut self, p: &Points, r: &RectSide, zero: &Coord) {
        self.x = p.coord(r.width, zero.x);
    }

    pub(crate) fn y(&mut self, p: &Points, r: &RectSide, zero: &Coord) {
        self.y = p.coord(r.height, zero.y);
    }

    pub(crate) fn next(&mut self, ordinal: &Ordinal) -> Option<FixedRect> {
        match ordinal {
            Ordinal::Number(i) => {
                let n = self.x.len();
                self.next_xy(i % n, i / n)
            }
            Ordinal::X(x) => self.next_xy(*x, self.y_n),
            Ordinal::Y(y) => self.next_xy(self.x_n, *y),
            Ordinal::XY(x, y) => self.next_xy(*x, *y),
            Ordinal::None => self.next_xy(self.x_n, self.y_n),
        }
    }

    fn next_xy(&mut self, x_n: usize, y_n: usize) -> Option<FixedRect> {
        if let Some(x) = self.x.get(x_n) {
            if let Some(y) = self.y.get(y_n) {
                if x_n + 1 == self.x.len() {
                    self.y_n = y_n + 1;
                    self.x_n = 0;
                } else {
                    self.y_n = y_n;
                    self.x_n = x_n + 1;
                }
                return Some(FixedRect {
                    pos: Coord2D::xy(x.begin, y.begin),
                    side: RectSide {
                        width: x.length,
                        height: y.length,
                    },
                });
            }
        }
        None
    }
}

pub trait Painter: std::fmt::Debug {
    fn set_color(&mut self, color: Color);
    fn act(&mut self, rect: &FixedRect, canvas: &Canvas);
}

///DrawUnit.
#[derive(Debug)]
pub(crate) enum DrawUnit {
    AREA(Area),
    AUDIO(Audio),
    BUTTON(Button),
    CANVAS(Canv),
    FORM(Form),
    IFRAME(Iframe),
    IMG(Img),
    INP(Inp),
    OPTION(Opt),
    PT(Pt),
    SELECT(Select),
    TIME(Time),
    VIDEO(Video),
    None,
}

impl DrawUnit {
    pub(crate) fn from(mark_type: &Mark) -> Self {
        match mark_type {
            Mark::AREA => Self::AREA(Area::new()),
            Mark::AUDIO => Self::AUDIO(Audio::new()),
            Mark::BUTTON => Self::BUTTON(Button::new()),
            Mark::CANVAS => Self::CANVAS(Canv::new()),
            Mark::FORM => Self::FORM(Form::new()),
            Mark::IFRAME => Self::IFRAME(Iframe::new()),
            Mark::IMG => Self::IMG(Img::new()),
            Mark::INP => Self::INP(Inp::new()),
            Mark::OPTION => Self::OPTION(Opt::new()),
            Mark::PT => Self::PT(Pt::new()),
            Mark::SELECT => Self::SELECT(Select::new()),
            Mark::TIME => Self::TIME(Time::new()),
            Mark::VIDEO => Self::VIDEO(Video::new()),
            _ => Self::None,
        }
    }
}

///ApplyFont.
pub(crate) struct ApplyFont {
    name: String,
    color: Option<Color>,
    font: Arc<Font>,
    cursor: bool,
    time_meter: Chronograph,
}

impl std::fmt::Debug for ApplyFont {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApplyFont")
            .field("name", &self.name)
            .field("color", &self.color)
            .field("font", &self.font.typeface().family_name())
            .field("cursor", &self.cursor)
            .field("time_meter", &self.time_meter)
            .finish()
    }
}

impl ApplyFont {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            color: None,
            font: default_font(),
            cursor: false,
            time_meter: Chronograph::new(1000),
        }
    }

    pub fn with(s: &str) -> Option<Self> {
        get_applied(s).map(|font| Self {
            name: s.to_string(),
            color: None,
            font,
            cursor: false,
            time_meter: Chronograph::new(1000),
        })
    }

    pub fn font(&self) -> &Font {
        &self.font
    }

    pub fn color(&self) -> Option<&Color> {
        self.color.as_ref()
    }

    pub(crate) fn set_cursor(&mut self, cursor: bool) {
        self.cursor = cursor;
    }

    pub(crate) fn is_cursor(&self) -> bool {
        self.cursor
    }

    pub(crate) fn draw(
        &mut self,
        rect: &FixedRect,
        align_pattern: &AlignPattern,
        text: &String,
        canvas: &Canvas,
    ) {
        let mut paint = Paint::default();
        if let Some(color) = self.color {
            paint.set_color(color);
        } else {
            paint.set_color(*default_font_color());
        }
        paint.set_anti_alias(true);

        let font = self.font();
        let text_size = font.measure_str(text, Some(&paint));
        let text_w = text_size.1.width() as isize;
        let text_h = text_size.1.height() as isize;
        let (c, a) = align_pattern.font_xy(rect, text_h);
        let point0 = c.move_xy(text_w + 2, 2);
        canvas.draw_str_align(text, c, font, &paint, a);
        if self.cursor {
            if self.time_meter.elapsed() {
                self.time_meter.refresh();
            } else {
                let point1 = point0.move_xy(0, text_h.saturating_neg());
                canvas.draw_line(point0, point1, &paint);
            }
        }
    }
}

impl Drop for ApplyFont {
    fn drop(&mut self) {
        check_applied(&self.name);
    }
}

///Sides.
#[derive(Debug)]
pub(crate) struct Sides {
    pub width: isize,
    pub height: isize,
    pub(crate) effect: RectSide,
}

impl Sides {
    pub(crate) fn new(width: isize, height: isize) -> Self {
        Self {
            width,
            height,
            effect: RectSide::empty(),
        }
    }

    // pub(crate) fn pixel(w: isize, h: isize) -> Self {
    //     Self::new(Distance::Pixel(w), Distance::Pixel(h))
    // }

    // pub(crate) fn percentage(w: usize, h: usize) -> Self {
    //     Self::new(Distance::Percentage(w), Distance::Percentage(h))
    // }

    // pub(crate) fn full_horizontal(n: isize) -> Self {
    //     Self::new(Distance::Percentage(100), Distance::Pixel(n))
    // }

    // pub(crate) fn full_vertical(n: isize) -> Self {
    //     Self::new(Distance::Pixel(n), Distance::Percentage(100))
    // }

    // pub fn is_empty(&self) -> bool {
    //     self.width.is_empty() || self.height.is_empty()
    // }

    pub(crate) fn value(&mut self, w: &Distance, h: &Distance) -> &RectSide {
        self.effect.width = w.get(self.width);
        self.effect.height = h.get(self.height);
        &self.effect
    }

    pub(crate) fn value_with(&mut self, r: &RectSide) -> &RectSide {
        //     self.value(r.width, r.height)
        self.effect.width = r.width;
        self.effect.height = r.height;
        &self.effect
    }

    pub(crate) fn to_rect(&self, zero: &Coord) -> FixedRect {
        if self.effect.width == 0 {
            //self.effect.width = self.width.get(0);
        }
        if self.effect.height == 0 {
            //self.effect.height = self.height.get(0);
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
            color: *default_bg_color(),
            x_rad: 0,
            y_rad: 0,
        }
    }

    pub(crate) fn to_rrect(&self, rect: &FixedRect) -> RRect {
        RRect::new_rect_xy(rect.to_rect(), self.x_rad as scalar, self.y_rad as scalar)
    }
}

impl Painter for Range {
    fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    fn act(&mut self, rect: &FixedRect, canvas: &Canvas) {
        let mut paint = Paint::default();
        paint.set_color(self.color);
        paint.set_anti_alias(true);
        canvas.draw_rrect(self.to_rrect(rect), &paint);
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
