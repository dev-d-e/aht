mod css;

use crate::markup::Page;
use css::CssParser;

#[derive(Debug)]
pub(crate) struct StyleContext {
    parser: Option<CssParser>,
}

impl StyleContext {
    pub(crate) fn new() -> Self {
        Self { parser: None }
    }

    pub fn build(&mut self, s: &str, page: &mut Page) {
        if self.parser.is_none() {
            self.parser.replace(CssParser::new());
        }

        if let Some(p) = &mut self.parser {
            p.parse(s, page);
        }
    }
}
