use crate::markup::{Attribute, Page, TypeEntity, AREA, BODY};
use crate::parts::{
    AlignPattern, Coord, FixedRect, Ordinal, Painter, Points, Range, ScrollBar, Sides, Subset,
};
use skia_safe::Canvas;
use std::fmt::Debug;

///"Body" is grid layout.
#[derive(Debug)]
pub struct Body {
    pub subset: Subset,
    pub text: String,
    pub class: String,
    pub id: String,
    pub tip: String,
    pub zero: Coord,
    pub side: Sides,
    pub background: Box<dyn Painter>,
    pub align_pattern: AlignPattern,
    pub grid: Grid,
    scroll_bar: ScrollBar,
    parent: *mut Page,
}

impl Body {
    pub fn new() -> Self {
        Body {
            subset: Subset::new(),
            text: String::new(),
            class: String::new(),
            id: String::new(),
            tip: String::new(),
            zero: Coord::new(),
            side: Sides::percentage(100, 100),
            background: Box::new(Range::new()),
            align_pattern: AlignPattern::left_middle(),
            grid: Grid::new(),
            scroll_bar: ScrollBar::new(),
            parent: std::ptr::null_mut(),
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::CLASS(a) => self.class = a,
            Attribute::COLUMN(a) => self.grid.column = a,
            Attribute::HEIGHT(a) => self.side.height = a,
            Attribute::ID(a) => self.id = a,
            Attribute::ROW(a) => self.grid.row = a,
            Attribute::TIP(a) => self.tip = a,
            Attribute::WIDTH(a) => self.side.width = a,
            _ => {}
        }
    }

    element!(BODY);

    zero!();

    set_parent!(Page);

    pub(crate) fn resize(&mut self, w: isize, h: isize) {
        self.grid
            .xy(self.side.width(w), self.side.height(h), &self.zero);
        self.subset.resize(&mut self.grid);
    }

    pub fn draw(&mut self, canvas: &Canvas) {
        let r = self.side.to_rect(&self.zero);
        if r.is_empty() {
            return;
        }

        self.background.as_mut().act(&r, canvas);
        self.subset.draw(canvas);
    }
}

///"Area" is grid layout.
#[derive(Debug)]
pub struct Area {
    pub subset: Subset,
    pub text: String,
    pub class: String,
    pub hidden: bool,
    pub id: String,
    pub ordinal: Ordinal,
    tip: String,
    pub zero: Coord,
    pub side: Sides,
    pub background: Box<dyn Painter>,
    pub align_pattern: AlignPattern,
    pub grid: Grid,
    scroll_bar: ScrollBar,
    parent: *mut TypeEntity,
}

impl Area {
    pub fn new() -> Self {
        Area {
            subset: Subset::new(),
            text: String::new(),
            class: String::new(),
            hidden: false,
            id: String::new(),
            ordinal: Ordinal::None,
            tip: String::new(),
            zero: Coord::new(),
            side: Sides::percentage(100, 100),
            background: Box::new(Range::new()),
            align_pattern: AlignPattern::left_middle(),
            grid: Grid::new(),
            scroll_bar: ScrollBar::new(),
            parent: std::ptr::null_mut(),
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::CLASS(a) => self.class = a,
            Attribute::COLUMN(a) => self.grid.column = a,
            Attribute::HEIGHT(a) => self.side.height = a,
            Attribute::HIDDEN(a) => self.hidden = a,
            Attribute::ID(a) => self.id = a,
            Attribute::ORDINAL(a) => self.ordinal = a,
            Attribute::ROW(a) => self.grid.row = a,
            Attribute::TIP(a) => self.tip = a,
            Attribute::WIDTH(a) => self.side.width = a,
            _ => {}
        }
    }

    element!(AREA);

    zero!();

    set_parent!();

    pub(crate) fn resize(&mut self, t: Option<FixedRect>) {
        if let Some(t) = t {
            self.zero.x = t.x;
            self.zero.y = t.y;
            self.grid.xy(
                self.side.width(t.width),
                self.side.height(t.height),
                &self.zero,
            );
            self.subset.resize(&mut self.grid);
        } else {
            self.hidden = true;
        }
    }

    pub fn draw(&mut self, canvas: &Canvas) {
        if self.hidden {
            return;
        }

        let r = self.side.to_rect(&self.zero);
        if r.is_empty() {
            return;
        }
        self.background.as_mut().act(&r, canvas);
        self.subset.draw(canvas);
    }
}

///Grid.
#[derive(Debug)]
pub struct Grid {
    pub column: Points,
    pub row: Points,
    x: Vec<(isize, isize)>,
    x_n: usize,
    y: Vec<(isize, isize)>,
    y_n: usize,
}

impl Grid {
    fn new() -> Self {
        Grid {
            column: Points::new(),
            row: Points::new(),
            x: Vec::new(),
            x_n: 0,
            y: Vec::new(),
            y_n: 0,
        }
    }

    pub(crate) fn xy(&mut self, width: isize, height: isize, zero: &Coord) {
        self.x = self.column.coord(width, zero.x);
        self.y = self.row.coord(height, zero.y);
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
                    x: x.0,
                    y: y.0,
                    width: x.1,
                    height: y.1,
                });
            }
        }
        None
    }
}
