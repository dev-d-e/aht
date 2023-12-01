use crate::element::{Coordinate, Element};
use skia_safe::utils::text_utils::Align::{self, Center};
use skia_safe::{Canvas, Font, Paint, Point, Rect};

///"A" is link
pub struct A<'a> {
    canvas: &'a mut Canvas,
    bg_paint: &'a Paint,
    paint: &'a Paint,
    text: &'a String,
    font: &'a Font,
    align: Align,
    coordinate: Coordinate,
    width: u32,
    height: u32,
}

impl<'a> A<'a> {
    pub fn new(
        canvas: &'a mut Canvas,
        bg_paint: &'a Paint,
        paint: &'a Paint,
        text: &'a String,
        font: &'a Font,
    ) -> Self {
        A {
            canvas,
            bg_paint,
            paint,
            text,
            font,
            align: Center,
            coordinate: Coordinate { x: 0, y: 0 },
            width: 0,
            height: 0,
        }
    }

    pub fn set_align(&mut self, align: Align) {
        self.align = align;
    }

    pub fn set_coordinate(&mut self, coordinate: Coordinate) {
        self.coordinate = coordinate;
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }
}

impl<'a> Element for A<'a> {
    fn draw(&mut self) {
        if self.width == 0 || self.height == 0 {
            return;
        }
        let rect = Rect::from_xywh(
            self.coordinate.x as f32,
            self.coordinate.y as f32,
            self.width as f32,
            self.height as f32,
        );
        self.canvas.draw_rect(rect, self.bg_paint);
        let p = Point::new(self.coordinate.x as f32, self.coordinate.y as f32);
        self.canvas
            .draw_str_align(self.text, p, self.font, self.paint, self.align);
    }
}
