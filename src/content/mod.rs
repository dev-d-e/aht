mod form;
mod media;
mod other;

use crate::parts::FixedRect;
use crate::utils::color::*;
pub use form::{Button, Form, Inp, Opt, Pt, Select, Time};
pub use media::{Audio, Img, Video};
pub use other::{Canv, Iframe};
use skia_safe::{Canvas, Color, Paint, RRect};

pub trait OutPainter: std::fmt::Debug {
    fn act(&mut self, rect: &FixedRect, canvas: &Canvas);

    fn final_position(&self) -> (isize, isize);
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
    pub(crate) max_x: isize,
    pub(crate) max_y: isize,
}

impl Border {
    pub(crate) fn new() -> Self {
        Border {
            bg_color: BG_COLOR,
            left: 1,
            top: 1,
            right: 1,
            bottom: 1,
            border_color: BORDER_COLOR,
            x_rad: 0,
            y_rad: 0,
            max_x: 0,
            max_y: 0,
        }
    }

    fn max(&mut self, rect: &FixedRect) {
        self.max_x = rect.x + rect.width;
        self.max_y = rect.y + rect.height;
    }
}

impl OutPainter for Border {
    fn act(&mut self, rect: &FixedRect, canvas: &Canvas) {
        let mut paint = Paint::default();
        paint.set_color(self.border_color);
        paint.set_anti_alias(true);
        let r = rect.add(self.left, self.top, self.right, self.bottom);
        self.max(&r);
        let x_rad = self.x_rad as f32;
        let y_rad = self.y_rad as f32;
        let r = RRect::new_rect_xy(r.to_rect(), x_rad, y_rad);
        canvas.draw_rrect(r, &paint);
        paint.set_color(self.bg_color);
        let r = RRect::new_rect_xy(rect.to_rect(), x_rad, y_rad);
        canvas.draw_rrect(r, &paint);
    }

    fn final_position(&self) -> (isize, isize) {
        (self.max_x, self.max_y)
    }
}
