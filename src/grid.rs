use crate::element::{Coordinate, Element};
use skia_safe::{Canvas, Paint, Rect};

fn calculate_point<'a>(sum: u32, segments: &'a [u32], quantity: u32, v: &mut Vec<u32>) {
    let mut point: u32 = 0;
    v.push(point);
    for i in 0..segments.len() {
        point = point + segments[i];
        v.push(point);
    }
    v.pop();
    let v_len = v.len() as u32;
    if quantity > 0 && v_len != quantity {
        if v_len > quantity {
            v.truncate(quantity as usize);
        } else if point < sum {
            let n = quantity - v_len;
            let segment = (sum - point) / n;
            for _ in 0..n {
                v.push(point);
                point = point + segment;
            }
        }
    }
}

///"Body" is grid layout
pub struct Body<'a> {
    canvas: &'a Canvas,
    paint: &'a Paint,
    w_segments: &'a [u32],
    h_segments: &'a [u32],
    column: u32,
    row: u32,
    coordinate: Coordinate,
    width: u32,
    height: u32,
    w_point: Vec<u32>,
    h_point: Vec<u32>,
}

impl<'a> Body<'a> {
    pub fn new(
        canvas: &'a Canvas,
        paint: &'a Paint,
        w_segments: &'a [u32],
        h_segments: &'a [u32],
        column: u32,
        row: u32,
    ) -> Self {
        Body {
            canvas,
            paint,
            w_segments,
            h_segments,
            column,
            row,
            coordinate: Coordinate { x: 0, y: 0 },
            width: 0,
            height: 0,
            w_point: Vec::new(),
            h_point: Vec::new(),
        }
    }

    pub fn set_coordinate(&mut self, coordinate: Coordinate) {
        self.coordinate = coordinate;
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        calculate_point(width, self.w_segments, self.column, &mut self.w_point);
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        calculate_point(height, self.h_segments, self.row, &mut self.h_point);
    }

    pub fn contained_points(&mut self) -> (Vec<u32>, Vec<u32>) {
        let x = self.coordinate.x;
        let y = self.coordinate.y;
        let w = self.w_point.iter().map(|p| x + p).collect();
        let h = self.h_point.iter().map(|p| y + p).collect();
        (w, h)
    }
}

impl<'a> Element for Body<'a> {
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
        self.canvas.draw_rect(rect, self.paint);
    }
}

///"Area" is grid layout
pub struct Area<'a> {
    canvas: &'a Canvas,
    paint: &'a Paint,
    w_segments: &'a [u32],
    h_segments: &'a [u32],
    column: u32,
    row: u32,
    coordinate: Coordinate,
    width: u32,
    height: u32,
    w_point: Vec<u32>,
    h_point: Vec<u32>,
}

impl<'a> Area<'a> {
    pub fn new(
        canvas: &'a Canvas,
        paint: &'a Paint,
        w_segments: &'a [u32],
        h_segments: &'a [u32],
        column: u32,
        row: u32,
    ) -> Self {
        Area {
            canvas,
            paint,
            w_segments,
            h_segments,
            column,
            row,
            coordinate: Coordinate { x: 0, y: 0 },
            width: 0,
            height: 0,
            w_point: Vec::new(),
            h_point: Vec::new(),
        }
    }

    pub fn set_coordinate(&mut self, coordinate: Coordinate) {
        self.coordinate = coordinate;
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        calculate_point(width, self.w_segments, self.column, &mut self.w_point);
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        calculate_point(height, self.h_segments, self.row, &mut self.h_point);
    }

    pub fn contained_points(&mut self) -> (Vec<u32>, Vec<u32>) {
        let x = self.coordinate.x;
        let y = self.coordinate.y;
        let w = self.w_point.iter().map(|p| x + p).collect();
        let h = self.h_point.iter().map(|p| y + p).collect();
        (w, h)
    }
}

impl<'a> Element for Area<'a> {
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
        self.canvas.draw_rect(rect, self.paint);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_point() {
        let mut v: Vec<u32> = Vec::new();
        calculate_point(1000, &[100, 100], 0, &mut v);
        assert_eq!(v, [0, 100]);

        v.clear();
        calculate_point(1000, &[100, 100], 1, &mut v);
        assert_eq!(v, [0]);

        v.clear();
        calculate_point(1000, &[100, 100], 2, &mut v);
        assert_eq!(v, [0, 100]);

        v.clear();
        calculate_point(1000, &[100, 100], 3, &mut v);
        assert_eq!(v, [0, 100, 200]);

        v.clear();
        calculate_point(1000, &[100, 100], 4, &mut v);
        assert_eq!(v, [0, 100, 200, 600]);

        v.clear();
        calculate_point(1000, &[], 2, &mut v);
        assert_eq!(v, [0, 500]);
    }
}
