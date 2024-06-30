use crate::markup::{Attribute, TypeEntity, CSS};
use std::collections::VecDeque;

///"Css" represents css.
#[derive(Debug)]
pub struct Css {
    subset: VecDeque<TypeEntity>,
    text: String,
    id: String,
    class: String,
}

impl Css {
    pub(crate) fn new() -> Self {
        Css {
            subset: VecDeque::new(),
            text: String::new(),
            id: String::new(),
            class: String::new(),
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::ID(a) => self.set_id(a),
            Attribute::CLASS(a) => self.set_class(a),

            _ => {}
        }
    }

    element!(CSS);

    subset!();

    text!();

    id_class!();
}
