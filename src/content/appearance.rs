use super::*;
use skia_safe::{Image, RRect};

#[derive(Debug)]
pub(crate) enum Appearance {
    Rectangle(Rectangle),
    RoundRectangle(RoundRectangle),
    RectangleCurve(RectangleCurve),
    RoundRectCurve(RoundRectCurve),
    Image(ImageHolder),
}

impl Default for Appearance {
    fn default() -> Self {
        Self::Rectangle(Default::default())
    }
}

impl Appearance {
    pub(crate) fn draw(&mut self, rect: &FixedRect, t: &mut DrawCtx) {
        match self {
            Self::Rectangle(o) => o.draw(rect, t),
            Self::RoundRectangle(o) => o.draw(rect, t),
            Self::RectangleCurve(o) => o.draw(rect, t),
            Self::RoundRectCurve(o) => o.draw(rect, t),
            Self::Image(o) => o.draw(rect, t),
        }
    }

    pub(crate) fn within(&self, rect: &FixedRect, c: &Coord2D) -> bool {
        match self {
            Self::Rectangle(o) => o.within(rect, c),
            Self::RoundRectangle(o) => o.within(rect, c),
            Self::RectangleCurve(o) => o.within(rect, c),
            Self::RoundRectCurve(o) => o.within(rect, c),
            Self::Image(o) => o.within(rect, c),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Rectangle {
    pub(crate) color: Color,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self::new(*default_bg_color())
    }
}

impl Rectangle {
    pub(crate) fn new(color: Color) -> Self {
        Self { color }
    }

    pub(crate) fn draw(&mut self, rect: &FixedRect, t: &mut DrawCtx) {
        let paint = &mut t.paint;
        paint.set_color(self.color);
        t.surface.canvas().draw_rect(rect.to_rect(), paint);
    }

    pub(crate) fn within(&self, rect: &FixedRect, c: &Coord2D) -> bool {
        rect.within(c)
    }
}

#[derive(Debug)]
pub(crate) struct RoundRectangle {
    pub(crate) color: Color,
    pub(crate) x_rad: f32,
    pub(crate) y_rad: f32,
}

impl Default for RoundRectangle {
    fn default() -> Self {
        Self::new(*default_bg_color(), 10.0, 10.0)
    }
}

impl RoundRectangle {
    pub(crate) fn new(color: Color, x_rad: f32, y_rad: f32) -> Self {
        Self {
            color,
            x_rad,
            y_rad,
        }
    }

    fn to_rrect(&self, rect: &FixedRect) -> RRect {
        RRect::new_rect_xy(rect.to_rect(), self.x_rad, self.y_rad)
    }

    pub(crate) fn draw(&mut self, rect: &FixedRect, t: &mut DrawCtx) {
        let paint = &mut t.paint;
        paint.set_color(self.color);
        t.surface.canvas().draw_rrect(self.to_rrect(rect), paint);
    }

    pub(crate) fn within(&self, rect: &FixedRect, c: &Coord2D) -> bool {
        rect.within(c)
    }
}

#[derive(Debug)]
pub(crate) struct RectangleCurve {
    pub(crate) color: Color,
    pub(crate) left: f32,
    pub(crate) top: f32,
    pub(crate) right: f32,
    pub(crate) bottom: f32,
}

impl Default for RectangleCurve {
    fn default() -> Self {
        Self::new(*default_border_color(), 1.0, 1.0, 1.0, 1.0)
    }
}

impl RectangleCurve {
    pub(crate) fn new(color: Color, left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            color,
            left,
            top,
            right,
            bottom,
        }
    }

    pub(crate) fn draw(&mut self, rect: &FixedRect, t: &mut DrawCtx) {
        let paint = &mut t.paint;
        paint.set_color(self.color);
        t.surface.canvas().draw_rect(rect.to_rect(), paint);
        let r = rect
            + (
                (self.left, self.top),
                (-self.left - self.right, -self.top - self.bottom),
            );
        paint.set_color(*default_blank_color());
        t.surface.canvas().draw_rect(r.to_rect(), &paint);
    }

    pub(crate) fn within(&self, rect: &FixedRect, c: &Coord2D) -> bool {
        rect.within(c)
    }
}

#[derive(Debug)]
pub(crate) struct RoundRectCurve {
    pub(crate) color: Color,
    pub(crate) x_rad: f32,
    pub(crate) y_rad: f32,
    pub(crate) left: f32,
    pub(crate) top: f32,
    pub(crate) right: f32,
    pub(crate) bottom: f32,
}

impl Default for RoundRectCurve {
    fn default() -> Self {
        Self::new(*default_border_color(), 0.0, 0.0, 1.0, 1.0, 1.0, 1.0)
    }
}

impl RoundRectCurve {
    pub(crate) fn new(
        color: Color,
        x_rad: f32,
        y_rad: f32,
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
    ) -> Self {
        Self {
            color,
            x_rad,
            y_rad,
            left,
            top,
            right,
            bottom,
        }
    }

    fn to_rrect(&self, rect: &FixedRect) -> RRect {
        RRect::new_rect_xy(rect.to_rect(), self.x_rad, self.y_rad)
    }

    pub(crate) fn draw(&mut self, rect: &FixedRect, t: &mut DrawCtx) {
        let paint = &mut t.paint;
        paint.set_color(self.color);
        t.surface.canvas().draw_rrect(self.to_rrect(rect), paint);
        let r = rect
            + (
                (self.left, self.top),
                (-self.left - self.right, -self.top - self.bottom),
            );
        paint.set_color(*default_blank_color());
        t.surface.canvas().draw_rrect(self.to_rrect(&r), paint);
    }

    pub(crate) fn within(&self, rect: &FixedRect, c: &Coord2D) -> bool {
        rect.within(c)
    }
}

#[derive(Debug)]
pub(crate) struct ImageHolder {
    pub(crate) image: Image,
}

impl ImageHolder {
    pub(crate) fn new(image: Image) -> Self {
        Self { image }
    }

    pub(crate) fn draw(&mut self, rect: &FixedRect, t: &mut DrawCtx) {
        t.surface.canvas().draw_image(&self.image, &***rect, None);
    }

    pub(crate) fn within(&self, rect: &FixedRect, c: &Coord2D) -> bool {
        rect.within(c)
    }
}

pub(crate) struct AppearanceComposite {
    buffer: Vec<Appearance>,
}

impl Default for AppearanceComposite {
    fn default() -> Self {
        Self::new(vec![Default::default()])
    }
}

deref!(AppearanceComposite, Vec<Appearance>, buffer);

impl std::fmt::Debug for AppearanceComposite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.buffer.iter()).finish()
    }
}

impl From<Vec<Appearance>> for AppearanceComposite {
    fn from(o: Vec<Appearance>) -> Self {
        Self::new(o)
    }
}

macro_rules! appearance_composite {
    ($t:ty, $o:tt) => {
        impl From<$t> for AppearanceComposite {
            fn from(o: $t) -> Self {
                Self::new(vec![Appearance::$o(o)])
            }
        }
    };
}

appearance_composite!(Rectangle, Rectangle);

appearance_composite!(RoundRectangle, RoundRectangle);

appearance_composite!(RectangleCurve, RectangleCurve);

appearance_composite!(RoundRectCurve, RoundRectCurve);

appearance_composite!(ImageHolder, Image);

impl AppearanceComposite {
    pub fn new(buffer: Vec<Appearance>) -> Self {
        Self { buffer }
    }

    pub(crate) fn draw(&mut self, rect: &FixedRect, t: &mut DrawCtx) {
        let mut i = self.buffer.iter_mut();
        while let Some(o) = i.next_back() {
            o.draw(rect, t);
        }
    }

    pub(crate) fn within(&self, rect: &FixedRect, c: &Coord2D) -> bool {
        for o in &self.buffer {
            if o.within(rect, c) {
                return true;
            }
        }
        false
    }
}
