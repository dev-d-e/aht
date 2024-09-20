use super::*;
use crate::markup::{Attribute, Page, TypeEntity, BUTTON, FORM, INP, OPTION, PT, SELECT, TIME};
use crate::parts::{
    AlignPattern, ApplyFont, Coord, Ordinal, Painter, Range, ScrollBar, Sides, Subset,
};
use skia_safe::Canvas;

///"Button" represents a button.
#[derive(Debug)]
pub struct Button {
    pub subset: Subset,
    pub text: String,
    pub asynchronous: bool,
    pub class: String,
    pub disabled: bool,
    pub hidden: bool,
    pub href: String,
    pub id: String,
    pub name: String,
    pub ordinal: Ordinal,
    pub tip: String,
    pub value: String,
    pub zero: Coord,
    pub side: Sides,
    pub background: Box<dyn Painter>,
    pub align_pattern: AlignPattern,
    pub apply_font: ApplyFont,
    parent: *mut TypeEntity,
}

impl Button {
    pub(crate) fn new() -> Self {
        Self {
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
            zero: Coord::new(),
            side: Sides::pixel(300, 50),
            background: Box::new(range!(SURFACE_COLOR, 10, 10)),
            align_pattern: AlignPattern::center_middle(),
            apply_font: ApplyFont::new(),
            parent: std::ptr::null_mut(),
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::ASYNCHRONOUS(a) => self.asynchronous = a,
            Attribute::CLASS(a) => self.class = a,
            Attribute::DISABLED(a) => self.disabled = a,
            Attribute::HEIGHT(a) => self.side.height = a,
            Attribute::HIDDEN(a) => self.hidden = a,
            Attribute::HREF(a) => self.href = a,
            Attribute::ID(a) => self.id = a,
            Attribute::NAME(a) => self.name = a,
            Attribute::ORDINAL(a) => self.ordinal = a,
            Attribute::TIP(a) => self.tip = a,
            Attribute::VALUE(a) => self.value = a,
            Attribute::WIDTH(a) => self.side.width = a,
            _ => {}
        }
    }

    element!(BUTTON);

    zero!();

    set_parent!();

    pub fn draw(&mut self, canvas: &Canvas, page: &mut Page) {
        if self.disabled || self.hidden {
            return;
        }

        let r = self.side.to_rect(&self.zero);
        if r.is_empty() {
            return;
        }
        self.background.as_mut().act(&r, canvas);
        self.apply_font
            .draw(&r, &self.align_pattern, &self.text, canvas);
        self.subset.draw(canvas, page);
    }
}

///"Form" represents form.
#[derive(Debug)]
pub struct Form {
    pub subset: Subset,
    pub text: String,
    pub action: String,
    pub asynchronous: bool,
    pub class: String,
    pub enctype: String,
    pub id: String,
    pub method: String,
    pub name: String,
}

impl Form {
    pub(crate) fn new() -> Self {
        Self {
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
            Attribute::ACTION(a) => self.action = a,
            Attribute::ASYNCHRONOUS(a) => self.asynchronous = a,
            Attribute::CLASS(a) => self.class = a,
            Attribute::ENCTYPE(a) => self.enctype = a,
            Attribute::ID(a) => self.id = a,
            Attribute::METHOD(a) => self.method = a,
            Attribute::NAME(a) => self.name = a,
            _ => {}
        }
    }

    element!(FORM);
}

///"Inp" represents input.
#[derive(Debug)]
pub struct Inp {
    pub subset: Subset,
    pub text: String,
    pub class: String,
    pub disabled: bool,
    pub hidden: bool,
    pub id: String,
    pub multiple: bool,
    pub name: String,
    pub ordinal: Ordinal,
    pub readonly: bool,
    pub required: bool,
    pub tip: String,
    pub value: String,
    pub zero: Coord,
    pub side: Sides,
    pub background: Box<dyn Painter>,
    pub align_pattern: AlignPattern,
    pub apply_font: ApplyFont,
    pub outside: Box<dyn OutPainter>,
    scroll_bar: ScrollBar,
    parent: *mut TypeEntity,
}

impl Inp {
    pub(crate) fn new() -> Self {
        Self {
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
            zero: Coord::new(),
            side: Sides::pixel(100, 50),
            background: Box::new(Range::new()),
            align_pattern: AlignPattern::left_middle(),
            apply_font: ApplyFont::new(),
            outside: Box::new(Border::new()),
            scroll_bar: ScrollBar::new(),
            parent: std::ptr::null_mut(),
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::CLASS(a) => self.class = a,
            Attribute::DISABLED(a) => self.disabled = a,
            Attribute::HEIGHT(a) => self.side.height = a,
            Attribute::HIDDEN(a) => self.hidden = a,
            Attribute::ID(a) => self.id = a,
            Attribute::MULTIPLE(a) => self.multiple = a,
            Attribute::NAME(a) => self.name = a,
            Attribute::ORDINAL(a) => self.ordinal = a,
            Attribute::READONLY(a) => self.readonly = a,
            Attribute::REQUIRED(a) => self.required = a,
            Attribute::TIP(a) => self.tip = a,
            Attribute::VALUE(a) => self.value = a,
            Attribute::WIDTH(a) => self.side.width = a,
            _ => {}
        }
    }

    element!(INP);

    zero!();

    set_parent!();

    pub fn draw(&mut self, canvas: &Canvas, page: &mut Page) {
        if self.disabled || self.hidden {
            return;
        }

        let r = self.side.to_rect(&self.zero);
        if r.is_empty() {
            return;
        }
        self.outside.as_mut().act(&r, canvas);
        self.background.as_mut().act(&r, canvas);
        self.apply_font
            .draw(&r, &self.align_pattern, &self.text, canvas);
    }
}

///"Opt" represents an option.
#[derive(Debug)]
pub struct Opt {
    pub text: String,
    pub disabled: bool,
    pub selected: bool,
    pub value: String,
    pub zero: Coord,
    pub side: Sides,
    pub background: Box<dyn Painter>,
    pub align_pattern: AlignPattern,
    pub apply_font: ApplyFont,
    pub outside: Box<dyn OutPainter>,
    parent: *mut TypeEntity,
}

impl Opt {
    pub(crate) fn new() -> Self {
        Self {
            text: String::new(),
            disabled: false,
            selected: false,
            value: String::new(),
            zero: Coord::new(),
            side: Sides::pixel(100, 50),
            background: Box::new(Range::new()),
            align_pattern: AlignPattern::left_middle(),
            apply_font: ApplyFont::new(),
            outside: Box::new(Border::new()),
            parent: std::ptr::null_mut(),
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::DISABLED(a) => self.disabled = a,
            Attribute::SELECTED(a) => self.selected = a,
            Attribute::VALUE(a) => self.value = a,
            _ => {}
        }
    }

    element!(OPTION);

    pub(crate) fn set_parent(&mut self, parent_ptr: &mut TypeEntity) {
        self.parent = parent_ptr;
    }

    pub(crate) fn resize(&mut self) {}

    pub fn draw(&mut self, canvas: &Canvas, page: &mut Page) {
        if self.disabled {
            return;
        }

        let r = self.side.to_rect(&self.zero);
        if r.is_empty() {
            return;
        }
        self.outside.as_mut().act(&r, canvas);
        self.background.as_mut().act(&r, canvas);
        self.apply_font
            .draw(&r, &self.align_pattern, &self.text, canvas);
    }
}

///"Pt" represents plain text.
#[derive(Debug)]
pub struct Pt {
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
    pub apply_font: ApplyFont,
    pub outside: Box<dyn OutPainter>,
    parent: *mut TypeEntity,
}

impl Pt {
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
            align_pattern: AlignPattern::left_middle(),
            apply_font: ApplyFont::new(),
            outside: Box::new(Border::new()),
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

    element!(PT);

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
        self.apply_font
            .draw(&r, &self.align_pattern, &self.text, canvas);
    }
}

///"Select" represents a select.
#[derive(Debug)]
pub struct Select {
    pub subset: Subset,
    pub text: String,
    pub class: String,
    pub disabled: bool,
    pub hidden: bool,
    pub id: String,
    pub multiple: bool,
    pub name: String,
    pub ordinal: Ordinal,
    pub required: bool,
    pub tip: String,
    pub zero: Coord,
    pub side: Sides,
    pub background: Box<dyn Painter>,
    pub align_pattern: AlignPattern,
    pub apply_font: ApplyFont,
    pub outside: Box<dyn OutPainter>,
    scroll_bar: ScrollBar,
    parent: *mut TypeEntity,
}

impl Select {
    pub(crate) fn new() -> Self {
        Self {
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
            zero: Coord::new(),
            side: Sides::pixel(100, 50),
            background: Box::new(Range::new()),
            align_pattern: AlignPattern::left_middle(),
            apply_font: ApplyFont::new(),
            outside: Box::new(Border::new()),
            scroll_bar: ScrollBar::new(),
            parent: std::ptr::null_mut(),
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::CLASS(a) => self.class = a,
            Attribute::DISABLED(a) => self.disabled = a,
            Attribute::HEIGHT(a) => self.side.height = a,
            Attribute::HIDDEN(a) => self.hidden = a,
            Attribute::ID(a) => self.id = a,
            Attribute::MULTIPLE(a) => self.multiple = a,
            Attribute::NAME(a) => self.name = a,
            Attribute::ORDINAL(a) => self.ordinal = a,
            Attribute::REQUIRED(a) => self.required = a,
            Attribute::TIP(a) => self.tip = a,
            Attribute::WIDTH(a) => self.side.width = a,
            _ => {}
        }
    }

    element!(SELECT);

    zero!();

    set_parent!();

    pub fn draw(&mut self, canvas: &Canvas, page: &mut Page) {
        if self.disabled || self.hidden {
            return;
        }

        let r = self.side.to_rect(&self.zero);
        if r.is_empty() {
            return;
        }
        self.outside.as_mut().act(&r, canvas);
        self.background.as_mut().act(&r, canvas);
        self.apply_font
            .draw(&r, &self.align_pattern, &self.text, canvas);
    }
}

///"Time" represents date time.
#[derive(Debug)]
pub struct Time {
    pub subset: Subset,
    pub text: String,
    pub class: String,
    pub disabled: bool,
    pub hidden: bool,
    pub id: String,
    pub name: String,
    pub ordinal: Ordinal,
    pub readonly: bool,
    pub required: bool,
    pub tip: String,
    pub value: String,
    pub zero: Coord,
    pub side: Sides,
    pub background: Box<dyn Painter>,
    pub align_pattern: AlignPattern,
    pub apply_font: ApplyFont,
    pub outside: Box<dyn OutPainter>,
    parent: *mut TypeEntity,
}

impl Time {
    pub(crate) fn new() -> Self {
        Self {
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
            zero: Coord::new(),
            side: Sides::pixel(100, 50),
            background: Box::new(Range::new()),
            align_pattern: AlignPattern::left_middle(),
            apply_font: ApplyFont::new(),
            outside: Box::new(Border::new()),
            parent: std::ptr::null_mut(),
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::CLASS(a) => self.class = a,
            Attribute::DISABLED(a) => self.disabled = a,
            Attribute::HEIGHT(a) => self.side.height = a,
            Attribute::HIDDEN(a) => self.hidden = a,
            Attribute::ID(a) => self.id = a,
            Attribute::NAME(a) => self.name = a,
            Attribute::ORDINAL(a) => self.ordinal = a,
            Attribute::READONLY(a) => self.readonly = a,
            Attribute::REQUIRED(a) => self.required = a,
            Attribute::TIP(a) => self.tip = a,
            Attribute::VALUE(a) => self.value = a,
            Attribute::WIDTH(a) => self.side.width = a,
            _ => {}
        }
    }

    element!(TIME);

    zero!();

    set_parent!();

    pub fn draw(&mut self, canvas: &Canvas, page: &mut Page) {
        if self.disabled || self.hidden {
            return;
        }

        let r = self.side.to_rect(&self.zero);
        if r.is_empty() {
            return;
        }
        self.outside.as_mut().act(&r, canvas);
        self.background.as_mut().act(&r, canvas);
        self.apply_font
            .draw(&r, &self.align_pattern, &self.text, canvas);
    }
}
