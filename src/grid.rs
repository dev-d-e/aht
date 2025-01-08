use crate::content::{
    Area, Audio, Button, Canv, Form, Iframe, Img, Inp, Opt, Pt, Select, Time, Video,
};
use crate::global::*;
use crate::markup::{AttrName, Attribute, Element, Mark};
use crate::parts::{Chronograph, Coord, Coord2D, FixedRect, LineSegment, Ordinal, RectSide};
use skia_safe::utils::text_utils::Align;
use skia_safe::{Canvas, Color, Font, Paint};
use std::sync::Arc;

///Grid.
#[derive(Debug)]
pub(crate) struct Grid {
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

    pub(crate) fn get_attr(&mut self, e: &Element, r: &RectSide, zero: &Coord) {
        if let Some(Attribute::COLUMN(a)) = e.attribute.get(&AttrName::COLUMN) {
            self.x = a.coord(r.width, zero.x);
        }
        if let Some(Attribute::ROW(a)) = e.attribute.get(&AttrName::ROW) {
            self.y = a.coord(r.height, zero.y);
        }
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
    pub(crate) fn new() -> Self {
        Self {
            name: String::new(),
            color: None,
            font: default_font(),
            cursor: false,
            time_meter: Chronograph::new(1000),
        }
    }

    pub(crate) fn with(s: &str) -> Option<Self> {
        get_applied(s).map(|font| Self {
            name: s.to_string(),
            color: None,
            font,
            cursor: false,
            time_meter: Chronograph::new(1000),
        })
    }

    pub(crate) fn font(&self) -> &Font {
        &self.font
    }

    pub(crate) fn color(&self) -> Option<&Color> {
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
    reserve: RectSide,
    effect: RectSide,
}

impl Sides {
    pub(crate) fn new(width: isize, height: isize) -> Self {
        Self {
            reserve: RectSide::new(width, height),
            effect: RectSide::empty(),
        }
    }

    pub(crate) fn reserve(&mut self, width: isize, height: isize) {
        self.reserve.width = width;
        self.reserve.height = height;
        self.effect.width = width;
        self.effect.height = height;
    }

    pub(crate) fn replace(&mut self, r: &RectSide) {
        self.reserve.replace(r);
        self.effect.replace(r);
    }

    pub(crate) fn get_attr(&mut self, e: &Element) {
        if let Some(Attribute::WIDTH(a)) = e.attribute.get(&AttrName::WIDTH) {
            self.effect.width = a.get(self.reserve.width);
        }
        if let Some(Attribute::HEIGHT(a)) = e.attribute.get(&AttrName::HEIGHT) {
            self.effect.height = a.get(self.reserve.height);
        }
    }

    pub(crate) fn effect(&self) -> &RectSide {
        &self.effect
    }

    pub(crate) fn to_rect(&self, zero: &Coord) -> FixedRect {
        self.effect.to_rect(zero)
    }
}

///HorizontalAlign.
#[derive(Debug)]
pub(crate) enum HorizontalAlign {
    Left,
    Center,
    Right,
}

///VerticalAlign.
#[derive(Debug)]
pub(crate) enum VerticalAlign {
    Top,
    Middle,
    Bottom,
}

///AlignPattern.
#[derive(Debug)]
pub(crate) struct AlignPattern {
    horizontal: HorizontalAlign,
    vertical: VerticalAlign,
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

    pub(crate) fn font_xy(&self, rect: &FixedRect, size: isize) -> (Coord2D, Align) {
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
