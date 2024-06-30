use crate::markup::{Attribute, TypeEntity, HEAD, TITLE};
use std::collections::VecDeque;

///"Head" represents head.
#[derive(Debug)]
pub struct Head {
    subset: VecDeque<TypeEntity>,
    text: String,
    id: String,
    class: String,
    lang: String,
}

impl Head {
    pub(crate) fn new() -> Self {
        Head {
            subset: VecDeque::new(),
            text: String::new(),
            id: String::new(),
            class: String::new(),
            lang: String::new(),
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::ID(a) => self.set_id(a),
            Attribute::CLASS(a) => self.set_class(a),

            _ => {}
        }
    }

    element!(HEAD);

    subset!();

    text!();

    id_class!();
}

///"Title" represents title.
#[derive(Debug)]
pub struct Title {
    subset: VecDeque<TypeEntity>,
    text: String,
    id: String,
    class: String,
    tip: String,
}

impl Title {
    pub(crate) fn new() -> Self {
        Title {
            subset: VecDeque::new(),
            text: String::new(),
            id: String::new(),
            class: String::new(),
            tip: String::new(),
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::ID(a) => self.set_id(a),
            Attribute::CLASS(a) => self.set_class(a),

            _ => {}
        }
    }

    element!(TITLE);

    subset!();

    text!();

    id_class!();

    tip!();
}
