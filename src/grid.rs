use crate::markup::{Attribute, TypeEntity, AREA, BODY, DIALOG};
use skia_safe::{Canvas, Color, IRect, Paint, Rect};
use std::collections::VecDeque;
use std::fmt::Debug;

///"Body" is grid layout.
#[derive(Debug)]
pub struct Body {
    subset: VecDeque<TypeEntity>,
    text: String,
    id: String,
    class: String,
    tip: String,
    range: IRect,
    column: i32,
    row: i32,
    xpoints: Vec<i32>,
    ypoints: Vec<i32>,
    background: Color,
}

impl Body {
    pub fn new() -> Self {
        Body {
            subset: VecDeque::new(),
            text: String::new(),
            id: String::new(),
            class: String::new(),
            tip: String::new(),
            range: IRect::new_empty(),
            column: 0,
            row: 0,
            xpoints: Vec::new(),
            ypoints: Vec::new(),
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

    element!(BODY);

    subset!();

    text!();

    id_class!();

    tip!();

    column_row!();

    xpoints_ypoints!();

    range_background!();

    pub fn draw(&mut self, canvas: &Canvas) {
        if self.range.is_empty() {
            return;
        }
        let mut paint = Paint::default();
        paint.set_color(self.background);
        canvas.draw_irect(self.range, &paint);
    }
}

///"Area" is grid layout.
#[derive(Debug)]
pub struct Area {
    subset: VecDeque<TypeEntity>,
    text: String,
    id: String,
    class: String,
    tip: String,
    hidden: bool,
    range: IRect,
    column: i32,
    row: i32,
    xpoints: Vec<i32>,
    ypoints: Vec<i32>,
    background: Color,
}

impl Area {
    pub(crate) fn new() -> Self {
        Area {
            subset: VecDeque::new(),
            text: String::new(),
            id: String::new(),
            class: String::new(),
            tip: String::new(),
            hidden: false,
            range: IRect::new_empty(),
            column: 0,
            row: 0,
            xpoints: Vec::new(),
            ypoints: Vec::new(),
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

    element!(AREA);

    subset!();

    text!();

    id_class!();

    tip!();

    hidden!();

    column_row!();

    xpoints_ypoints!();

    range_background!();

    pub fn draw(&mut self, canvas: &Canvas) {
        if self.range.is_empty() {
            return;
        }
        let mut paint = Paint::default();
        paint.set_color(self.background);
        canvas.draw_irect(self.range, &paint);
    }
}

///"Dialog" is grid layout.
#[derive(Debug)]
pub struct Dialog {
    subset: VecDeque<TypeEntity>,
    text: String,
    id: String,
    class: String,
    tip: String,
    range: IRect,
    column: i32,
    row: i32,
    xpoints: Vec<i32>,
    ypoints: Vec<i32>,
    background: Color,
}

impl Dialog {
    pub fn new() -> Self {
        Dialog {
            subset: VecDeque::new(),
            text: String::new(),
            id: String::new(),
            class: String::new(),
            tip: String::new(),
            range: IRect::new_empty(),
            column: 0,
            row: 0,
            xpoints: Vec::new(),
            ypoints: Vec::new(),
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

    element!(DIALOG);

    subset!();

    text!();

    id_class!();

    tip!();

    range_background!();

    pub fn draw(&mut self, canvas: &Canvas) {
        if self.range.is_empty() {
            return;
        }
        let mut paint = Paint::default();
        paint.set_color(self.background);
        canvas.draw_irect(self.range, &paint);
    }
}
