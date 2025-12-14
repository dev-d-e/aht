use super::*;
use crate::utils::ascii::*;
use std::str::FromStr;
use std::sync::{Arc, RwLock};

pub(super) struct Parser {
    current_function: fn(&mut Self),
    next_function: fn(&mut Self),
    c: char,
    n: usize,
    temporary: (String, String),
    o: StyleRule,
    rst: StyleSheet,
    error: ErrorHolder,
}

impl Parser {
    pub(crate) fn new() -> Self {
        Self {
            current_function: Self::start,
            next_function: Self::start,
            c: 0 as char,
            n: 0,
            temporary: (String::new(), String::new()),
            o: Default::default(),
            rst: Default::default(),
            error: Default::default(),
        }
    }
    pub(crate) fn parse(mut self, mut i: impl Iterator<Item = char>) -> (StyleSheet, ErrorHolder) {
        while let Some(c) = i.next() {
            self.c = c;
            (self.current_function)(&mut self);
            self.n += 1;
        }
        (self.rst, self.error)
    }

    pub(crate) fn parse_str(self, s: &str) -> (StyleSheet, ErrorHolder) {
        self.parse(s.chars())
    }

    fn push_at(&mut self) {}

    fn push_mark(&mut self) {
        if !self.temporary.0.is_empty() {
            let s = self.temporary.0.drain(..).as_str().to_lowercase();
            if let Ok(m) = Mark::try_from(&s) {
                let k = (Selector::Mark(m), Combiner::Descendant(0));
                self.o.key_mut().push(k);
            }
        }
    }

    fn push_selector(&mut self) {
        if !self.temporary.1.is_empty() {
            let v = self.temporary.1.drain(..).as_str().trim().to_string();
            if self.temporary.0.is_empty() {
                let k = (
                    Selector::Attribute(AttrName::CLASS, AttrPattern::Contain(v)),
                    Combiner::Descendant(0),
                );
                self.o.key_mut().push(k);
            } else {
                let n = self.temporary.0.drain(..).as_str().to_string();
                match AttrName::from_str(&n) {
                    Ok(a) => {
                        let k = (
                            Selector::Attribute(a, AttrPattern::Contain(v)),
                            Combiner::Descendant(0),
                        );
                        self.o.key_mut().push(k);
                    }
                    Err(e) => {
                        error!("Unclear selector attribute {e}");
                    }
                }
            }
        }
    }

    fn push_attribute(&mut self) {
        if self.temporary.0.is_empty() {
            self.temporary.1.clear();
        } else {
            let n = self.temporary.0.drain(..).as_str().to_string();
            let mut v = self.temporary.1.drain(..).as_str().trim().to_string();
            match Attribute::from_s(&n, &mut v) {
                Ok(a) => {
                    self.o.attribute_mut().insert(a.name(), a);
                }
                Err(e) => {
                    error!("Unclear attribute {e}");
                }
            }
        }
    }

    fn push_style_rule(&mut self) {
        self.push_attribute();
        let o = std::mem::take(&mut self.o);
        self.rst.style_rules_mut().push(o);
    }

    fn ignore(&mut self) {
        match self.c {
            SPACE | CR | LF => {}
            _ => {
                self.current_function = self.next_function;
                (self.current_function)(self);
            }
        }
    }

    fn start(&mut self) {
        let c = self.c;
        match c {
            AT => {
                self.current_function = Self::at_0;
            }
            'A'..='Z' | 'a'..='z' => {
                self.current_function = Self::m_selector;
                self.m_selector();
            }
            FULL_STOP => {
                self.current_function = Self::a_selector_0;
            }
            LEFT_CURLY_BRACKET => {
                self.current_function = Self::ignore;
                self.next_function = Self::attr_expr_0;
            }
            RIGHT_CURLY_BRACKET => {
                self.push_style_rule();
                self.current_function = Self::ignore;
                self.next_function = Self::start;
            }
            _ => {}
        }
    }

    fn at_0(&mut self) {
        let c = self.c;
        match c {
            SPACE => {
                self.push_at();
                self.current_function = Self::ignore;
                self.next_function = Self::start;
            }
            LEFT_CURLY_BRACKET => {
                self.push_at();
                self.current_function = Self::ignore;
                self.next_function = Self::at_1;
            }
            _ => {}
        }
    }

    fn at_1(&mut self) {
        let c = self.c;
        match c {
            RIGHT_CURLY_BRACKET => {
                self.push_at();
                self.current_function = Self::ignore;
                self.next_function = Self::start;
            }
            _ => {
                self.temporary.0.push(c);
            }
        }
    }

    fn m_selector(&mut self) {
        let c = self.c;
        match c {
            'A'..='Z' | 'a'..='z' => {
                self.temporary.0.push(c);
            }
            SPACE => {
                self.push_mark();
                self.current_function = Self::ignore;
                self.next_function = Self::start;
            }
            LEFT_CURLY_BRACKET => {
                self.push_mark();
                self.current_function = Self::ignore;
                self.next_function = Self::attr_expr_0;
            }
            _ => {}
        }
    }

    fn a_selector_0(&mut self) {
        let c = self.c;
        match c {
            'A'..='Z' | 'a'..='z' => {
                self.temporary.1.push(c);
            }
            EQUAL => {
                std::mem::swap(&mut self.temporary.0, &mut self.temporary.1);
                self.current_function = Self::ignore;
                self.next_function = Self::a_selector_1;
            }
            SPACE => {
                self.push_selector();
                self.current_function = Self::ignore;
                self.next_function = Self::start;
            }
            LEFT_CURLY_BRACKET => {
                self.push_selector();
                self.current_function = Self::ignore;
                self.next_function = Self::attr_expr_0;
            }
            _ => {}
        }
    }

    fn a_selector_1(&mut self) {
        let c = self.c;
        match c {
            'A'..='Z' | 'a'..='z' => {
                self.temporary.1.push(c);
            }
            SPACE => {
                self.push_selector();
                self.current_function = Self::ignore;
                self.next_function = Self::start;
            }
            LEFT_CURLY_BRACKET => {
                self.push_selector();
                self.current_function = Self::ignore;
                self.next_function = Self::attr_expr_0;
            }
            _ => {}
        }
    }

    fn attr_expr_0(&mut self) {
        let c = self.c;
        match c {
            COLON => {
                self.current_function = Self::attr_expr_0_1;
            }
            SEMICOLON => {}
            LEFT_CURLY_BRACKET => {}
            RIGHT_CURLY_BRACKET => {
                self.push_style_rule();
                self.current_function = Self::ignore;
                self.next_function = Self::start;
            }
            _ => {
                self.current_function = Self::attr_expr_1;
                self.attr_expr_1();
            }
        }
    }

    fn attr_expr_0_1(&mut self) {
        match self.c {
            SEMICOLON => {
                self.current_function = Self::attr_expr_0;
            }
            RIGHT_CURLY_BRACKET => {
                self.push_style_rule();
                self.current_function = Self::ignore;
                self.next_function = Self::start;
            }
            _ => {}
        }
    }

    fn attr_expr_1(&mut self) {
        let c = self.c;
        match c {
            SPACE => {
                self.current_function = Self::ignore;
                self.next_function = Self::attr_expr_2;
            }
            COLON => {
                self.current_function = Self::ignore;
                self.next_function = Self::attr_expr_3;
            }
            RIGHT_CURLY_BRACKET => {
                self.push_style_rule();
                self.current_function = Self::ignore;
                self.next_function = Self::start;
            }
            _ => {
                self.temporary.0.push(c);
            }
        }
    }

    fn attr_expr_2(&mut self) {
        let c = self.c;
        match c {
            COLON => {
                self.current_function = Self::ignore;
                self.next_function = Self::attr_expr_3;
            }
            SEMICOLON => {
                self.push_attribute();
                self.current_function = Self::ignore;
                self.next_function = Self::attr_expr_0;
            }
            RIGHT_CURLY_BRACKET => {
                self.push_style_rule();
                self.current_function = Self::ignore;
                self.next_function = Self::start;
            }
            _ => {}
        }
    }

    fn attr_expr_3(&mut self) {
        let c = self.c;
        match c {
            SEMICOLON => {
                self.push_attribute();
                self.current_function = Self::ignore;
                self.next_function = Self::attr_expr_0;
            }
            RIGHT_CURLY_BRACKET => {
                self.push_style_rule();
                self.current_function = Self::ignore;
                self.next_function = Self::start;
            }
            _ => {
                self.temporary.1.push(c);
            }
        }
    }
}
