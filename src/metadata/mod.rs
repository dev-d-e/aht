use crate::markup::{Attribute, Page, SCRIPT, STYLE};
use crate::parts::Subset;
use crate::script::ScriptRuntime;
use crate::style::StyleContext;

///"Style" represents style.
#[derive(Debug)]
pub struct Style {
    pub subset: Subset,
    pub text: String,
    pub class: String,
    pub id: String,
    style: StyleContext,
}

impl Style {
    pub(crate) fn new() -> Self {
        Self {
            subset: Subset::new(),
            text: String::new(),
            class: String::new(),
            id: String::new(),
            style: StyleContext::new(),
        }
    }

    pub fn attr(&mut self, attr: Attribute) {
        match attr {
            Attribute::ID(a) => self.id = a,
            Attribute::CLASS(a) => self.class = a,
            _ => {}
        }
    }

    element!(STYLE);

    pub fn build(&mut self, page: &mut Page) {
        if self.text.is_empty() {
            return;
        }
        self.style.build(&self.text, page)
    }
}

///"Script" represents script.
#[derive(Debug)]
pub struct Script {
    pub subset: Subset,
    pub text: String,
    pub class: String,
    pub id: String,
    script_rt: ScriptRuntime,
}

impl Script {
    pub(crate) fn new() -> Self {
        Self {
            subset: Subset::new(),
            text: String::new(),
            class: String::new(),
            id: String::new(),
            script_rt: ScriptRuntime::new(),
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

    pub fn run(&mut self, page: &mut Page) {
        if self.text.is_empty() {
            return;
        }
        self.script_rt.run(&self.text, page)
    }
}
