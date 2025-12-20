mod entity;
mod format;

use self::entity::*;
use self::format::*;
use crate::error::*;
use crate::markup::*;
use crate::page::*;
use crate::utils::*;
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub(crate) struct StyleContext {
    style_sheet: StyleSheet,
}

impl StyleContext {
    pub(crate) fn new() -> Self {
        Self {
            style_sheet: Default::default(),
        }
    }

    pub(crate) fn build(&mut self, s: &str, context: &mut PageContext) {
        let (r, _) = StyleSheetBuilder::build(s);
        self.style_sheet = r;

        self.set_style(vec![context.body_element().clone()]);
    }

    pub(crate) fn set_style(&mut self, v: Vec<Arc<RwLock<Element>>>) {
        for i in self.style_sheet.style_rules_mut().iter_mut() {
            let r = i.key().find(v.clone());
            if r.is_empty() {
                continue;
            }
            for o in &r {
                if let Ok(mut e) = o.write() {
                    for a in i.attribute().values() {
                        e.attribute_insert(a.clone());
                    }
                }
            }
        }
    }
}
