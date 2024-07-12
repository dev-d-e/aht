use crate::markup::{Attribute, TypeEntity, BUTTON, FORM, INP, OPTION, PT, SELECT, TIME};
use crate::parts::{
    ApplyFont, HorizontalAlign, Ordinal, RectangleRange, Shape, Subset, VerticalAlign,
};
use skia_safe::utils::text_utils::Align;
use skia_safe::{Canvas, Color, Paint};

///"Button" represents a button.
#[derive(Debug)]
pub struct Button {
    subset: Subset,
    text: String,
    asynchronous: bool,
    class: String,
    disabled: bool,
    hidden: bool,
    href: String,
    id: String,
    name: String,
    ordinal: Ordinal,
    tip: String,
    value: String,
    range: RectangleRange,
    background: Color,
    horizontal_align: HorizontalAlign,
    vertical_align: VerticalAlign,
    apply_font: ApplyFont,
    shape: Shape,
    shape_background: Color,
    border_width: isize,
    border_color: Color,
}

impl Button {
    pub fn new() -> Self {
        Button {
            subset: Subset::new(),
            text: String::new(),
            asynchronous: false,
            class: String::new(),
            disabled: false,
            hidden: false,
            href: String::new(),
            id: String::new(),
            name: String::new(),
            ordinal: Ordinal::None,
            tip: String::new(),
            value: String::new(),
            range: RectangleRange::new(),
            background: Color::GRAY,
            horizontal_align: HorizontalAlign::Left,
            vertical_align: VerticalAlign::Middle,
            apply_font: ApplyFont::new(),
            shape: Shape::Default,
            shape_background: Color::WHITE,
            border_width: 0,
            border_color: Color::BLACK,
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::ASYNCHRONOUS(a) => self.set_asynchronous(a),
            Attribute::CLASS(a) => self.set_class(a),
            Attribute::DISABLED(a) => self.set_disabled(a),
            Attribute::HEIGHT(a) => self.set_height(a),
            Attribute::HIDDEN(a) => self.set_hidden(a),
            Attribute::HREF(a) => self.set_href(a),
            Attribute::ID(a) => self.set_id(a),
            Attribute::NAME(a) => self.set_name(a),
            Attribute::ORDINAL(a) => self.set_ordinal(a),
            Attribute::TIP(a) => self.set_tip(a),
            Attribute::VALUE(a) => self.set_value(a),
            Attribute::WIDTH(a) => self.set_width(a),
            _ => {}
        }
    }

    element!(BUTTON);

    subset!();

    text!();

    asynchronous!();

    class_id!();

    disabled!();

    hidden!();

    href!();

    name!();

    ordinal!();

    tip!();

    value!();

    range_background!();

    align!();

    apply_font!();

    shape_background_border!();

    pub fn draw(&mut self, canvas: &Canvas) {}
}

///"Form" represents form.
#[derive(Debug)]
pub struct Form {
    subset: Subset,
    text: String,
    action: String,
    asynchronous: bool,
    class: String,
    enctype: String,
    id: String,
    method: String,
    name: String,
}

impl Form {
    pub fn new() -> Self {
        Form {
            subset: Subset::new(),
            text: String::new(),
            action: String::new(),
            asynchronous: false,
            class: String::new(),
            enctype: String::new(),
            id: String::new(),
            method: String::new(),
            name: String::new(),
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::ACTION(a) => self.set_action(a),
            Attribute::ASYNCHRONOUS(a) => self.set_asynchronous(a),
            Attribute::CLASS(a) => self.set_class(a),
            Attribute::ENCTYPE(a) => self.set_enctype(a),
            Attribute::ID(a) => self.set_id(a),
            Attribute::METHOD(a) => self.set_method(a),
            Attribute::NAME(a) => self.set_name(a),
            _ => {}
        }
    }

    element!(FORM);

    subset!();

    text!();

    action!();

    asynchronous!();

    class_id!();

    enctype!();

    method!();

    name!();

    pub fn draw(&mut self, canvas: &Canvas) {}
}

///"Inp" represents input.
#[derive(Debug)]
pub struct Inp {
    subset: Subset,
    text: String,
    class: String,
    disabled: bool,
    hidden: bool,
    id: String,
    multiple: bool,
    name: String,
    ordinal: Ordinal,
    readonly: bool,
    required: bool,
    tip: String,
    value: String,
    range: RectangleRange,
    background: Color,
    horizontal_align: HorizontalAlign,
    vertical_align: VerticalAlign,
    apply_font: ApplyFont,
    shape: Shape,
    shape_background: Color,
    border_width: isize,
    border_color: Color,
}

impl Inp {
    pub fn new() -> Self {
        Inp {
            subset: Subset::new(),
            text: String::new(),
            class: String::new(),
            disabled: false,
            hidden: false,
            id: String::new(),
            multiple: false,
            name: String::new(),
            ordinal: Ordinal::None,
            readonly: false,
            required: false,
            tip: String::new(),
            value: String::new(),
            range: RectangleRange::new(),
            background: Color::WHITE,
            horizontal_align: HorizontalAlign::Left,
            vertical_align: VerticalAlign::Middle,
            apply_font: ApplyFont::new(),
            shape: Shape::Default,
            shape_background: Color::WHITE,
            border_width: 0,
            border_color: Color::WHITE,
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::CLASS(a) => self.set_class(a),
            Attribute::DISABLED(a) => self.set_disabled(a),
            Attribute::HEIGHT(a) => self.set_height(a),
            Attribute::HIDDEN(a) => self.set_hidden(a),
            Attribute::ID(a) => self.set_id(a),
            Attribute::MULTIPLE(a) => self.set_multiple(a),
            Attribute::NAME(a) => self.set_name(a),
            Attribute::ORDINAL(a) => self.set_ordinal(a),
            Attribute::READONLY(a) => self.set_readonly(a),
            Attribute::REQUIRED(a) => self.set_required(a),
            Attribute::TIP(a) => self.set_tip(a),
            Attribute::VALUE(a) => self.set_value(a),
            Attribute::WIDTH(a) => self.set_width(a),
            _ => {}
        }
    }

    element!(INP);

    subset!();

    text!();

    class_id!();

    disabled!();

    hidden!();

    multiple!();

    name!();

    ordinal!();

    readonly!();

    required!();

    tip!();

    value!();

    range_background!();

    align!();

    apply_font!();

    shape_background_border!();

    pub fn draw(&mut self, canvas: &Canvas) {}
}

///"Opt" represents an option.
#[derive(Debug)]
pub struct Opt {
    text: String,
    disabled: bool,
    selected: bool,
    value: String,
}

impl Opt {
    pub fn new() -> Self {
        Opt {
            text: String::new(),
            disabled: false,
            selected: false,
            value: String::new(),
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::DISABLED(a) => self.set_disabled(a),
            Attribute::SELECTED(a) => self.set_selected(a),
            Attribute::VALUE(a) => self.set_value(a),
            _ => {}
        }
    }

    element!(OPTION);

    text!();

    disabled!();

    selected!();

    value!();

    pub fn draw(&mut self, canvas: &Canvas) {}
}

///"Pt" represents plain text.
#[derive(Debug)]
pub struct Pt {
    subset: Subset,
    text: String,
    class: String,
    hidden: bool,
    id: String,
    ordinal: Ordinal,
    tip: String,
    range: RectangleRange,
    background: Color,
    horizontal_align: HorizontalAlign,
    vertical_align: VerticalAlign,
    apply_font: ApplyFont,
    shape: Shape,
    shape_background: Color,
    border_width: isize,
    border_color: Color,
}

impl Pt {
    pub fn new() -> Self {
        Pt {
            subset: Subset::new(),
            text: String::new(),
            class: String::new(),
            hidden: false,
            id: String::new(),
            ordinal: Ordinal::None,
            tip: String::new(),
            range: RectangleRange::new(),
            background: Color::GRAY,
            horizontal_align: HorizontalAlign::Left,
            vertical_align: VerticalAlign::Middle,
            apply_font: ApplyFont::new(),
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
            Attribute::TIP(a) => self.set_tip(a),
            Attribute::WIDTH(a) => self.set_width(a),
            _ => {}
        }
    }

    element!(PT);

    subset!();

    text!();

    class_id!();

    hidden!();

    ordinal!();

    tip!();

    range_background!();

    align!();

    apply_font!();

    shape_background_border!();

    pub fn draw(&mut self, canvas: &Canvas) {}
}

///"Select" represents a select.
#[derive(Debug)]
pub struct Select {
    subset: Subset,
    text: String,
    class: String,
    disabled: bool,
    hidden: bool,
    id: String,
    multiple: bool,
    name: String,
    ordinal: Ordinal,
    required: bool,
    tip: String,
    range: RectangleRange,
    background: Color,
    horizontal_align: HorizontalAlign,
    vertical_align: VerticalAlign,
    apply_font: ApplyFont,
    shape: Shape,
    shape_background: Color,
    border_width: isize,
    border_color: Color,
}

impl Select {
    pub fn new() -> Self {
        Select {
            subset: Subset::new(),
            text: String::new(),
            class: String::new(),
            disabled: false,
            hidden: false,
            id: String::new(),
            multiple: false,
            name: String::new(),
            ordinal: Ordinal::None,
            required: false,
            tip: String::new(),
            range: RectangleRange::new(),
            background: Color::WHITE,
            horizontal_align: HorizontalAlign::Left,
            vertical_align: VerticalAlign::Middle,
            apply_font: ApplyFont::new(),
            shape: Shape::Default,
            shape_background: Color::WHITE,
            border_width: 0,
            border_color: Color::WHITE,
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::CLASS(a) => self.set_class(a),
            Attribute::DISABLED(a) => self.set_disabled(a),
            Attribute::HEIGHT(a) => self.set_height(a),
            Attribute::HIDDEN(a) => self.set_hidden(a),
            Attribute::ID(a) => self.set_id(a),
            Attribute::MULTIPLE(a) => self.set_multiple(a),
            Attribute::NAME(a) => self.set_name(a),
            Attribute::ORDINAL(a) => self.set_ordinal(a),
            Attribute::REQUIRED(a) => self.set_required(a),
            Attribute::TIP(a) => self.set_tip(a),
            Attribute::WIDTH(a) => self.set_width(a),
            _ => {}
        }
    }

    element!(SELECT);

    subset!();

    text!();

    class_id!();

    disabled!();

    hidden!();

    multiple!();

    name!();

    ordinal!();

    required!();

    tip!();

    range_background!();

    align!();

    apply_font!();

    shape_background_border!();

    pub fn draw(&mut self, canvas: &Canvas) {}
}

///"Time" represents date time.
#[derive(Debug)]
pub struct Time {
    subset: Subset,
    text: String,
    class: String,
    disabled: bool,
    hidden: bool,
    id: String,
    name: String,
    ordinal: Ordinal,
    readonly: bool,
    required: bool,
    tip: String,
    value: String,
    range: RectangleRange,
    background: Color,
    horizontal_align: HorizontalAlign,
    vertical_align: VerticalAlign,
    apply_font: ApplyFont,
    shape: Shape,
    shape_background: Color,
    border_width: isize,
    border_color: Color,
}

impl Time {
    pub fn new() -> Self {
        Time {
            subset: Subset::new(),
            text: String::new(),
            class: String::new(),
            disabled: false,
            hidden: false,
            id: String::new(),
            name: String::new(),
            ordinal: Ordinal::None,
            readonly: false,
            required: false,
            tip: String::new(),
            value: String::new(),
            range: RectangleRange::new(),
            background: Color::WHITE,
            horizontal_align: HorizontalAlign::Left,
            vertical_align: VerticalAlign::Middle,
            apply_font: ApplyFont::new(),
            shape: Shape::Default,
            shape_background: Color::WHITE,
            border_width: 0,
            border_color: Color::WHITE,
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::CLASS(a) => self.set_class(a),
            Attribute::DISABLED(a) => self.set_disabled(a),
            Attribute::HEIGHT(a) => self.set_height(a),
            Attribute::HIDDEN(a) => self.set_hidden(a),
            Attribute::ID(a) => self.set_id(a),
            Attribute::NAME(a) => self.set_name(a),
            Attribute::ORDINAL(a) => self.set_ordinal(a),
            Attribute::READONLY(a) => self.set_readonly(a),
            Attribute::REQUIRED(a) => self.set_required(a),
            Attribute::TIP(a) => self.set_tip(a),
            Attribute::VALUE(a) => self.set_value(a),
            Attribute::WIDTH(a) => self.set_width(a),
            _ => {}
        }
    }

    element!(TIME);

    subset!();

    text!();

    class_id!();

    disabled!();

    hidden!();

    name!();

    ordinal!();

    readonly!();

    required!();

    tip!();

    value!();

    range_background!();

    align!();

    apply_font!();

    shape_background_border!();

    pub fn draw(&mut self, canvas: &Canvas) {}
}
