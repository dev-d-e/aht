use crate::markup::{Attribute, Page, TypeEntity, VisionActionResult, AREA, BODY};
use crate::parts::{
    AlignPattern, Coord, Coord2D, FixedRect, LineSegment, Ordinal, Painter, Points, Range,
    RectSide, ScrollBar, Sides, Subset,
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
}

impl Body {
    pub(crate) fn new() -> Self {
        Self {
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

    set_parent!(TypeEntity);

    pub(crate) fn resize(&mut self, w: isize, h: isize) {
        self.grid.xy(self.side.value(w, h), &self.zero);
        self.subset.resize(&mut self.grid);
    }

    pub fn draw(&mut self, canvas: &Canvas, page: &mut Page) {
        let r = self.side.to_rect(&self.zero);
        if r.is_empty() {
            return;
        }

        self.background.as_mut().act(&r, canvas);
        canvas.save();

        subset_draw(&mut self.subset, page, &mut self.scroll_bar, &r, canvas);
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
    vision_zero: Coord2D,
}

impl Area {
    pub(crate) fn new() -> Self {
        Self {
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
            vision_zero: Coord2D::new(),
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
            self.zero.from_2d(&t.pos);
            self.grid.xy(self.side.value_with(&t.side), &self.zero);
            self.subset.resize(&mut self.grid);
        } else {
            self.hidden = true;
        }
    }

    pub fn draw(&mut self, canvas: &Canvas, page: &mut Page) {
        if self.hidden {
            return;
        }

        let r = self.side.to_rect(&self.zero);
        if r.is_empty() {
            return;
        }

        self.background.as_mut().act(&r, canvas);
        canvas.save();

        subset_draw(&mut self.subset, page, &mut self.scroll_bar, &r, canvas);
    }
}

///Grid.
#[derive(Debug)]
pub struct Grid {
    pub column: Points,
    pub row: Points,
    x: Vec<LineSegment>,
    x_n: usize,
    y: Vec<LineSegment>,
    y_n: usize,
}

impl Grid {
    fn new() -> Self {
        Self {
            column: Points::new(),
            row: Points::new(),
            x: Vec::new(),
            x_n: 0,
            y: Vec::new(),
            y_n: 0,
        }
    }

    pub(crate) fn xy(&mut self, r: &RectSide, zero: &Coord) {
        self.x = self.column.coord(r.width, zero.x);
        self.y = self.row.coord(r.height, zero.y);
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
                    pos: Coord2D::xy(x.begin, y.begin),
                    side: RectSide {
                        width: x.length,
                        height: y.length,
                    },
                });
            }
        }
        None
    }
}

fn subset_draw(
    subset: &mut Subset,
    page: &mut Page,
    scroll_bar: &mut ScrollBar,
    r: &FixedRect,
    canvas: &Canvas,
) {
    if let Some(mut surface) = unsafe { canvas.surface() } {
        let s = subset.right_bottom().away_from(&r.pos);

        if let VisionActionResult::PressSweep(c, a) = page.cursor.analyse() {
            scroll_bar.cursor_move(c, &a);
        }

        let vision_start = scroll_bar.resize(&r, &s);
        let rr = r.move_xy(vision_start.width, vision_start.height);
        let info = surface.image_info().with_dimensions(rr.right_bottom());
        if let Some(mut surface2) = surface.new_surface(&info) {
            let canvas2 = surface2.canvas();
            subset.draw(canvas2, page);

            if let Some(image2) = surface2.image_snapshot_with_bounds(rr.to_irect()) {
                canvas.draw_image(image2, r.pos.clone(), None);
                scroll_bar.draw(canvas);
            }
        }
    }
}
