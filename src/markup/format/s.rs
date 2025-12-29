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
    temporary: (String, String),
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
            c: NULL,
            counter: Default::default(),
            temporary: (String::new(), String::new()),
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
        self.c = NULL;
        (self.current_function)(&mut self);
        self.output
    }

    pub(super) fn parse_str(self, s: &str) -> T {
        self.parse(s.chars())
    }

    fn output_child(&mut self) {
        self.output.child();
    }

    fn output_colon(&mut self) {
        if self.temporary.0.is_empty() {
            self.output.root(1);
        } else {
            match to_usize(self.temporary.0.drain(..).as_str()) {
                Ok(n) => {
                    self.output.root(n);
                }
                Err(e) => {
                    error!("output_colon: {e}");
                }
            };
        }
    }

    fn output_circumflex_accent(&mut self) {
        if self.temporary.0.is_empty() {
            self.output.upper(1);
        } else {
            match to_usize(self.temporary.0.drain(..).as_str()) {
                Ok(n) => {
                    self.output.upper(n);
                }
                Err(e) => {
                    error!("output_circumflex_accent: {e}");
                }
            };
        }
    }

    fn output_tag(&mut self) {
        if !self.temporary.0.is_empty() {
            let s = self.temporary.0.drain(..).as_str().to_lowercase();
            self.output.tag(s);
        }
    }

    fn output_text(&mut self) {
        let s = self.temporary.0.drain(..).as_str().to_string();
        if !s.is_empty() {
            self.output.text(s);
        }
    }

    fn output_attribute(&mut self) {
        let k = self.temporary.0.drain(..).as_str().to_lowercase();
        let v = self.temporary.1.drain(..).as_str().to_string();
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
        match self.c {
            SPACE => {}
            _ => {
                self.current_function = self.next_function;
                (self.next_function)(self);
            }
        }
    }

    fn escaping(&mut self) {
        let c = self.c;
        self.temporary_escape.push(c);
        if c == SEMICOLON {
            match self.temporary_escape.drain(..).as_str() {
                "&amp;" => {
                    self.temporary.1.push(AMPERSAND);
                }
                "&apos;" => {
                    self.temporary.1.push(APOSTROPHE);
                }
                "&gt;" => {
                    self.temporary.1.push(GT);
                }
                "&lt;" => {
                    self.temporary.1.push(LT);
                }
                "&nbsp;" => {
                    self.temporary.1.push(SPACE);
                }
                "&quot;" => {
                    self.temporary.1.push(QUOTATION);
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
        match self.c {
            LT => {
                self.current_function = Self::tag_1;
            }
            _ => {}
        }
    }

    fn tag_1(&mut self) {
        let c = self.c;
        match c {
            GT => {
                self.current_function = Self::tag_0_1;
            }
            SPACE => {
                self.output_child();
                self.current_function = Self::series_space;
                self.next_function = Self::tag_1_1;
            }
            CIRCUMFLEX_ACCENT => {
                self.current_function = Self::tag_3;
            }
            COLON => {
                self.current_function = Self::tag_4;
            }
            LF | CR => {
                self.output_error("illegal char");
            }
            _ => {
                self.current_function = Self::tag_2;
                self.tag_2();
            }
        }
    }

    fn tag_1_1(&mut self) {
        match self.c {
            LF | CR => {
                self.output_error("illegal char");
                self.current_function = Self::ignore;
                self.next_function = Self::tag_2;
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
                self.temporary.0.push(c);
            }
            GT => {
                self.output_tag();
                self.current_function = Self::ignore;
                self.next_function = Self::text_0;
            }
            SPACE => {
                self.output_tag();
                self.current_function = Self::ignore;
                self.next_function = Self::tag_5;
            }
            LF | CR => {
                self.output_tag();
                self.current_function = Self::tag_6;
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
                self.temporary.0.push(c);
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

    fn tag_4(&mut self) {
        let c = self.c;
        match c {
            '0'..='9' => {
                self.temporary.0.push(c);
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
            GT => {
                self.output_child();
                self.current_function = Self::ignore;
                self.next_function = Self::text_0;
            }
            _ => {
                self.current_function = Self::attribute_0;
                self.attribute_0();
            }
        }
    }

    fn tag_6(&mut self) {
        match self.c {
            SPACE => {
                self.current_function = Self::ignore;
                self.next_function = Self::tag_5;
            }
            LF | CR => {}
            GT => {
                self.current_function = Self::ignore;
                self.next_function = Self::text_0;
            }
            _ => {
                self.current_function = Self::attribute_0;
                self.attribute_0();
            }
        }
    }

    fn attribute_0(&mut self) {
        let c = self.c;
        match c {
            GT => {
                self.current_function = Self::ignore;
                self.next_function = Self::text_0;
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
                self.current_function = Self::ignore;
                self.next_function = Self::text_0;
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
                self.current_function = Self::ignore;
                self.next_function = Self::attribute_2;
            }
            SPACE => {
                self.current_function = Self::ignore;
                self.next_function = Self::attribute_1_1;
            }
            GT => {
                self.current_function = Self::attribute_3;
                self.attribute_3();
            }
            _ => {
                self.temporary.0.push(c);
            }
        }
    }

    fn attribute_1_1(&mut self) {
        match self.c {
            EQUAL | GT => {
                self.current_function = Self::attribute_1;
                self.attribute_1();
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
                self.current_function = Self::ignore;
                self.next_function = Self::text_0;
            }
            AMPERSAND => {
                self.current_function = Self::escaping;
                self.next_function = Self::attribute_3;
                self.escaping();
            }
            _ => {
                self.temporary.1.push(c);
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
                self.temporary.1.push(c);
            }
        }
    }

    fn attribute_5(&mut self) {
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
                self.temporary.1.push(c);
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
            SPACE | LF | CR => {
                self.current_function = Self::text_1;
                self.text_1();
            }
            AMPERSAND => {
                self.current_function = Self::escaping;
                self.next_function = Self::text_2;
                self.escaping();
            }
            NULL => {
                self.output_text();
            }
            _ => {
                self.temporary.0.push(c);
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

    fn text_1(&mut self) {
        let c = self.c;
        match c {
            SPACE | LF | CR => {
                self.temporary.1.push(c);
            }
            LT | GT => {
                self.temporary.1.clear();
                self.current_function = Self::text_0;
                self.text_0();
            }
            NULL => {
                self.temporary.1.clear();
                self.text_0();
            }
            _ => {
                self.text_2();
            }
        }
    }

    fn text_2(&mut self) {
        self.temporary
            .0
            .push_str(self.temporary.1.drain(..).as_str());
        self.current_function = Self::text_0;
        self.text_0();
    }
}
