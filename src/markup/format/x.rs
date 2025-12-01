use super::*;

pub(super) trait XParser {
    fn start_tag(&mut self, s: String);

    fn end_tag(&mut self, s: String);

    fn end_slash(&mut self);

    fn tag_text(&mut self, s: String);

    fn attribute(&mut self, k: String, v: Option<String>);

    fn error(&mut self, e: Error);
}

pub(super) struct Context<'a> {
    current_function: fn(&mut Context),
    next_function: fn(&mut Context),
    c: char,
    n: usize,
    temporary: String,
    temporary_attr: (String, String),
    temporary_escape: String,
    parser: Box<&'a mut dyn XParser>,
}

impl<'a> Context<'a> {
    pub(super) fn new<T: XParser>(parser: &'a mut T) -> Self {
        Self {
            current_function: tag_0,
            next_function: tag_0,
            c: 0 as char,
            n: 0,
            temporary: String::new(),
            temporary_attr: (String::new(), String::new()),
            temporary_escape: String::new(),
            parser: Box::new(parser),
        }
    }

    fn attribute_k(&mut self) -> Option<String> {
        let k = self.temporary_attr.0.drain(..);
        let k = k.as_str();
        if k.is_empty() {
            None
        } else {
            Some(k.to_string())
        }
    }

    fn attribute_v(&mut self) -> Option<String> {
        let v = self.temporary_attr.1.drain(..);
        let v = v.as_str().trim();
        if v.is_empty() {
            None
        } else {
            Some(v.to_string())
        }
    }

    fn attribute(&mut self) {
        if let Some(k) = self.attribute_k() {
            let v = self.attribute_v();
            self.parser.attribute(k, v);
        }
    }

    fn attribute_push(&mut self) {
        if let Some(k) = self.attribute_k() {
            self.parser.attribute(k, None);
            if let Some(v) = self.attribute_v() {
                self.temporary_attr.0.push_str(&v);
            }
        }
    }
}

pub(super) fn accept(context: &mut Context, buf: &str) {
    let mut cs = buf.chars();
    while let Some(c) = cs.next() {
        context.c = c;
        (context.current_function)(context);
        context.n += 1;
    }
}

fn series_space(context: &mut Context) {
    let c = context.c;
    if c == SPACE {
    } else {
        context.current_function = context.next_function;
        (context.next_function)(context);
    }
}

fn escaping(context: &mut Context) {
    let c = context.c;
    context.temporary_escape.push(c);
    if c == SEMICOLON {
        match context.temporary_escape.drain(..).as_str() {
            "&amp;" => {
                context.temporary_attr.1.push(AMPERSAND);
            }
            "&apos;" => {
                context.temporary_attr.1.push(APOSTROPHE);
            }
            "&gt;" => {
                context.temporary_attr.1.push(GT);
            }
            "&lt;" => {
                context.temporary_attr.1.push(LT);
            }
            "&nbsp;" => {
                context.temporary_attr.1.push(SPACE);
            }
            "&quot;" => {
                context.temporary_attr.1.push(QUOTATION);
            }
            _ => {}
        }
        context.current_function = context.next_function;
    }
}

fn tag_0(context: &mut Context) {
    let c = context.c;
    if c == LT {
        context.current_function = series_space;
        context.next_function = tag_1;
        if !context.temporary.is_empty() {
            let s = context.temporary.drain(..).as_str().trim().to_string();
            context.parser.tag_text(s);
        }
    } else {
        context.temporary.push(c);
    }
}

fn tag_1(context: &mut Context) {
    let c = context.c;
    if c == SLASH {
        context.current_function = series_space;
        context.next_function = tag_2;
    } else if c == GT {
        context.current_function = tag_0;
    } else {
        context.current_function = tag_3;
        tag_3(context);
    }
}

fn tag_2(context: &mut Context) {
    let c = context.c;
    if c == GT {
        if !context.temporary.is_empty() {
            let t = context.temporary.drain(..).collect();
            context.parser.end_tag(t);
        }
        context.current_function = tag_0;
    } else {
        context.temporary.push(c);
    }
}

fn tag_3(context: &mut Context) {
    let c = context.c;
    if c == GT {
        let t = context.temporary.drain(..).collect();
        context.parser.start_tag(t);
        context.current_function = tag_0;
    } else if c == SLASH {
        let t = context.temporary.drain(..).collect();
        context.parser.start_tag(t);
        context.parser.end_slash();
        context.current_function = series_space;
        context.next_function = tag_4;
    } else if c == SPACE {
        let t = context.temporary.drain(..).collect();
        context.parser.start_tag(t);
        context.current_function = series_space;
        context.next_function = tag_5;
    } else {
        context.temporary.push(c);
    }
}

fn tag_4(context: &mut Context) {
    let c = context.c;
    if c == GT {
        context.current_function = tag_0;
    } else {
        let e = (ErrorKind::BrokenEnd, context.n, context.n).into();
        context.parser.error(e);
    }
}

fn tag_5(context: &mut Context) {
    let c = context.c;
    if c == GT {
        context.current_function = tag_0;
    } else if c == SLASH {
        context.parser.end_slash();
        context.current_function = series_space;
        context.next_function = tag_4;
    } else {
        context.current_function = attribute_0;
        attribute_0(context);
    }
}

fn attribute_0(context: &mut Context) {
    let c = context.c;
    if c == GT {
        context.attribute();
        context.current_function = tag_0;
    } else if c == SPACE {
        context.current_function = series_space;
        context.next_function = attribute_1;
    } else if c == EQUAL {
        attribute_1(context);
    } else {
        context.temporary_attr.0.push(c);
    }
}

fn attribute_1(context: &mut Context) {
    let c = context.c;
    if c == EQUAL {
        context.current_function = series_space;
        context.next_function = attribute_2;
    } else {
        context.attribute();
        context.current_function = attribute_0;
        attribute_0(context);
    }
}

fn attribute_2(context: &mut Context) {
    let c = context.c;
    if c == GT {
        context.attribute();
        context.current_function = tag_0;
    } else if c == SPACE {
        context.attribute();
        context.current_function = series_space;
        context.next_function = tag_5;
    } else if c == QUOTATION {
        context.current_function = attribute_4;
    } else if c == APOSTROPHE {
        context.current_function = attribute_5;
    } else if c == EQUAL {
        context.temporary_attr.1.push(c);
    } else {
        context.current_function = attribute_3;
        attribute_3(context);
    }
}

fn attribute_3(context: &mut Context) {
    let c = context.c;
    if c == GT {
        context.attribute();
        context.current_function = tag_0;
    } else if c == SPACE {
        context.attribute();
        context.current_function = series_space;
        context.next_function = tag_5;
    } else if c == EQUAL {
        context.attribute_push();
    } else {
        context.temporary_attr.1.push(c);
    }
}

fn attribute_4(context: &mut Context) {
    let c = context.c;
    if c == QUOTATION {
        context.attribute();
        context.current_function = series_space;
        context.next_function = tag_5;
    } else {
        attribute_6(context);
    }
}

fn attribute_5(context: &mut Context) {
    let c = context.c;
    if c == APOSTROPHE {
        context.attribute();
        context.current_function = series_space;
        context.next_function = tag_5;
    } else {
        attribute_6(context);
    }
}

fn attribute_6(context: &mut Context) {
    let c = context.c;
    if c == GT {
        let v = context.temporary_attr.1.trim_end();
        if v.ends_with(SLASH) {
            context.temporary_attr.1.truncate(v.len() - 1);
            context.attribute();
            context.parser.end_slash();
        } else {
            context.attribute();
        }
        context.current_function = tag_0;
    } else if c == AMPERSAND {
        context.next_function = context.current_function;
        context.current_function = escaping;
        escaping(context);
    } else {
        context.temporary_attr.1.push(c);
    }
}
