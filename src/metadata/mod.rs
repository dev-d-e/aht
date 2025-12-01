mod head;

pub(crate) use self::head::*;
use crate::markup::*;
use crate::page::*;
use crate::script::*;
use crate::style::*;
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

    pub(crate) fn build(&mut self, context: &mut PageContext) {
        if let Ok(e) = self.element.read() {
            if e.text().is_empty() {
                return;
            }
            self.style.build(e.text(), context)
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

    pub(crate) fn run(&mut self, context: &mut PageContext) {
        if let Ok(e) = self.element.read() {
            if e.text().is_empty() {
                return;
            }
            if let Some(t) = e.attribute().script_type() {
                self.script_rt.run(e.text(), &t, context)
            }
        }
    }
}
