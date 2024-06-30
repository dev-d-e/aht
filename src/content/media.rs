use crate::markup::{Attribute, TypeEntity, AUDIO, IMG, VIDEO};
use crate::parts::{HorizontalAlign, Shape, VerticalAlign};
use skia_safe::{Canvas, Color, Font, IRect, Image, Paint, Rect};
use std::collections::VecDeque;

///"Audio" represents audio stream.
#[derive(Debug)]
pub struct Audio {
    subset: VecDeque<TypeEntity>,
    text: String,
    id: String,
    class: String,
    tip: String,
    hidden: bool,
    range: IRect,
    background: Color,
}

impl Audio {
    pub fn new() -> Self {
        Audio {
            subset: VecDeque::new(),
            text: String::new(),
            id: String::new(),
            class: String::new(),
            tip: String::new(),
            hidden: false,
            range: IRect::new_empty(),
            background: Color::WHITE,
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::ID(a) => self.set_id(a),
            Attribute::CLASS(a) => self.set_class(a),
            Attribute::TIP(a) => self.set_tip(a),
            _ => {}
        }
    }

    element!(AUDIO);

    subset!();

    text!();

    id_class!();

    tip!();

    hidden!();

    range_background!();

    pub fn draw(&mut self, canvas: &Canvas) {}
}

///"Img" represents an image.
#[derive(Debug)]
pub struct Img {
    subset: VecDeque<TypeEntity>,
    text: String,
    id: String,
    class: String,
    tip: String,
    hidden: bool,
    src: Option<Image>,
    range: IRect,
    background: Color,
    horizontal_align: HorizontalAlign,
    vertical_align: VerticalAlign,
    shape: Shape,
    shape_background: Color,
    border_width: i32,
    border_color: Color,
}

impl Img {
    pub fn new() -> Self {
        Img {
            subset: VecDeque::new(),
            text: String::new(),
            id: String::new(),
            class: String::new(),
            tip: String::new(),
            hidden: false,
            src: None,
            range: IRect::new_empty(),
            background: Color::GRAY,
            horizontal_align: HorizontalAlign::Left,
            vertical_align: VerticalAlign::Middle,
            shape: Shape::Circle(0, 0, 0),
            shape_background: Color::WHITE,
            border_width: 0,
            border_color: Color::BLACK,
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::ID(a) => self.set_id(a),
            Attribute::CLASS(a) => self.set_class(a),
            Attribute::TIP(a) => self.set_tip(a),
            _ => {}
        }
    }

    element!(IMG);

    subset!();

    text!();

    id_class!();

    tip!();

    hidden!();

    range_background!();

    align!();

    shape_background_border!();

    pub fn draw(&mut self, canvas: &Canvas, font: &Font) {}
}

///"Video" represents video.
#[derive(Debug)]
pub struct Video {
    subset: VecDeque<TypeEntity>,
    text: String,
    id: String,
    class: String,
    tip: String,
    hidden: bool,
    range: IRect,
    background: Color,
}

impl Video {
    pub fn new() -> Self {
        Video {
            subset: VecDeque::new(),
            text: String::new(),
            id: String::new(),
            class: String::new(),
            tip: String::new(),
            hidden: false,
            range: IRect::new_empty(),
            background: Color::WHITE,
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::ID(a) => self.set_id(a),
            Attribute::CLASS(a) => self.set_class(a),
            Attribute::TIP(a) => self.set_tip(a),
            _ => {}
        }
    }

    element!(VIDEO);

    subset!();

    text!();

    id_class!();

    tip!();

    hidden!();

    range_background!();

    pub fn draw(&mut self, canvas: &Canvas) {}
}
