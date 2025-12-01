mod css;

use self::css::*;
use crate::markup::*;
use crate::page::*;
use crate::utils::*;

#[derive(Debug)]
pub(crate) struct StyleContext {
    parser: Option<CssParser>,
}

impl StyleContext {
    pub(crate) fn new() -> Self {
        Self { parser: None }
    }

    pub(crate) fn build(&mut self, s: &str, context: &mut PageContext) {
        if self.parser.is_none() {
            self.parser.replace(Default::default());
        }

        if let Some(p) = &mut self.parser {
            p.parse(s, context);
        }
    }
}
