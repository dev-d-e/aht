use skia_safe::{Canvas, Color, IRect, Paint, Rect};

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

///Shape.
#[derive(Debug)]
pub enum Shape {
    Rectangle(IRect),
    Circle(i32, i32, i32),
    Polygon(Vec<(i32, i32)>),
}

///"HorizontalScrollBar" is horizontal scroll bar
#[derive(Debug)]
pub(crate) struct HorizontalScrollBar {
    range: IRect,
    background: Color,
    cursor_x: i32,
    cursor_width: i32,
}

impl HorizontalScrollBar {
    pub fn new() -> Self {
        HorizontalScrollBar {
            range: IRect::new_empty(),
            background: Color::WHITE,
            cursor_x: 0,
            cursor_width: 0,
        }
    }

    range_background!();

    cursor_x!();

    pub fn draw(&mut self, canvas: &Canvas) {}
}

///"VerticalScrollBar" is vertical scroll bar
#[derive(Debug)]
pub(crate) struct VerticalScrollBar {
    range: IRect,
    background: Color,
    cursor_y: i32,
    cursor_height: i32,
}

impl VerticalScrollBar {
    pub fn new() -> Self {
        VerticalScrollBar {
            range: IRect::new_empty(),
            background: Color::WHITE,
            cursor_y: 0,
            cursor_height: 0,
        }
    }

    range_background!();

    cursor_y!();

    pub fn draw(&mut self, canvas: &Canvas) {}
}
