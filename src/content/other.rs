use crate::markup::{Attribute, TypeEntity, CANVAS, IFRAME};
use skia_safe::{Canvas, Color, IRect, Paint, Rect};
use std::collections::VecDeque;

///"Canv" represents canvas.
#[derive(Debug)]
pub struct Canv {
    subset: VecDeque<TypeEntity>,
    text: String,
    id: String,
    class: String,
    tip: String,
    hidden: bool,
    range: IRect,
    background: Color,
}

impl Canv {
    pub fn new() -> Self {
        Canv {
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

    element!(CANVAS);

    subset!();

    text!();

    id_class!();

    tip!();

    hidden!();

    range_background!();

    pub fn draw(&mut self, canvas: &Canvas) {}
}

///"Iframe" represents iframe.
#[derive(Debug)]
pub struct Iframe {
    subset: VecDeque<TypeEntity>,
    text: String,
    id: String,
    class: String,
    tip: String,
    hidden: bool,
    src: String,
    range: IRect,
    background: Color,
}

impl Iframe {
    pub fn new() -> Self {
        Iframe {
            subset: VecDeque::new(),
            text: String::new(),
            id: String::new(),
            class: String::new(),
            tip: String::new(),
            hidden: false,
            src: String::new(),
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

    element!(IFRAME);

    subset!();

    text!();

    id_class!();

    tip!();

    hidden!();

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
