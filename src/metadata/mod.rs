mod head;

pub(crate) use self::head::*;
use crate::markup::*;
use crate::page::*;
use crate::script::*;
use crate::style::*;
use crate::utils::*;
use std::sync::{Arc, RwLock};

///"Style" represents style.
#[derive(Debug)]
pub(crate) struct Style {
    style: Option<StyleContext>,
}

impl Style {
    pub(crate) fn new(cx: &mut PageContext) -> Self {
        let mut style = None;
        if let Some(s) = cx.text(cx.style_key()) {
            let mut o = StyleContext::new(s);
            o.set_style(cx);
            style.replace(o);
        }
        Self { style }
    }
}

///"Script" represents script.
#[derive(Debug)]
pub(crate) struct Script {
    rt: Option<ScriptRuntime>,
}

impl Script {
    pub(crate) fn new(cx: &mut PageContext) -> Self {
        Self {
            rt: Default::default(),
        }
    }

    pub(crate) fn build(&mut self, cx: Arc<RwLock<PageContext>>) {
        if self.rt.is_none() {
            let (s, t) = {
                let o = result_return!(cx.read());
                let e = option_return!(o.script_element().filter(|e| !e.text().is_empty()));
                let t = option_return!(e.script_type());
                (e.text().to_string(), t.clone())
            };
            let mut rt = ScriptRuntime::new(t, cx);
            rt.exec(s);
            self.rt.replace(rt);
        }
    }

    pub(crate) fn rebuild(&mut self) {
        if let Some(rt) = &mut self.rt {
            rt.rebuild();
        }
    }
}
