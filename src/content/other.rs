use crate::markup::{Attribute, TypeEntity, CANVAS, IFRAME};
use crate::parts::{Ordinal, RectangleRange, Subset};
use skia_safe::{Canvas, Color, Paint};

///"Canv" represents canvas.
#[derive(Debug)]
pub struct Canv {
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

impl Canv {
    pub fn new() -> Self {
        Canv {
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

    element!(CANVAS);

    subset!();

    text!();

    class_id!();

    hidden!();

    ordinal!();

    tip!();

    range_background!();

    pub fn draw(&mut self, canvas: &Canvas) {}
}

///"Iframe" represents iframe.
#[derive(Debug)]
pub struct Iframe {
    subset: Subset,
    text: String,
    class: String,
    hidden: bool,
    id: String,
    src: String,
    tip: String,
    range: RectangleRange,
    background: Color,
}

impl Iframe {
    pub fn new() -> Self {
        Iframe {
            subset: Subset::new(),
            text: String::new(),
            class: String::new(),
            hidden: false,
            id: String::new(),
            src: String::new(),
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
            Attribute::SRC(a) => self.set_src(a),
            Attribute::TIP(a) => self.set_tip(a),
            Attribute::WIDTH(a) => self.set_width(a),
            _ => {}
        }
    }

    element!(IFRAME);

    subset!();

    text!();

    class_id!();

    hidden!();

    src!();

    tip!();

    range_background!();

    pub fn draw(&mut self, canvas: &Canvas) {
        if self.range.is_empty() {
            return;
        }
        let mut paint = Paint::default();
        paint.set_color(self.background);
        canvas.draw_irect(self.range.to_irect(), &paint);
    }
}
