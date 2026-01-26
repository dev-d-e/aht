use super::*;
use crate::imagesound::*;
use getset::{CopyGetters, Getters, MutGetters, Setters};
use skia_safe::codec::{gif_decoder, jpeg_decoder, png_decoder, webp_decoder};
use skia_safe::codecs::Decoder;
use skia_safe::utils::text_utils::Align;
use skia_safe::{EncodedImageFormat, Font, IRect, Image, Rect, SamplingOptions};
use std::io::Read;
use std::ops::Add;
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;

#[derive(Clone, Default, Getters, MutGetters, Setters)]
#[getset(set = "pub(crate)")]
pub(crate) struct FixedRect {
    pos: Coord,
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    side: RectSide,
}

deref!(FixedRect, Coord, pos);

impl std::fmt::Debug for FixedRect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.pos, self.side)
    }
}

impl FixedRect {
    pub(crate) fn new(pos: Coord, side: RectSide) -> Self {
        Self { pos, side }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.side.is_empty() || self.pos.is_empty()
    }

    pub(crate) fn right(&self) -> f32 {
        self.pos.x() + self.side.width()
    }

    pub(crate) fn bottom(&self) -> f32 {
        self.pos.y() + self.side.height()
    }

    pub(crate) fn right_bottom(&self) -> Coord2D {
        Coord2D::new(self.right(), self.bottom())
    }

    pub(crate) fn within(&self, c: &Coord2D) -> bool {
        between(c.x(), self.pos.x(), self.right()) && between(c.y(), self.pos.y(), self.bottom())
    }

    pub(crate) fn to_rect(&self) -> Rect {
        Rect::from(self)
    }

    pub(crate) fn to_irect(&self) -> IRect {
        IRect::from(self)
    }

    pub(crate) fn get_attr(&mut self, e: &Element, c: &mut LayoutCoord) {
        if let Some(a) = e.attribute().position() {
            self.pos = &c.upper_rect().pos + a;
        } else {
            let a = e.attribute().ordinal().unwrap_or(&Ordinal::None);
            if let Some(pos) = c.next(&a) {
                self.pos = pos;
            } else {
                self.pos.set_x(f32::NAN);
                self.pos.set_y(f32::NAN);
            }
        }
        self.side.get_attr(e, c.upper_rect().side());
    }
}

impl From<&FixedRect> for Rect {
    fn from(o: &FixedRect) -> Self {
        Self::from_xywh(o.pos.x(), o.pos.y(), o.side.width(), o.side.height())
    }
}

impl From<&FixedRect> for IRect {
    fn from(o: &FixedRect) -> Self {
        Self::from_xywh(
            o.pos.x() as i32,
            o.pos.y() as i32,
            o.side.width() as i32,
            o.side.height() as i32,
        )
    }
}

impl Add<(f32, f32)> for &FixedRect {
    type Output = FixedRect;

    fn add(self, other: (f32, f32)) -> Self::Output {
        FixedRect::new(&self.pos + other, self.side.clone())
    }
}

impl Add<(f32, f32, f32)> for &FixedRect {
    type Output = FixedRect;

    fn add(self, other: (f32, f32, f32)) -> Self::Output {
        FixedRect::new(&self.pos + other, self.side.clone())
    }
}

impl Add<((f32, f32), (f32, f32))> for &FixedRect {
    type Output = FixedRect;

    fn add(self, other: ((f32, f32), (f32, f32))) -> Self::Output {
        FixedRect::new(&self.pos + other.0, &self.side + other.1)
    }
}

impl Add<((f32, f32, f32), (f32, f32))> for &FixedRect {
    type Output = FixedRect;

    fn add(self, other: ((f32, f32, f32), (f32, f32))) -> Self::Output {
        FixedRect::new(&self.pos + other.0, &self.side + other.1)
    }
}

impl From<Coord> for FixedRect {
    fn from(o: Coord) -> Self {
        Self::new(o, Default::default())
    }
}

impl From<RectSide> for FixedRect {
    fn from(o: RectSide) -> Self {
        Self::new(Default::default(), o)
    }
}

impl From<(f32, f32)> for FixedRect {
    fn from(o: (f32, f32)) -> Self {
        RectSide::new(o.0, o.1).into()
    }
}

#[derive(Debug, Default, Getters)]
pub(crate) struct LayoutCoord {
    x: Vec<f32>,
    x_n: usize,
    y: Vec<f32>,
    y_n: usize,
    #[getset(get = "pub(crate)")]
    upper_rect: FixedRect,
}

impl LayoutCoord {
    pub(crate) fn get_attr(&mut self, e: &Element, rect: FixedRect) {
        if let Some(a) = e.attribute().column() {
            self.x = a.coord(rect.side().width(), rect.x());
        }
        if let Some(a) = e.attribute().row() {
            self.y = a.coord(rect.side().height(), rect.y());
        }
        self.upper_rect = rect;
    }

    pub(crate) fn next(&mut self, ordinal: &Ordinal) -> Option<Coord> {
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

    fn next_xy(&mut self, x_n: usize, y_n: usize) -> Option<Coord> {
        self.x.get(x_n).and_then(|x| {
            self.y.get(y_n).map(|y| {
                if x_n + 1 == self.x.len() {
                    self.y_n = y_n + 1;
                    self.x_n = 0;
                } else {
                    self.y_n = y_n;
                    self.x_n = x_n + 1;
                }
                Coord::new(*x, *y, self.upper_rect.z())
            })
        })
    }
}

#[derive(Getters, MutGetters)]
pub(crate) struct ApplyFont {
    name: String,
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    color: Option<Color>,
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    font: Arc<Font>,
}

impl std::fmt::Debug for ApplyFont {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApplyFont")
            .field("name", &self.name)
            .field("color", &self.color)
            .field("font", &self.font.typeface().family_name())
            .finish()
    }
}

impl Default for ApplyFont {
    fn default() -> Self {
        Self {
            name: String::new(),
            color: None,
            font: default_font(),
        }
    }
}

impl Drop for ApplyFont {
    fn drop(&mut self) {
        check_applied(&self.name);
    }
}

impl ApplyFont {
    pub(crate) fn new(s: &str) -> Option<Self> {
        get_applied(s).map(|font| Self {
            name: s.to_string(),
            color: None,
            font,
        })
    }

    pub(crate) fn text_size(&self, text: &str, t: &mut DrawCtx) -> Rect {
        let text_size = self.font.measure_str(text, Some(&t.paint));
        text_size.1
    }
}

#[derive(Debug)]
pub(crate) enum HorizontalAlign {
    Left,
    Center,
    Right,
}

#[derive(Debug)]
pub(crate) enum VerticalAlign {
    Top,
    Middle,
    Bottom,
}

#[derive(Debug)]
pub(crate) struct AlignPattern {
    horizontal: HorizontalAlign,
    vertical: VerticalAlign,
}

impl Default for AlignPattern {
    fn default() -> Self {
        Self::left_middle()
    }
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

    pub(crate) fn font_xy(&self, rect: &FixedRect, size: f32) -> (Coord2D, Align) {
        let mut x = rect.x();
        let mut font_align = Align::Left;
        match self.horizontal {
            HorizontalAlign::Left => {}
            HorizontalAlign::Center => {
                font_align = Align::Center;
                x = x + rect.side().width() / 2.0;
            }
            HorizontalAlign::Right => {
                font_align = Align::Right;
                x = x + rect.side().width();
            }
        }

        let mut y = rect.y();
        match self.vertical {
            VerticalAlign::Top => {
                y += size;
            }
            VerticalAlign::Middle => {
                y += rect.side().height() / 2.0 + size / 2.0;
            }
            VerticalAlign::Bottom => {
                y += rect.side().height();
            }
        }
        (Coord2D::new(x, y), font_align)
    }
}

#[derive(CopyGetters, Getters, MutGetters, Setters)]
pub(crate) struct DrawText {
    #[getset(get_copy = "pub(crate)", set = "pub(crate)")]
    cursor: bool,
    #[getset(get = "pub(crate)", get_mut = "pub(crate)", set = "pub(crate)")]
    align_pattern: AlignPattern,
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    apply_font: ApplyFont,
    time_meter: Chronograph,
    interval: f32,
}

impl std::fmt::Debug for DrawText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("DrawText");
        f.field("cursor", &self.cursor)
            .field("align_pattern", &self.align_pattern)
            .field("apply_font", &self.apply_font)
            .finish()
    }
}

impl Default for DrawText {
    fn default() -> Self {
        Self {
            cursor: false,
            align_pattern: Default::default(),
            apply_font: Default::default(),
            time_meter: Chronograph::new(1000),
            interval: 20.0,
        }
    }
}

impl DrawText {
    pub(crate) fn draw(&mut self, rect: &FixedRect, text: &str, t: &mut DrawCtx) {
        let paint = &mut t.paint;
        if let Some(color) = self.apply_font.color() {
            paint.set_color(*color);
        } else {
            paint.set_color(*default_font_color());
        }

        let font = self.apply_font.font();
        let size = self.apply_font.text_size(text, t);
        let text_w = size.width();
        let text_h = size.height();
        let (c, a) = self.align_pattern.font_xy(rect, text_h);
        if text_w <= rect.side.width() {
            let paint = &t.paint;
            t.surface.canvas().draw_str_align(text, &c, font, paint, a);
        } else {
            let rect2 = rect.side.clone().into();
            let (c, a) = self.align_pattern.font_xy(&rect2, text_h);
            t.draw_in_rect(rect, |surface2, paint| {
                surface2.canvas().draw_str_align(text, &c, font, paint, a);
            });
        }

        if self.cursor {
            if self.time_meter.elapsed() {
                self.time_meter.refresh();
            } else {
                let paint = &mut t.paint;
                let point0 = &c + (text_w + 2.0, (self.interval - text_h / 2.0) / 2.0);
                let point1 = &point0 + (0.0, -self.interval);
                paint.set_color(*default_cursor_color());
                t.surface.canvas().draw_line(point0, point1, paint);
            }
        }
    }
}

impl From<AlignPattern> for DrawText {
    fn from(align_pattern: AlignPattern) -> Self {
        Self {
            align_pattern,
            ..Default::default()
        }
    }
}

///ScrollBarType.
#[derive(Debug)]
pub(crate) enum ScrollBarType {
    Both,
    Horizontal,
    Vertical,
}

///"ScrollBar" include horizontal scroll bar & vertical scroll bar.
#[derive(Debug, CopyGetters, Getters, MutGetters, Setters)]
pub(crate) struct ScrollBar {
    scroll_bar_type: ScrollBarType,
    #[getset(set = "pub(crate)")]
    painter: AppearanceComposite,
    #[getset(set = "pub(crate)")]
    scroll_bar: AppearanceComposite,
    hor_show: bool,
    hor_rect: FixedRect,
    hor_f_offset: f32,
    hor_f_length: f32,
    ver_show: bool,
    ver_rect: FixedRect,
    ver_f_offset: f32,
    ver_f_length: f32,
    #[getset(get_copy = "pub(crate)")]
    vision_var: (f32, f32),
}

impl Default for ScrollBar {
    fn default() -> Self {
        Self {
            scroll_bar_type: ScrollBarType::Both,
            painter: Rectangle::new(*default_scroll_bar2_color()).into(),
            scroll_bar: Rectangle::new(*default_scroll_bar_color()).into(),
            hor_show: false,
            hor_rect: (0.0, 20.0).into(),
            hor_f_offset: 0.0,
            hor_f_length: 0.0,
            ver_show: false,
            ver_rect: (20.0, 0.0).into(),
            ver_f_offset: 0.0,
            ver_f_length: 0.0,
            vision_var: Default::default(),
        }
    }
}

impl ScrollBar {
    pub(crate) fn horizontal() -> Self {
        let mut s = Self::default();
        s.scroll_bar_type = ScrollBarType::Horizontal;
        s
    }

    pub(crate) fn vertical() -> Self {
        let mut s = Self::default();
        s.scroll_bar_type = ScrollBarType::Vertical;
        s
    }

    fn ver_resize(&mut self, r: &FixedRect, max_h: f32) {
        let vision_h = r.side().height();

        if vision_h < max_h {
            self.ver_show = true;
            let n = r.right() - self.ver_rect.side().width();
            self.ver_rect.set_x(n);
            self.ver_rect.set_y(r.y());
            self.ver_rect.side_mut().set_height(r.side().height());
            self.ver_f_length = vision_h * vision_h / max_h;
            let n = self.ver_rect.side().height() - self.ver_f_length;
            if self.ver_f_offset > n {
                self.ver_f_offset = n
            }
        } else {
            self.ver_show = false;
            self.ver_f_offset = 0.0;
        }
    }

    fn hor_resize(&mut self, r: &FixedRect, max_w: f32) {
        let vision_w = r.side().width();

        if vision_w < max_w {
            self.hor_show = true;
            self.hor_rect.set_x(r.x());
            let n = r.bottom() - self.hor_rect.side().height();
            self.hor_rect.set_y(n);
            self.hor_rect.side_mut().set_width(r.side().width());
            let n = vision_w * vision_w / max_w;
            self.hor_f_length = n;
            //deal with coincidence
            if self.ver_show && !self.ver_rect.is_empty() {
                let n = self.hor_rect.side().width() - self.ver_rect.side().width();
                self.hor_rect.side_mut().set_width(n);
            }
            let n = self.hor_rect.side().width() - self.hor_f_length;
            if self.hor_f_offset > n {
                self.hor_f_offset = n;
            }
        } else {
            self.hor_show = false;
            self.hor_f_offset = 0.0;
        }
    }

    pub(crate) fn resize(&mut self, r: &FixedRect, max: &RectSide) -> FixedRect {
        if r.is_empty() {
            return r.clone();
        }
        let vision_h = r.side().height();
        let vision_w = r.side().width();
        let max_h = max.height();
        let max_w = max.width();

        self.vision_var = match self.scroll_bar_type {
            ScrollBarType::Both => {
                self.ver_resize(r, max_h);
                self.hor_resize(r, max_w);
                (
                    self.hor_f_offset * max_w / vision_w,
                    self.ver_f_offset * max_h / vision_h,
                )
            }
            ScrollBarType::Horizontal => {
                self.hor_resize(r, max_w);
                (self.hor_f_offset * max_w / vision_w, 0.0)
            }
            ScrollBarType::Vertical => {
                self.ver_resize(r, max_h);
                (0.0, self.ver_f_offset * max_h / vision_h)
            }
        };
        r + self.vision_var
    }

    fn ver_draw(&mut self, t: &mut DrawCtx) {
        let rect = &self.ver_rect;
        if self.ver_show && !rect.is_empty() {
            self.painter.draw(rect, t);
            let r = FixedRect::new(
                &**rect + (0.0, self.ver_f_offset),
                RectSide::new(rect.side().width(), self.ver_f_length),
            );
            self.scroll_bar.draw(&r, t);
        }
    }

    fn hor_draw(&mut self, t: &mut DrawCtx) {
        let rect = &self.hor_rect;
        if self.hor_show && !rect.is_empty() {
            self.painter.draw(rect, t);
            let r = FixedRect::new(
                &**rect + (self.hor_f_offset, 0.0),
                RectSide::new(self.hor_f_length, rect.side().height()),
            );
            self.scroll_bar.draw(&r, t);
        }
    }

    pub(crate) fn draw(&mut self, t: &mut DrawCtx) {
        match self.scroll_bar_type {
            ScrollBarType::Both => {
                self.ver_draw(t);
                self.hor_draw(t);
            }
            ScrollBarType::Horizontal => {
                self.hor_draw(t);
            }
            ScrollBarType::Vertical => {
                self.ver_draw(t);
            }
        }
    }

    pub(crate) fn within(&mut self, o: &Coord2D) -> bool {
        (self.hor_show && self.hor_rect.within(o)) || (self.ver_show && self.ver_rect.within(o))
    }

    fn hor_move(&mut self, o: &Coord2D, dw: f32) -> bool {
        if self.hor_show && self.hor_rect.within(o) {
            if dw != 0.0 {
                let n = self.hor_f_offset + dw;
                self.hor_f_offset = n.min(self.hor_rect.side().width()).max(0.0);
            }
            true
        } else {
            false
        }
    }

    fn ver_move(&mut self, o: &Coord2D, dh: f32) -> bool {
        if self.ver_show && self.ver_rect.within(o) {
            if dh != 0.0 {
                let n = self.ver_f_offset + dh;
                self.ver_f_offset = n.min(self.ver_rect.side().height()).max(0.0);
            }
            true
        } else {
            false
        }
    }

    pub(crate) fn move_a_to_b(&mut self, a: &Coord2D, b: &Coord2D) {
        let d = RectSide::away_from(b, a);
        let dw = d.width();
        let dh = d.height();
        match self.scroll_bar_type {
            ScrollBarType::Both => self.hor_move(b, dw) || self.ver_move(b, dh),
            ScrollBarType::Horizontal => self.hor_move(b, dw),
            ScrollBarType::Vertical => self.ver_move(b, dh),
        };
    }
}

pub(super) fn get_image(
    rect: &RectSide,
    t: &mut DrawCtx,
    f: EncodedImageFormat,
    data: &mut impl Read,
) -> Option<Image> {
    if let Some(decoder) = get_decoder(f) {
        if let Ok(mut c) = decoder.from_stream(data) {
            if let Ok(i) = c.get_image(None, None) {
                let info = t.surface.canvas().image_info();
                let info = info.with_dimensions(rect);
                return i.make_scaled(&info, SamplingOptions::default());
            }
        }
    }
    None
}

fn get_decoder(f: EncodedImageFormat) -> Option<Decoder> {
    match f {
        EncodedImageFormat::GIF => Some(gif_decoder::decoder()),
        EncodedImageFormat::JPEG => Some(jpeg_decoder::decoder()),
        EncodedImageFormat::PNG => Some(png_decoder::decoder()),
        EncodedImageFormat::WEBP => Some(webp_decoder::decoder()),
        _ => None,
    }
}

#[derive(Debug)]
pub(super) struct MediaReader {
    result: Option<ImageDataOutput>,
    receiver: Receiver<ImageDataOutput>,
}

impl MediaReader {
    pub(super) fn new(s: &str, is_output: bool) -> Self {
        let receiver = build(s.to_string(), is_output);
        Self {
            result: None,
            receiver,
        }
    }

    pub(super) fn pause(&mut self, o: bool) {
        if let Some(r) = &mut self.result {
            r.pause(o);
        }
    }

    pub(super) fn seek(&mut self, n: (u32, u32)) {
        if let Some(r) = &mut self.result {
            r.seek(n);
        }
    }

    pub(super) fn draw(&mut self, rect: &FixedRect, t: &mut DrawCtx) {
        if let Some(r) = &mut self.result {
            let info = t.surface.canvas().image_info();
            if let Some(i) = r.data(info, rect.side()) {
                t.surface.canvas().draw_image(i, &***rect, None);
            }
        } else if let Ok(a) = self.receiver.try_recv() {
            self.result.replace(a);
        }
    }

    pub(super) fn no_draw(&mut self) {
        if self.result.is_none() {
            if let Ok(a) = self.receiver.try_recv() {
                self.result.replace(a);
            }
        }
    }
}

impl From<(&String, bool)> for MediaReader {
    fn from(o: (&String, bool)) -> Self {
        Self::new(o.0, o.1)
    }
}
