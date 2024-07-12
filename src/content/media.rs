use crate::markup::{Attribute, TypeEntity, AUDIO, IMG, VIDEO};
use crate::parts::{HorizontalAlign, Ordinal, RectangleRange, Shape, Subset, VerticalAlign};
use skia_safe::{Canvas, Color, Image, Paint};

///"Audio" represents audio stream.
#[derive(Debug)]
pub struct Audio {
    subset: Subset,
    text: String,
    class: String,
    hidden: bool,
    id: String,
    ordinal: Ordinal,
    tip: String,
    range: RectangleRange,
    background: Color,
}

impl Audio {
    pub fn new() -> Self {
        Audio {
            subset: Subset::new(),
            text: String::new(),
            class: String::new(),
            hidden: false,
            id: String::new(),
            ordinal: Ordinal::None,
            tip: String::new(),
            range: RectangleRange::new(),
            background: Color::WHITE,
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::CLASS(a) => self.set_class(a),
            Attribute::HEIGHT(a) => self.set_height(a),
            Attribute::HIDDEN(a) => self.set_hidden(a),
            Attribute::ID(a) => self.set_id(a),
            Attribute::ORDINAL(a) => self.set_ordinal(a),
            Attribute::TIP(a) => self.set_tip(a),
            Attribute::WIDTH(a) => self.set_width(a),
            _ => {}
        }
    }

    element!(AUDIO);

    subset!();

    text!();

    class_id!();

    hidden!();

    ordinal!();

    tip!();

    range_background!();

    pub fn draw(&mut self, canvas: &Canvas) {}
}

///"Img" represents an image.
#[derive(Debug)]
pub struct Img {
    subset: Subset,
    text: String,
    class: String,
    hidden: bool,
    id: String,
    ordinal: Ordinal,
    src: String,
    tip: String,
    range: RectangleRange,
    background: Color,
    horizontal_align: HorizontalAlign,
    vertical_align: VerticalAlign,
    shape: Shape,
    shape_background: Color,
    border_width: isize,
    border_color: Color,
}

impl Img {
    pub fn new() -> Self {
        Img {
            subset: Subset::new(),
            text: String::new(),
            class: String::new(),
            hidden: false,
            id: String::new(),
            ordinal: Ordinal::None,
            src: String::new(),
            tip: String::new(),
            range: RectangleRange::new(),
            background: Color::GRAY,
            horizontal_align: HorizontalAlign::Left,
            vertical_align: VerticalAlign::Middle,
            shape: Shape::Default,
            shape_background: Color::WHITE,
            border_width: 0,
            border_color: Color::BLACK,
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::CLASS(a) => self.set_class(a),
            Attribute::HEIGHT(a) => self.set_height(a),
            Attribute::HIDDEN(a) => self.set_hidden(a),
            Attribute::ID(a) => self.set_id(a),
            Attribute::ORDINAL(a) => self.set_ordinal(a),
            Attribute::SRC(a) => self.set_src(a),
            Attribute::TIP(a) => self.set_tip(a),
            Attribute::WIDTH(a) => self.set_width(a),
            _ => {}
        }
    }

    element!(IMG);

    subset!();

    text!();

    class_id!();

    hidden!();

    ordinal!();

    src!();

    tip!();

    range_background!();

    align!();

    shape_background_border!();

    pub fn draw(&mut self, canvas: &Canvas) {}
}

///"Video" represents video.
#[derive(Debug)]
pub struct Video {
    subset: Subset,
    text: String,
    class: String,
    hidden: bool,
    id: String,
    ordinal: Ordinal,
    tip: String,
    range: RectangleRange,
    background: Color,
}

impl Video {
    pub fn new() -> Self {
        Video {
            subset: Subset::new(),
            text: String::new(),
            class: String::new(),
            hidden: false,
            id: String::new(),
            ordinal: Ordinal::None,
            tip: String::new(),
            range: RectangleRange::new(),
            background: Color::WHITE,
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::CLASS(a) => self.set_class(a),
            Attribute::HEIGHT(a) => self.set_height(a),
            Attribute::HIDDEN(a) => self.set_hidden(a),
            Attribute::ID(a) => self.set_id(a),
            Attribute::ORDINAL(a) => self.set_ordinal(a),
            Attribute::TIP(a) => self.set_tip(a),
            Attribute::WIDTH(a) => self.set_width(a),
            _ => {}
        }
    }

    element!(VIDEO);

    subset!();

    text!();

    class_id!();

    hidden!();

    ordinal!();

    tip!();

    range_background!();

    pub fn draw(&mut self, canvas: &Canvas) {}
}
