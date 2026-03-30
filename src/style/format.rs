use super::*;
use crate::utils::ascii::*;
use std::mem::take;

pub(super) trait Output {
    fn meta(&mut self, s: String);

    fn mark_selector(&mut self, c: String, s: String);

    fn attribute_selector(&mut self, c: String, k: String, v: String);

    fn attribute(&mut self, k: String, v: String);

    fn end_block(&mut self);

    fn error(&mut self, e: Error);
}

pub(super) struct Parser<T>
where
    T: Output,
{
    current_function: fn(&mut Self),
    next_function: fn(&mut Self),
    c: char,
    counter: CharCounter,
    temporary: (String, String),
    temporary_c: String,
    output: T,
}

impl<T> Parser<T>
where
    T: Output,
{
    pub(super) fn new(output: T) -> Self {
        Self {
            current_function: Self::ignore,
            next_function: Self::start,
            c: NULL,
            counter: Default::default(),
            temporary: (String::new(), String::new()),
            temporary_c: String::new(),
            output,
        }
    }

    pub(super) fn parse(mut self, mut i: impl Iterator<Item = char>) -> T {
        while let Some(c) = i.next() {
            self.c = c;
            (self.current_function)(&mut self);
            self.counter.count(c);
        }
        self.output
    }

    pub(super) fn parse_str(self, s: &str) -> T {
        self.parse(s.chars())
    }

    fn output_at(&mut self) {
        if self.temporary.0.is_empty() {
            self.temporary.1.clear();
        } else {
            let s = self.temporary.0.drain(..).as_str().to_lowercase();
            self.output.meta(s);
        }
    }

    fn output_m_selector(&mut self) {
        if self.temporary.0.is_empty() {
            self.temporary_c.clear();
            self.temporary.1.clear();
        } else {
            let c = take(&mut self.temporary_c);
            let s = self.temporary.0.drain(..).as_str().to_lowercase();
            self.output.mark_selector(c, s);
        }
    }

    fn output_a_selector(&mut self) {
        if self.temporary.1.is_empty() {
            self.temporary_c.clear();
            self.temporary.0.clear();
        } else {
            let c = take(&mut self.temporary_c);
            let n = self.temporary.0.drain(..).as_str().to_lowercase();
            let v = take(&mut self.temporary.1);
            self.output.attribute_selector(c, n, v);
        }
    }

    fn output_attr(&mut self) {
        if self.temporary.0.is_empty() {
            self.temporary.1.clear();
        } else {
            let n = self.temporary.0.drain(..).as_str().to_lowercase();
            let v = take(&mut self.temporary.1);
            self.output.attribute(n, v);
        }
    }

    fn output_end(&mut self) {
        self.output_attr();
        self.output.end_block();
    }

    fn output_error(&mut self, s: &str) {
        let e = self.counter.to_error(ErrorKind::Style, s);
        self.output.error(e);
    }

    fn ignore(&mut self) {
        match self.c {
            SPACE | LF | CR => {}
            _ => {
                self.current_function = self.next_function;
                (self.current_function)(self);
            }
        }
    }

    fn start(&mut self) {
        let c = self.c;
        match c {
            EXCLAMATION => {
                self.current_function = Self::meta_0;
            }
            'A'..='Z' | 'a'..='z' => {
                self.current_function = Self::m_selector;
                self.m_selector();
            }
            FULL_STOP => {
                self.current_function = Self::a_selector_0;
            }
            _ => {
                self.output_error("illegal char");
                self.current_function = Self::ignore;
                self.next_function = Self::start;
            }
        }
    }

    fn next_0(&mut self) {
        let c = self.c;
        match c {
            'A'..='Z' | 'a'..='z' => {
                self.current_function = Self::m_selector;
                self.m_selector();
            }
            FULL_STOP => {
                self.current_function = Self::a_selector_0;
            }
            QUESTION | PLUS | HYPHEN => {
                self.temporary_c.push(c);
                self.current_function = Self::combiner;
            }
            LEFT_CURLY_BRACKET => {
                self.current_function = Self::ignore;
                self.next_function = Self::attr_expr_0;
            }
            _ => {
                self.output_error("illegal char");
                self.current_function = Self::next_0_1;
            }
        }
    }

    fn next_0_1(&mut self) {
        match self.c {
            SPACE | LF | CR => {
                self.current_function = Self::ignore;
                self.next_function = Self::next_0;
            }
            _ => {}
        }
    }

    fn next_1(&mut self) {
        let c = self.c;
        match c {
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
            _ => {
                self.output_error("illegal char");
                self.current_function = Self::next_1_1;
            }
        }
    }

    fn next_1_1(&mut self) {
        match self.c {
            SPACE | LF | CR => {
                self.current_function = Self::ignore;
                self.next_function = Self::next_1;
            }
            _ => {}
        }
    }

    fn meta_0(&mut self) {
        let c = self.c;
        match c {
            'A'..='Z' | 'a'..='z' | HYPHEN => {
                self.current_function = Self::meta_1;
                self.meta_1();
            }
            _ => {
                self.output_error("illegal char");
                self.current_function = Self::ignore;
                self.next_function = Self::meta_0_1;
            }
        }
    }

    fn meta_1(&mut self) {
        let c = self.c;
        match c {
            'A'..='Z' | 'a'..='z' | HYPHEN => {
                self.temporary.0.push(c);
            }
            SPACE => {
                self.output_at();
                self.current_function = Self::ignore;
                self.next_function = Self::meta_2;
            }
            _ => {
                self.current_function = Self::meta_2;
                self.meta_2();
            }
        }
    }

    fn meta_2(&mut self) {
        match self.c {
            LEFT_CURLY_BRACKET => {
                self.output_at();
                self.current_function = Self::ignore;
                self.next_function = Self::attr_expr_0;
            }
            RIGHT_CURLY_BRACKET => {
                self.output_error("illegal char");
                self.current_function = Self::ignore;
                self.next_function = Self::start;
            }
            _ => {
                self.output_error("illegal char");
                self.current_function = Self::ignore;
                self.next_function = Self::meta_0_1;
            }
        }
    }

    fn meta_0_1(&mut self) {
        match self.c {
            RIGHT_CURLY_BRACKET => {
                self.current_function = Self::ignore;
                self.next_function = Self::start;
            }
            _ => {}
        }
    }

    fn m_selector(&mut self) {
        let c = self.c;
        match c {
            'A'..='Z' | 'a'..='z' => {
                self.temporary.0.push(c);
            }
            SPACE => {
                self.output_m_selector();
                self.current_function = Self::ignore;
                self.next_function = Self::next_0;
            }
            LEFT_CURLY_BRACKET => {
                self.output_m_selector();
                self.current_function = Self::ignore;
                self.next_function = Self::attr_expr_0;
            }
            RIGHT_CURLY_BRACKET => {
                self.output_error("illegal char");
            }
            _ => {
                self.output_error("invalid mark");
            }
        }
    }

    fn a_selector_0(&mut self) {
        match self.c {
            'A'..='Z' | 'a'..='z' => {
                self.current_function = Self::a_selector_1;
                self.a_selector_1();
            }
            EQUAL => {
                self.current_function = Self::a_selector_2;
                self.a_selector_2();
            }
            _ => {
                self.output_error("invalid attribute");
            }
        }
    }

    fn a_selector_1(&mut self) {
        let c = self.c;
        match c {
            'A'..='Z' | 'a'..='z' => {
                self.temporary.1.push(c);
            }
            EQUAL => {
                self.current_function = Self::a_selector_2;
                self.a_selector_2();
            }
            SPACE => {
                self.current_function = Self::ignore;
                self.next_function = Self::a_selector_2;
            }
            LEFT_CURLY_BRACKET => {
                self.current_function = Self::a_selector_4;
                self.a_selector_4();
            }
            _ => {
                self.output_error("invalid attribute");
            }
        }
    }

    fn a_selector_2(&mut self) {
        match self.c {
            EQUAL => {
                std::mem::swap(&mut self.temporary.0, &mut self.temporary.1);
                self.current_function = Self::ignore;
                self.next_function = Self::a_selector_3;
            }
            LEFT_CURLY_BRACKET => {
                self.current_function = Self::a_selector_4;
                self.a_selector_4();
            }
            _ => {
                self.output_a_selector();
                self.current_function = Self::ignore;
                self.next_function = Self::next_0;
                self.ignore();
            }
        }
    }

    fn a_selector_3(&mut self) {
        let c = self.c;
        match c {
            'A'..='Z' | 'a'..='z' => {
                self.temporary.1.push(c);
            }
            SPACE => {
                self.output_a_selector();
                self.current_function = Self::ignore;
                self.next_function = Self::a_selector_4;
            }
            LEFT_CURLY_BRACKET => {
                self.current_function = Self::a_selector_4;
                self.a_selector_4();
            }
            _ => {
                self.output_a_selector();
                self.output_error("invalid attribute");
            }
        }
    }

    fn a_selector_4(&mut self) {
        match self.c {
            LEFT_CURLY_BRACKET => {
                self.output_a_selector();
                self.current_function = Self::ignore;
                self.next_function = Self::attr_expr_0;
            }
            _ => {
                self.current_function = Self::ignore;
                self.next_function = Self::next_0;
            }
        }
    }

    fn combiner(&mut self) {
        let c = self.c;
        match c {
            '0'..='9' => {
                self.temporary_c.push(c);
            }
            SPACE => {
                self.current_function = Self::ignore;
                self.next_function = Self::next_1;
            }
            LEFT_CURLY_BRACKET => {
                self.temporary_c.clear();
                self.current_function = Self::ignore;
                self.next_function = Self::attr_expr_0;
            }
            _ => {
                self.temporary_c.clear();
                self.output_error("illegal char");
                self.current_function = Self::ignore;
                self.next_function = Self::next_1;
            }
        }
    }

    fn attr_expr_0(&mut self) {
        let c = self.c;
        match c {
            'A'..='Z' | 'a'..='z' | HYPHEN => {
                self.current_function = Self::attr_expr_1;
                self.attr_expr_1();
            }
            COLON => {
                self.current_function = Self::ignore;
                self.next_function = Self::attr_expr_0_1;
            }
            SEMICOLON => {
                self.current_function = Self::ignore;
                self.next_function = Self::attr_expr_1;
            }
            RIGHT_CURLY_BRACKET => {
                self.current_function = Self::attr_expr_2;
                self.attr_expr_2();
            }
            _ => {
                self.output_error("illegal char");
                self.current_function = Self::attr_expr_0_2;
                self.attr_expr_0_2();
            }
        }
    }

    fn attr_expr_0_1(&mut self) {
        match self.c {
            SEMICOLON | RIGHT_CURLY_BRACKET => {
                self.current_function = Self::attr_expr_0_2;
                self.attr_expr_0_2();
            }
            _ => {
                self.output_error("illegal char");
                self.current_function = Self::attr_expr_0_2;
                self.attr_expr_0_2();
            }
        }
    }

    fn attr_expr_0_2(&mut self) {
        match self.c {
            SEMICOLON => {
                self.current_function = Self::ignore;
                self.next_function = Self::attr_expr_1;
            }
            RIGHT_CURLY_BRACKET => {
                self.output_end();
                self.current_function = Self::ignore;
                self.next_function = Self::start;
            }
            _ => {}
        }
    }

    fn attr_expr_1(&mut self) {
        let c = self.c;
        match c {
            'A'..='Z' | 'a'..='z' | HYPHEN => {
                self.current_function = Self::attr_expr_2;
                self.attr_expr_2();
            }
            COLON => {
                self.current_function = Self::ignore;
                self.next_function = Self::attr_expr_1_0;
            }
            SEMICOLON => {
                self.current_function = Self::ignore;
                self.next_function = Self::attr_expr_1;
            }
            RIGHT_CURLY_BRACKET => {
                self.current_function = Self::attr_expr_2;
                self.attr_expr_2();
            }
            _ => {
                self.output_error("illegal char");
                self.current_function = Self::attr_expr_1_0;
                self.attr_expr_1_0();
            }
        }
    }

    fn attr_expr_1_0(&mut self) {
        match self.c {
            SEMICOLON | RIGHT_CURLY_BRACKET => {
                self.current_function = Self::attr_expr_1;
                self.attr_expr_1();
            }
            _ => {}
        }
    }

    fn attr_expr_2(&mut self) {
        let c = self.c;
        match c {
            'A'..='Z' | 'a'..='z' | HYPHEN => {
                self.temporary.0.push(c);
            }
            SPACE => {
                self.current_function = Self::ignore;
                self.next_function = Self::attr_expr_3;
            }
            COLON => {
                self.current_function = Self::ignore;
                self.next_function = Self::attr_expr_4;
            }
            SEMICOLON => {
                self.output_attr();
                self.current_function = Self::ignore;
                self.next_function = Self::attr_expr_1;
            }
            RIGHT_CURLY_BRACKET => {
                self.output_end();
                self.current_function = Self::ignore;
                self.next_function = Self::start;
            }
            _ => {
                self.output_error("illegal char");
                self.current_function = Self::attr_expr_1_0;
                self.attr_expr_1_0();
            }
        }
    }

    fn attr_expr_3(&mut self) {
        let c = self.c;
        match c {
            COLON => {
                self.current_function = Self::ignore;
                self.next_function = Self::attr_expr_4;
            }
            SEMICOLON | RIGHT_CURLY_BRACKET => {
                self.current_function = Self::attr_expr_2;
                self.attr_expr_2();
            }
            _ => {
                self.output_error("illegal char");
                self.current_function = Self::attr_expr_1_0;
                self.attr_expr_1_0();
            }
        }
    }

    fn attr_expr_4(&mut self) {
        let c = self.c;
        match c {
            APOSTROPHE => {
                self.current_function = Self::attr_expr_6;
            }
            QUOTATION => {
                self.current_function = Self::attr_expr_7;
            }
            SEMICOLON | RIGHT_CURLY_BRACKET => {
                self.current_function = Self::attr_expr_2;
                self.attr_expr_2();
            }
            _ => {
                self.current_function = Self::attr_expr_5;
                self.attr_expr_5();
            }
        }
    }

    fn attr_expr_5(&mut self) {
        let c = self.c;
        match c {
            SEMICOLON | RIGHT_CURLY_BRACKET => {
                self.current_function = Self::attr_expr_2;
                self.attr_expr_2();
            }
            _ => {
                self.temporary.1.push(c);
            }
        }
    }

    fn attr_expr_6(&mut self) {
        let c = self.c;
        match c {
            APOSTROPHE => {
                self.current_function = Self::attr_expr_8;
            }
            _ => {
                self.temporary.1.push(c);
            }
        }
    }

    fn attr_expr_7(&mut self) {
        let c = self.c;
        match c {
            QUOTATION => {
                self.current_function = Self::attr_expr_8;
            }
            _ => {
                self.temporary.1.push(c);
            }
        }
    }

    fn attr_expr_8(&mut self) {
        let c = self.c;
        match c {
            SEMICOLON | RIGHT_CURLY_BRACKET => {
                self.current_function = Self::attr_expr_2;
                self.attr_expr_2();
            }
            _ => {}
        }
    }
}
