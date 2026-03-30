mod entity;
mod format;

use self::entity::*;
use self::format::*;
use crate::error::*;
use crate::markup::*;
use crate::utils::*;

#[derive(Debug)]
pub(crate) struct StyleContext {
    style_sheet: StyleSheet,
}

impl StyleContext {
    pub(crate) fn new(s: &str) -> Self {
        let (style_sheet, err) = StyleSheetBuilder::build(s);
        if err.len() > 0 {
            info!("{}", err);
        }
        Self { style_sheet }
    }

    pub(crate) fn set_style(&mut self, eh: &mut ElementHolder) {
        let ss = self.style_sheet.style_rules_mut().iter_mut();
        for sr in ss {
            let ks = sr.key().find(eh);
            debug!("{:?} : {:?}", sr.key(), ks);
            if ks.is_empty() {
                continue;
            }
            for k in ks {
                if let Some(e) = eh.get_mut(k) {
                    for a in sr.attribute().values() {
                        e.attribute_insert(a.clone());
                    }
                }
            }
        }
    }
}
