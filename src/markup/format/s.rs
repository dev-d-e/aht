use super::*;

pub(super) trait Output {
    fn child(&mut self);

    fn root(&mut self, n: usize);

    fn upper(&mut self, n: usize);

    fn tag(&mut self, s: String);

    fn attribute(&mut self, k: String, v: String);

    fn text(&mut self, s: String);

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
    temporary: String,
    temporary_attr: (String, String),
    temporary_escape: String,
    output: T,
}

impl<T> Parser<T>
where
    T: Output,
{
    pub(super) fn new(output: T) -> Self {
        Self {
            current_function: Self::ignore,
            next_function: Self::tag_0,
            c: 0 as char,
            counter: Default::default(),
            temporary: String::new(),
            temporary_attr: (String::new(), String::new()),
            temporary_escape: String::new(),
            output,
        }
    }

    pub(super) fn parse(mut self, mut i: impl Iterator<Item = char>) -> T {
        while let Some(c) = i.next() {
            self.c = c;
            (self.current_function)(&mut self);
            self.counter.count(c);
        }
        self.output_text();
        self.output
    }

    pub(super) fn parse_str(self, s: &str) -> T {
        self.parse(s.chars())
    }

    fn output_child(&mut self) {
        self.output.child();
    }

    fn output_colon(&mut self) {
        if self.temporary.is_empty() {
            self.output.root(1);
        } else {
            match to_usize(self.temporary.drain(..).as_str()) {
                Ok(n) => {
                    self.output.root(n);
                }
                Err(e) => {
                    error!("<{e}");
                }
            };
        }
    }

    fn output_circumflex_accent(&mut self) {
        if self.temporary.is_empty() {
            self.output.upper(1);
        } else {
            match to_usize(self.temporary.drain(..).as_str()) {
                Ok(n) => {
                    self.output.upper(n);
                }
                Err(e) => {
                    error!("<{e}");
                }
            };
        }
    }

    fn output_tag(&mut self) {
        if !self.temporary.is_empty() {
            let s = self.temporary.drain(..).as_str().to_string();
            self.output.tag(s);
        }
    }

    fn output_text(&mut self) {
        let s = self.temporary.drain(..).as_str().trim().to_string();
        if !s.is_empty() {
            self.output.text(s);
        }
    }

    fn output_attribute(&mut self) {
        let k = self.temporary_attr.0.drain(..).as_str().to_lowercase();
        let v = self.temporary_attr.1.drain(..).as_str().to_string();
        if !k.is_empty() {
            self.output.attribute(k, v);
        }
    }

    fn output_error(&mut self, s: &str) {
        let e = self.counter.to_error(ErrorKind::Markup, s);
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

    fn series_space(&mut self) {
        let c = self.c;
        if c == SPACE {
        } else {
            self.current_function = self.next_function;
            (self.next_function)(self);
        }
    }

    fn escaping(&mut self) {
        let c = self.c;
        self.temporary_escape.push(c);
        if c == SEMICOLON {
            match self.temporary_escape.drain(..).as_str() {
                "&amp;" => {
                    self.temporary_attr.1.push(AMPERSAND);
                }
                "&apos;" => {
                    self.temporary_attr.1.push(APOSTROPHE);
                }
                "&gt;" => {
                    self.temporary_attr.1.push(GT);
                }
                "&lt;" => {
                    self.temporary_attr.1.push(LT);
                }
                "&nbsp;" => {
                    self.temporary_attr.1.push(SPACE);
                }
                "&quot;" => {
                    self.temporary_attr.1.push(QUOTATION);
                }
                _ => {}
            }
            self.current_function = self.next_function;
        }
    }

    fn tag_0(&mut self) {
        let c = self.c;
        match c {
            LT => {
                self.current_function = Self::tag_1;
            }
            _ => {
                self.output_error("illegal char");
                self.current_function = Self::tag_0_1;
            }
        }
    }

    fn tag_0_1(&mut self) {
        let c = self.c;
        match c {
            LT => {
                self.current_function = Self::ignore;
                self.next_function = Self::tag_1;
            }
            _ => {}
        }
    }

    fn tag_1(&mut self) {
        let c = self.c;
        match c {
            GT => {
                self.tag_5();
            }
            SPACE => {
                self.output_child();
                self.current_function = Self::series_space;
                self.next_function = Self::tag_2;
            }
            CIRCUMFLEX_ACCENT => {
                self.current_function = Self::tag_3;
            }
            COLON => {
                self.current_function = Self::tag_4;
            }
            _ => {
                self.current_function = Self::tag_2;
                self.tag_2();
            }
        }
    }

    fn tag_2(&mut self) {
        let c = self.c;
        match c {
            'A'..='Z' | 'a'..='z' => {
                self.temporary.push(c);
            }
            GT => {
                self.output_tag();
                self.current_function = Self::text_0;
            }
            SPACE | LF | CR => {
                self.output_tag();
                self.current_function = Self::ignore;
                self.next_function = Self::attribute_0;
            }
            _ => {
                self.output_error("invalid mark");
                self.current_function = Self::tag_2_1;
            }
        }
    }

    fn tag_2_1(&mut self) {
        match self.c {
            GT | SPACE | LF | CR => {
                self.current_function = Self::tag_2;
                self.tag_2();
            }
            _ => {}
        }
    }

    fn tag_3(&mut self) {
        let c = self.c;
        match c {
            '0'..='9' => {
                self.temporary.push(c);
            }
            SPACE | LF | CR => {
                self.output_circumflex_accent();
                self.current_function = Self::ignore;
                self.next_function = Self::tag_2;
            }
            _ => {
                self.output_circumflex_accent();
                self.current_function = Self::tag_2;
                self.tag_2();
            }
        }
    }

    fn tag_3_1(&mut self) {
        match self.c {
            SPACE | LF | CR => {
                self.current_function = Self::ignore;
                self.next_function = Self::tag_2;
            }
            LT => {
                self.current_function = Self::tag_0;
                self.tag_0();
            }
            _ => {}
        }
    }

    fn tag_4(&mut self) {
        let c = self.c;
        match c {
            '0'..='9' => {
                self.temporary.push(c);
            }
            SPACE | LF | CR => {
                self.output_colon();
                self.current_function = Self::ignore;
                self.next_function = Self::tag_2;
            }
            _ => {
                self.output_colon();
                self.current_function = Self::tag_2;
                self.tag_2();
            }
        }
    }

    fn tag_5(&mut self) {
        match self.c {
            LT => {
                self.current_function = Self::tag_0;
                self.tag_0();
            }
            _ => {}
        }
    }

    fn attribute_0(&mut self) {
        let c = self.c;
        match c {
            GT => {
                self.current_function = Self::text_0;
            }
            EQUAL => {
                self.output_error("no attribute name");
                self.current_function = Self::attribute_0_1;
            }
            _ => {
                self.current_function = Self::attribute_1;
                self.attribute_1();
            }
        }
    }

    fn attribute_0_1(&mut self) {
        let c = self.c;
        match c {
            GT => {
                self.current_function = Self::text_0;
            }
            SPACE => {
                self.current_function = Self::ignore;
                self.next_function = Self::attribute_0;
            }
            _ => {}
        }
    }

    fn attribute_1(&mut self) {
        let c = self.c;
        match c {
            EQUAL => {
                self.current_function = Self::series_space;
                self.next_function = Self::attribute_2;
            }
            SPACE => {
                self.current_function = Self::series_space;
                self.next_function = Self::attribute_1_1;
            }
            GT => {
                self.current_function = Self::attribute_3;
                self.attribute_3();
            }
            _ => {
                self.temporary_attr.0.push(c);
            }
        }
    }

    fn attribute_1_1(&mut self) {
        let c = self.c;
        match c {
            EQUAL => {
                self.current_function = Self::series_space;
                self.next_function = Self::attribute_2;
            }
            GT => {
                self.current_function = Self::attribute_3;
                self.attribute_3();
            }
            _ => {
                self.output_attribute();
                self.current_function = Self::attribute_0;
                self.attribute_0();
            }
        }
    }

    fn attribute_2(&mut self) {
        let c = self.c;
        match c {
            QUOTATION => {
                self.current_function = Self::attribute_4;
            }
            APOSTROPHE => {
                self.current_function = Self::attribute_5;
            }
            _ => {
                self.current_function = Self::attribute_3;
                self.attribute_3();
            }
        }
    }

    fn attribute_3(&mut self) {
        let c = self.c;
        match c {
            SPACE => {
                self.output_attribute();
                self.current_function = Self::ignore;
                self.next_function = Self::attribute_0;
            }
            GT => {
                self.output_attribute();
                self.current_function = Self::text_0;
            }
            AMPERSAND => {
                self.current_function = Self::escaping;
                self.next_function = Self::attribute_3;
                self.escaping();
            }
            _ => {
                self.temporary_attr.1.push(c);
            }
        }
    }

    fn attribute_4(&mut self) {
        let c = self.c;
        match c {
            QUOTATION => {
                self.output_attribute();
                self.current_function = Self::ignore;
                self.next_function = Self::attribute_0;
            }
            AMPERSAND => {
                self.current_function = Self::escaping;
                self.next_function = Self::attribute_4;
                self.escaping();
            }
            _ => {
                self.temporary_attr.1.push(c);
            }
        }
    }

    fn attribute_5(&mut self) {
        let c = self.c;
        let c = self.c;
        match c {
            APOSTROPHE => {
                self.output_attribute();
                self.current_function = Self::ignore;
                self.next_function = Self::attribute_0;
            }
            AMPERSAND => {
                self.current_function = Self::escaping;
                self.next_function = Self::attribute_5;
                self.escaping();
            }
            _ => {
                self.temporary_attr.1.push(c);
            }
        }
    }

    fn text_0(&mut self) {
        let c = self.c;
        match c {
            LT => {
                self.output_text();
                self.current_function = Self::tag_0;
                self.tag_0();
            }
            GT => {
                self.output_text();
                self.current_function = Self::text_0_1;
                self.output_error("illegal char");
            }
            AMPERSAND => {
                self.current_function = Self::escaping;
                self.next_function = Self::attribute_5;
                self.escaping();
            }
            _ => {
                self.temporary.push(c);
            }
        }
    }

    fn text_0_1(&mut self) {
        match self.c {
            LT => {
                self.current_function = Self::tag_0;
                self.tag_0();
            }
            _ => {}
        }
    }
}
