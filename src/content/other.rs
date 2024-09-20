use crate::markup::{Attribute, Page, TypeEntity, CANVAS, IFRAME};
use crate::parts::{AlignPattern, Coord, Ordinal, Painter, Range, ScrollBar, Sides, Subset};
use skia_safe::Canvas;

///"Canv" represents canvas.
#[derive(Debug)]
pub struct Canv {
    pub subset: Subset,
    pub text: String,
    pub class: String,
    pub hidden: bool,
    pub id: String,
    pub ordinal: Ordinal,
    pub tip: String,
    pub zero: Coord,
    pub side: Sides,
    pub background: Box<dyn Painter>,
    pub align_pattern: AlignPattern,
    scroll_bar: ScrollBar,
    parent: *mut TypeEntity,
}

impl Canv {
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
            side: Sides::pixel(200, 100),
            background: Box::new(Range::new()),
            align_pattern: AlignPattern::center_middle(),
            scroll_bar: ScrollBar::new(),
            parent: std::ptr::null_mut(),
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::CLASS(a) => self.class = a,
            Attribute::HEIGHT(a) => self.side.height = a,
            Attribute::HIDDEN(a) => self.hidden = a,
            Attribute::ID(a) => self.id = a,
            Attribute::ORDINAL(a) => self.ordinal = a,
            Attribute::TIP(a) => self.tip = a,
            Attribute::WIDTH(a) => self.side.width = a,
            _ => {}
        }
    }

    element!(CANVAS);

    zero!();

    set_parent!();

    pub fn draw(&mut self, canvas: &Canvas, page: &mut Page) {
        if self.hidden {
            return;
        }

        let r = self.side.to_rect(&self.zero);
        if r.is_empty() {
            return;
        }
        self.background.as_mut().act(&r, canvas);
        self.subset.draw(canvas, page);
    }
}

///"Iframe" represents iframe.
#[derive(Debug)]
pub struct Iframe {
    pub subset: Subset,
    pub text: String,
    pub class: String,
    pub hidden: bool,
    pub id: String,
    pub ordinal: Ordinal,
    pub src: String,
    pub tip: String,
    pub zero: Coord,
    pub side: Sides,
    pub background: Box<dyn Painter>,
    pub align_pattern: AlignPattern,
    parent: *mut TypeEntity,
}

impl Iframe {
    pub(crate) fn new() -> Self {
        Self {
            subset: Subset::new(),
            text: String::new(),
            class: String::new(),
            hidden: false,
            id: String::new(),
            ordinal: Ordinal::None,
            src: String::new(),
            tip: String::new(),
            zero: Coord::new(),
            side: Sides::pixel(100, 50),
            background: Box::new(Range::new()),
            align_pattern: AlignPattern::left_middle(),
            parent: std::ptr::null_mut(),
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::CLASS(a) => self.class = a,
            Attribute::HEIGHT(a) => self.side.height = a,
            Attribute::HIDDEN(a) => self.hidden = a,
            Attribute::ID(a) => self.id = a,
            Attribute::SRC(a) => self.src = a,
            Attribute::TIP(a) => self.tip = a,
            Attribute::WIDTH(a) => self.side.width = a,
            _ => {}
        }
    }

    element!(IFRAME);

    zero!();

    set_parent!();

    pub fn draw(&mut self, canvas: &Canvas, page: &mut Page) {
        if self.hidden {
            return;
        }

        let r = self.side.to_rect(&self.zero);
        if r.is_empty() {
            return;
        }
        self.background.as_mut().act(&r, canvas);
        self.subset.draw(canvas, page);
    }
}
