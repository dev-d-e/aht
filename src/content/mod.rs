mod form;
mod media;
mod other;

use crate::parts::{Coord2D, FixedRect};
use crate::utils::color::*;
pub use form::{Button, Form, Inp, Opt, Pt, Select, Time};
pub use media::{Audio, Img, Video};
pub use other::{Canv, Iframe};
use skia_safe::{scalar, Canvas, Color, Paint, RRect};

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
            bg_color: BG_COLOR,
            left: 1,
            top: 1,
            right: 1,
            bottom: 1,
            border_color: BORDER_COLOR,
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
