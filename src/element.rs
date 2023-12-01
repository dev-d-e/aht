///draw element
pub trait Element {
    fn draw(&mut self);
}

///point on rectangular coordinates
pub struct Coordinate {
    pub x: u32,
    pub y: u32,
}
