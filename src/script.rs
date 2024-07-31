use crate::markup::{Attribute, SCRIPT};
use crate::parts::Subset;

///"Script" represents script.
#[derive(Debug)]
pub struct Script {
    pub subset: Subset,
    pub text: String,
    pub class: String,
    pub id: String,
}

impl Script {
    pub fn new() -> Self {
        Script {
            subset: Subset::new(),
            text: String::new(),
            class: String::new(),
            id: String::new(),
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::CLASS(a) => self.class = a,
            Attribute::ID(a) => self.id = a,
            _ => {}
        }
    }

    element!(SCRIPT);
}
