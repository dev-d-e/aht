use crate::markup::{Attribute, CSS};
use crate::parts::Subset;

///"Css" represents css.
#[derive(Debug)]
pub struct Css {
    pub subset: Subset,
    pub text: String,
    pub class: String,
    pub id: String,
}

impl Css {
    pub fn new() -> Self {
        Css {
            subset: Subset::new(),
            text: String::new(),
            class: String::new(),
            id: String::new(),
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::ID(a) => self.id = a,
            Attribute::CLASS(a) => self.class = a,

            _ => {}
        }
    }

    element!(CSS);
}
