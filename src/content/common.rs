use crate::global::*;
use crate::parts::{Coord2D, FixedRect, LineSegment, RectSide};
use skia_safe::codec::{gif_decoder, jpeg_decoder, png_decoder, webp_decoder};
use skia_safe::codecs::Decoder;
use skia_safe::{scalar, Canvas, Color, EncodedImageFormat, Image, Paint, RRect, SamplingOptions};
use std::io::Read;

pub trait Painter: std::fmt::Debug {
    fn set_color(&mut self, color: Color);
    fn act(&mut self, rect: &FixedRect, canvas: &Canvas);
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
pub(crate) enum ScrollBarType {
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

pub trait OutPainter: std::fmt::Debug {
    fn act(&mut self, rect: &FixedRect, canvas: &Canvas);

    fn final_position(&self) -> &Coord2D;
}

#[derive(Debug)]
pub(crate) struct Border {
    pub(crate) bg_color: Color,
    pub(crate) left: isize,
    pub(crate) top: isize,
    pub(crate) right: isize,
    pub(crate) bottom: isize,
    pub(crate) border_color: Color,
    pub(crate) x_rad: isize,
    pub(crate) y_rad: isize,
    pub(crate) max: Coord2D,
}

impl Border {
    pub(crate) fn new() -> Self {
        Self {
            bg_color: *default_bg_color(),
            left: 1,
            top: 1,
            right: 1,
            bottom: 1,
            border_color: *default_border_color(),
            x_rad: 0,
            y_rad: 0,
            max: Coord2D::new(),
        }
    }

    fn max(&mut self, rect: &FixedRect) {
        self.max = rect.right_bottom();
    }

    pub(crate) fn to_rrect(&self, rect: &FixedRect) -> RRect {
        RRect::new_rect_xy(rect.to_rect(), self.x_rad as scalar, self.y_rad as scalar)
    }
}

impl OutPainter for Border {
    fn act(&mut self, rect: &FixedRect, canvas: &Canvas) {
        let mut paint = Paint::default();
        paint.set_color(self.border_color);
        paint.set_anti_alias(true);
        let r = rect.add(self.left, self.top, self.right, self.bottom);
        self.max(&r);
        canvas.draw_rrect(self.to_rrect(&r), &paint);

        paint.set_color(self.bg_color);
        canvas.draw_rrect(self.to_rrect(rect), &paint);
    }

    fn final_position(&self) -> &Coord2D {
        &self.max
    }
}

pub(super) fn get_image(
    rect: &RectSide,
    canvas: &Canvas,
    f: EncodedImageFormat,
    data: &mut impl Read,
) -> Option<Image> {
    if let Some(decoder) = get_decoder(f) {
        if let Ok(mut c) = decoder.from_stream(data) {
            if let Ok(i) = c.get_image(None, None) {
                let info = canvas.image_info();
                let info = info.with_dimensions(rect.to_isize());
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
