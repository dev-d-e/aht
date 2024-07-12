use crate::markup::{Attribute, TypeEntity, AREA, BODY, DIALOG};
use crate::parts::{Ordinal, Points, RectangleRange, Subset};
use skia_safe::{Canvas, Color, Paint};
use std::fmt::Debug;

///"Body" is grid layout.
#[derive(Debug)]
pub struct Body {
    subset: Subset,
    text: String,
    class: String,
    column: Points,
    id: String,
    row: Points,
    tip: String,
    range: RectangleRange,
    background: Color,
}

impl Body {
    pub fn new() -> Self {
        Body {
            subset: Subset::new(),
            text: String::new(),
            class: String::new(),
            column: Points::empty(),
            id: String::new(),
            row: Points::empty(),
            tip: String::new(),
            range: RectangleRange::new(),
            background: Color::WHITE,
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::CLASS(a) => self.set_class(a),
            Attribute::COLUMN(a) => self.set_column(a),
            Attribute::HEIGHT(a) => self.set_height(a),
            Attribute::ID(a) => self.set_id(a),
            Attribute::ROW(a) => self.set_row(a),
            Attribute::TIP(a) => self.set_tip(a),
            Attribute::WIDTH(a) => self.set_width(a),
            _ => {}
        }
    }

    element!(BODY);

    subset!();

    text!();

    class_id!();

    column_row!();

    tip!();

    range_background!();

    pub fn draw(&mut self, canvas: &Canvas) {
        if self.range.is_empty() {
            return;
        }
        let mut paint = Paint::default();
        paint.set_color(self.background);
        canvas.draw_irect(self.range.to_irect(), &paint);

        set_subset_xy(&self.column, &self.row, &self.range, &mut self.subset);

        self.subset.draw(canvas);
    }
}

///"Area" is grid layout.
#[derive(Debug)]
pub struct Area {
    subset: Subset,
    text: String,
    class: String,
    column: Points,
    hidden: bool,
    id: String,
    ordinal: Ordinal,
    row: Points,
    tip: String,
    range: RectangleRange,
    background: Color,
}

impl Area {
    pub(crate) fn new() -> Self {
        Area {
            subset: Subset::new(),
            text: String::new(),
            class: String::new(),
            column: Points::empty(),
            hidden: false,
            id: String::new(),
            ordinal: Ordinal::None,
            row: Points::empty(),
            tip: String::new(),
            range: RectangleRange::new(),
            background: Color::WHITE,
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::CLASS(a) => self.set_class(a),
            Attribute::COLUMN(a) => self.set_column(a),
            Attribute::HEIGHT(a) => self.set_height(a),
            Attribute::HIDDEN(a) => self.set_hidden(a),
            Attribute::ID(a) => self.set_id(a),
            Attribute::ORDINAL(a) => self.set_ordinal(a),
            Attribute::ROW(a) => self.set_row(a),
            Attribute::TIP(a) => self.set_tip(a),
            Attribute::WIDTH(a) => self.set_width(a),
            _ => {}
        }
    }

    element!(AREA);

    subset!();

    text!();

    class_id!();

    column_row!();

    hidden!();

    ordinal!();

    tip!();

    range_background!();

    pub fn draw(&mut self, canvas: &Canvas) {
        if self.range.is_empty() {
            return;
        }
        let mut paint = Paint::default();
        paint.set_color(self.background);
        canvas.draw_irect(self.range.to_irect(), &paint);

        set_subset_xy(&self.column, &self.row, &self.range, &mut self.subset);

        self.subset.draw(canvas);
    }
}

///"Dialog" is grid layout.
#[derive(Debug)]
pub struct Dialog {
    subset: Subset,
    text: String,
    class: String,
    column: Points,
    hidden: bool,
    id: String,
    row: Points,
    tip: String,
    range: RectangleRange,
    background: Color,
}

impl Dialog {
    pub fn new() -> Self {
        Dialog {
            subset: Subset::new(),
            text: String::new(),
            class: String::new(),
            column: Points::empty(),
            hidden: false,
            id: String::new(),
            row: Points::empty(),
            tip: String::new(),
            range: RectangleRange::new(),
            background: Color::WHITE,
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::CLASS(a) => self.set_class(a),
            Attribute::COLUMN(a) => self.set_column(a),
            Attribute::HEIGHT(a) => self.set_height(a),
            Attribute::HIDDEN(a) => self.set_hidden(a),
            Attribute::ID(a) => self.set_id(a),
            Attribute::ROW(a) => self.set_row(a),
            Attribute::TIP(a) => self.set_tip(a),
            Attribute::WIDTH(a) => self.set_width(a),
            _ => {}
        }
    }

    element!(DIALOG);

    subset!();

    text!();

    class_id!();

    column_row!();

    hidden!();

    tip!();

    range_background!();

    pub fn draw(&mut self, canvas: &Canvas) {
        if self.range.is_empty() {
            return;
        }
        let mut paint = Paint::default();
        paint.set_color(self.background);
        canvas.draw_irect(self.range.to_irect(), &paint);

        set_subset_xy(&self.column, &self.row, &self.range, &mut self.subset);

        self.subset.draw(canvas);
    }
}

//
fn set_subset_xy(column: &Points, row: &Points, range: &RectangleRange, subset: &mut Subset) {
    let mut subset_xy = SubsetXY::new(column.effect(range.width), row.effect(range.height));
    let v = subset.vec();
    for o in v {
        match o {
            TypeEntity::AREA(o) => ordinal_xy!(subset_xy, o),
            TypeEntity::AUDIO(o) => ordinal_xy!(subset_xy, o),
            TypeEntity::BUTTON(o) => ordinal_xy!(subset_xy, o),
            TypeEntity::CANVAS(o) => ordinal_xy!(subset_xy, o),
            TypeEntity::IMG(o) => ordinal_xy!(subset_xy, o),
            TypeEntity::INP(o) => ordinal_xy!(subset_xy, o),
            TypeEntity::PT(o) => ordinal_xy!(subset_xy, o),
            TypeEntity::SELECT(o) => ordinal_xy!(subset_xy, o),
            TypeEntity::TIME(o) => ordinal_xy!(subset_xy, o),
            TypeEntity::VIDEO(o) => ordinal_xy!(subset_xy, o),
            _ => {}
        }
    }
}
