use super::*;
use crate::utils::ascii::*;

pub(super) trait Output {
    fn at(&mut self, s: String);

    fn mark_selector(&mut self, s: String);

    fn attribute_selector(&mut self, k: String, v: String);

    fn start_block(&mut self);

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
            c: 0 as char,
            counter: Default::default(),
            temporary: (String::new(), String::new()),
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
            self.output.at(s);
        }
    }

    fn output_m_selector(&mut self) {
        if self.temporary.0.is_empty() {
            self.temporary.1.clear();
        } else {
            let s = self.temporary.0.drain(..).as_str().to_lowercase();
            self.output.mark_selector(s);
        }
    }

    fn output_a_selector(&mut self) {
        if self.temporary.1.is_empty() {
            self.temporary.0.clear();
        } else {
            let n = self.temporary.0.drain(..).as_str().to_lowercase();
            let v = self.temporary.1.drain(..).as_str().to_string();
            self.output.attribute_selector(n, v);
        }
    }

    fn output_attr(&mut self) {
        if self.temporary.0.is_empty() {
            self.temporary.1.clear();
        } else {
            let n = self.temporary.0.drain(..).as_str().to_lowercase();
            let v = self.temporary.1.drain(..).as_str().to_string();
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
            _ => {
                self.output_error("illegal char");
                self.current_function = Self::ignore;
                self.next_function = Self::start;
            }
        }
    }

    fn at_0(&mut self) {
        let c = self.c;
        match c {
            'A'..='Z' | 'a'..='z' | HYPHEN => {
                self.current_function = Self::at_1;
                self.at_1();
            }
            _ => {
                self.output_error("illegal char");
                self.current_function = Self::ignore;
                self.next_function = Self::at_0_1;
            }
        }
    }

    fn at_1(&mut self) {
        let c = self.c;
        match c {
            'A'..='Z' | 'a'..='z' | HYPHEN => {
                self.temporary.0.push(c);
            }
            SPACE => {
                self.output_at();
                self.current_function = Self::ignore;
                self.next_function = Self::at_2;
            }
            _ => {
                self.current_function = Self::at_2;
                self.at_2();
            }
        }
    }

    fn at_2(&mut self) {
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
                self.next_function = Self::at_0_1;
            }
        }
    }

    fn at_0_1(&mut self) {
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
                self.next_function = Self::start;
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
                self.next_function = Self::start;
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
                self.next_function = Self::start;
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
                self.current_function = Self::ignore;
                self.next_function = Self::start;
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
