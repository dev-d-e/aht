use crate::markup::{Attribute, TypeEntity, BUTTON, FORM, INP, OPTION, PT, SELECT, TIME};
use crate::parts::{HorizontalAlign, Shape, VerticalAlign};
use skia_safe::utils::text_utils::Align;
use skia_safe::{Canvas, Color, Font, IRect, Paint, Rect};
use std::collections::VecDeque;

///"Button" represents a button.
#[derive(Debug)]
pub struct Button {
    subset: VecDeque<TypeEntity>,
    text: String,
    id: String,
    class: String,
    tip: String,
    hidden: bool,
    name: String,
    value: String,
    disabled: bool,
    asynchronous: bool,
    href: String,
    range: IRect,
    background: Color,
    horizontal_align: HorizontalAlign,
    vertical_align: VerticalAlign,
    font_color: Color,
    shape: Shape,
    shape_background: Color,
    border_width: i32,
    border_color: Color,
}

impl Button {
    pub fn new() -> Self {
        Button {
            subset: VecDeque::new(),
            text: String::new(),
            id: String::new(),
            class: String::new(),
            tip: String::new(),
            hidden: false,
            name: String::new(),
            value: String::new(),
            disabled: false,
            asynchronous: false,
            href: String::new(),
            range: IRect::new_empty(),
            background: Color::GRAY,
            horizontal_align: HorizontalAlign::Left,
            vertical_align: VerticalAlign::Middle,
            font_color: Color::BLACK,
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

    element!(BUTTON);

    subset!();

    text!();

    id_class!();

    tip!();

    hidden!();

    name!();

    value!();

    disabled!();

    href!();

    range_background!();

    align!();

    font_color!();

    shape_background_border!();

    pub fn draw(&mut self, canvas: &Canvas, font: &Font) {}
}

///"Form" represents form.
#[derive(Debug)]
pub struct Form {
    subset: VecDeque<TypeEntity>,
    text: String,
    id: String,
    class: String,
    action: String,
    method: String,
    name: String,
    enctype: String,
    asynchronous: bool,
}

impl Form {
    pub fn new() -> Self {
        Form {
            subset: VecDeque::new(),
            text: String::new(),
            id: String::new(),
            class: String::new(),
            action: String::new(),
            method: String::new(),
            name: String::new(),
            enctype: String::new(),
            asynchronous: false,
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::ID(a) => self.set_id(a),
            Attribute::CLASS(a) => self.set_class(a),
            _ => {}
        }
    }

    element!(FORM);

    subset!();

    text!();

    id_class!();

    name!();

    pub fn draw(&mut self, canvas: &Canvas) {}
}

///"Inp" represents input.
#[derive(Debug)]
pub struct Inp {
    subset: VecDeque<TypeEntity>,
    text: String,
    id: String,
    class: String,
    name: String,
    value: String,
    tip: String,
    hidden: bool,
    readonly: bool,
    disabled: bool,
    required: bool,
    multiple: bool,
    range: IRect,
    background: Color,
    horizontal_align: HorizontalAlign,
    vertical_align: VerticalAlign,
    font_color: Color,
    shape: Shape,
    shape_background: Color,
    border_width: i32,
    border_color: Color,
}

impl Inp {
    pub fn new() -> Self {
        Inp {
            subset: VecDeque::new(),
            text: String::new(),
            id: String::new(),
            class: String::new(),
            name: String::new(),
            value: String::new(),
            tip: String::new(),
            hidden: false,
            readonly: false,
            disabled: false,
            required: false,
            multiple: false,
            range: IRect::new_empty(),
            background: Color::WHITE,
            horizontal_align: HorizontalAlign::Left,
            vertical_align: VerticalAlign::Middle,
            font_color: Color::BLACK,
            shape: Shape::Rectangle(IRect::new_empty()),
            shape_background: Color::WHITE,
            border_width: 0,
            border_color: Color::WHITE,
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

    element!(INP);

    subset!();

    text!();

    id_class!();

    tip!();

    hidden!();

    name!();

    value!();

    readonly!();

    disabled!();

    required!();

    multiple!();

    range_background!();

    align!();

    font_color!();

    shape_background_border!();

    pub fn draw(&mut self, canvas: &Canvas, font: &Font) {}
}

///"Opt" represents an option.
#[derive(Debug)]
pub struct Opt {
    text: String,
    value: String,
    disabled: bool,
    selected: bool,
}

impl Opt {
    pub fn new() -> Self {
        Opt {
            text: String::new(),
            value: String::new(),
            disabled: false,
            selected: false,
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            _ => {}
        }
    }

    element!(OPTION);

    text!();

    value!();

    disabled!();

    selected!();

    pub fn draw(&mut self, canvas: &Canvas) {}
}

///"Pt" represents plain text.
#[derive(Debug)]
pub struct Pt {
    subset: VecDeque<TypeEntity>,
    text: String,
    id: String,
    class: String,
    tip: String,
    hidden: bool,
    range: IRect,
    background: Color,
    horizontal_align: HorizontalAlign,
    vertical_align: VerticalAlign,
    font_color: Color,
    shape: Shape,
    shape_background: Color,
    border_width: i32,
    border_color: Color,
}

impl Pt {
    pub fn new() -> Self {
        Pt {
            subset: VecDeque::new(),
            text: String::new(),
            id: String::new(),
            class: String::new(),
            tip: String::new(),
            hidden: false,
            range: IRect::new_empty(),
            background: Color::GRAY,
            horizontal_align: HorizontalAlign::Left,
            vertical_align: VerticalAlign::Middle,
            font_color: Color::BLACK,
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

    element!(PT);

    subset!();

    text!();

    id_class!();

    tip!();

    hidden!();

    range_background!();

    align!();

    font_color!();

    shape_background_border!();

    pub fn draw(&mut self, canvas: &Canvas, font: &Font) {}
}

///"Select" represents a select.
#[derive(Debug)]
pub struct Select {
    subset: VecDeque<TypeEntity>,
    text: String,
    id: String,
    class: String,
    tip: String,
    hidden: bool,
    name: String,
    disabled: bool,
    required: bool,
    multiple: bool,
    range: IRect,
    background: Color,
    horizontal_align: HorizontalAlign,
    vertical_align: VerticalAlign,
    font_color: Color,
    shape: Shape,
    shape_background: Color,
    border_width: i32,
    border_color: Color,
}

impl Select {
    pub fn new() -> Self {
        Select {
            subset: VecDeque::new(),
            text: String::new(),
            id: String::new(),
            class: String::new(),
            tip: String::new(),
            hidden: false,
            name: String::new(),
            disabled: false,
            required: false,
            multiple: false,
            range: IRect::new_empty(),
            background: Color::WHITE,
            horizontal_align: HorizontalAlign::Left,
            vertical_align: VerticalAlign::Middle,
            font_color: Color::BLACK,
            shape: Shape::Rectangle(IRect::new_empty()),
            shape_background: Color::WHITE,
            border_width: 0,
            border_color: Color::WHITE,
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

    element!(SELECT);

    subset!();

    text!();

    id_class!();

    tip!();

    hidden!();

    name!();

    disabled!();

    required!();

    multiple!();

    range_background!();

    align!();

    font_color!();

    shape_background_border!();

    pub fn draw(&mut self, canvas: &Canvas) {}
}

///"Time" represents date time.
#[derive(Debug)]
pub struct Time {
    subset: VecDeque<TypeEntity>,
    text: String,
    id: String,
    class: String,
    name: String,
    value: String,
    tip: String,
    hidden: bool,
    readonly: bool,
    disabled: bool,
    required: bool,
    range: IRect,
    background: Color,
    horizontal_align: HorizontalAlign,
    vertical_align: VerticalAlign,
    font_color: Color,
    shape: Shape,
    shape_background: Color,
    border_width: i32,
    border_color: Color,
}

impl Time {
    pub fn new() -> Self {
        Time {
            subset: VecDeque::new(),
            text: String::new(),
            id: String::new(),
            class: String::new(),
            name: String::new(),
            value: String::new(),
            tip: String::new(),
            hidden: false,
            readonly: false,
            disabled: false,
            required: false,
            range: IRect::new_empty(),
            background: Color::WHITE,
            horizontal_align: HorizontalAlign::Left,
            vertical_align: VerticalAlign::Middle,
            font_color: Color::BLACK,
            shape: Shape::Rectangle(IRect::new_empty()),
            shape_background: Color::WHITE,
            border_width: 0,
            border_color: Color::WHITE,
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

    element!(TIME);

    subset!();

    text!();

    id_class!();

    tip!();

    hidden!();

    name!();

    value!();

    readonly!();

    disabled!();

    required!();

    range_background!();

    align!();

    font_color!();

    shape_background_border!();

    pub fn draw(&mut self, canvas: &Canvas) {}
}
