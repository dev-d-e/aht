use crate::markup::{Attribute, AHT, HEAD, TITLE};
use crate::parts::Subset;

///"Head" represents head.
#[derive(Debug)]
pub struct Head {
    pub subset: Subset,
    pub text: String,
    pub class: String,
    pub id: String,
    pub lang: String,
}

impl Head {
    pub(crate) fn new() -> Self {
        Self {
            subset: Subset::new(),
            text: String::new(),
            class: String::new(),
            id: String::new(),
            lang: String::new(),
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::CLASS(a) => self.class = a,
            Attribute::ID(a) => self.id = a,
            Attribute::LANG(a) => self.lang = a,
            _ => {}
        }
    }

    element!(HEAD);
}

///"Title" represents title.
#[derive(Debug)]
pub struct Title {
    pub subset: Subset,
    pub text: String,
    pub class: String,
    pub id: String,
    pub tip: String,
}

impl Title {
    pub(crate) fn new() -> Self {
        Self {
            subset: Subset::new(),
            text: String::new(),
            class: String::new(),
            id: String::new(),
            tip: String::new(),
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::CLASS(a) => self.class = a,
            Attribute::ID(a) => self.id = a,
            _ => {}
        }
    }

    element!(TITLE);
}

///"Aht" represents root.
#[derive(Debug)]
pub struct Aht {
    pub(crate) subset: Subset,
    pub(crate) class: String,
    pub(crate) id: String,
}

impl Aht {
    pub(crate) fn new() -> Self {
        Self {
            subset: Subset::new(),
            class: String::new(),
            id: String::new(),
        }
    }

    pub(crate) fn attr(&mut self, attr: Attribute) {
        match attr {
            _ => {}
        }
    }

    element!(AHT);
}
