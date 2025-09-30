use crate::markup::{AttrName, Attribute, Element, Page};
use crate::script::ScriptRuntime;
use crate::style::StyleContext;
use std::sync::{Arc, RwLock};

///"Style" represents style.
pub(crate) struct Style {
    element: Arc<RwLock<Element>>,
    style: StyleContext,
}

impl std::fmt::Debug for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Style");
        if let Ok(o) = self.element.try_read() {
            f.field("element", &o.to_string());
        }
        f.finish()
    }
}

impl Style {
    pub(crate) fn new(element: Arc<RwLock<Element>>) -> Self {
        Self {
            element,
            style: StyleContext::new(),
        }
    }

    pub fn build(&mut self, page: &mut Page) {
        if let Ok(e) = self.element.read() {
            if e.text.is_empty() {
                return;
            }
            self.style.build(&e.text, page)
        }
    }
}

///"Script" represents script.
pub(crate) struct Script {
    element: Arc<RwLock<Element>>,
    script_rt: ScriptRuntime,
}

impl std::fmt::Debug for Script {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Script");
        if let Ok(o) = self.element.try_read() {
            f.field("element", &o.to_string());
        }
        f.finish()
    }
}

impl Script {
    pub(crate) fn new(element: Arc<RwLock<Element>>) -> Self {
        Self {
            element,
            script_rt: ScriptRuntime::new(),
        }
    }

    pub fn run(&mut self, page: &mut Page) {
        if let Ok(e) = self.element.read() {
            if e.text.is_empty() {
                return;
            }
            if let Some(Attribute::TYPE(t)) = e.attribute.get(&AttrName::TYPE) {
                self.script_rt.run(&e.text, &t, page)
            }
        }
    }
}
