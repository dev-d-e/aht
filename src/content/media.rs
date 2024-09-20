use super::*;
use crate::markup::{Attribute, Page, TypeEntity, AUDIO, IMG, VIDEO};
use crate::parts::{AlignPattern, Coord, Ordinal, Painter, Range, ScrollBar, Sides, Subset};
use skia_safe::Canvas;

///"Audio" represents audio stream.
#[derive(Debug)]
pub struct Audio {
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
    parent: *mut TypeEntity,
}

impl Audio {
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
            side: Sides::pixel(100, 50),
            background: Box::new(Range::new()),
            align_pattern: AlignPattern::center_middle(),
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

    element!(AUDIO);

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

///"Img" represents an image.
#[derive(Debug)]
pub struct Img {
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
    pub outside: Box<dyn OutPainter>,
    scroll_bar: ScrollBar,
    parent: *mut TypeEntity,
}

impl Img {
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
            align_pattern: AlignPattern::center_middle(),
            outside: Box::new(Border::new()),
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
            Attribute::SRC(a) => self.src = a,
            Attribute::TIP(a) => self.tip = a,
            Attribute::WIDTH(a) => self.side.width = a,
            _ => {}
        }
    }

    element!(IMG);

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
        self.outside.as_mut().act(&r, canvas);
        self.background.as_mut().act(&r, canvas);
        self.subset.draw(canvas, page);
    }
}

///"Video" represents video.
#[derive(Debug)]
pub struct Video {
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
    parent: *mut TypeEntity,
}

impl Video {
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
            side: Sides::pixel(100, 50),
            background: Box::new(Range::new()),
            align_pattern: AlignPattern::center_middle(),
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

    element!(VIDEO);

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
