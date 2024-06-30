use crate::markup::{Attribute, TypeEntity, SCRIPT};
use std::collections::VecDeque;

///"Script" represents script.
#[derive(Debug)]
pub struct Script {
    subset: VecDeque<TypeEntity>,
    text: String,
    id: String,
    class: String,
}

impl Script {
    pub(crate) fn new() -> Self {
        Script {
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

    element!(SCRIPT);

    subset!();

    text!();

    id_class!();
}
