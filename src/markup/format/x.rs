use super::*;

pub(super) trait XParser {
    fn start_tag(&mut self, s: String);

    fn end_tag(&mut self, s: String);

    fn end_slash(&mut self);

    fn tag_text(&mut self, s: String);

    fn attribute(&mut self, k: String, v: Option<String>);

    fn error(&mut self, n: usize);
}

pub(super) struct Context<'a> {
    current_function: fn(&mut Context),
    next_function: fn(&mut Context),
    c: char,
    n: usize,
    temporary: String,
    temporary_attr: (String, String),
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
            parser: Box::new(parser),
        }
    }

    fn attribute(&mut self) {
        let k = self.temporary_attr.0.drain(..);
        let k = k.as_str();
        if k.is_empty() {
            return;
        }
        let k = k.to_string();
        let v = self.temporary_attr.1.drain(..);
        let v = v.as_str();
        if v.is_empty() {
            self.parser.attribute(k, None);
        } else {
            self.parser.attribute(k, Some(v.to_string()));
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
        context.next_function = tag_4;
    } else if c == GT {
        context.current_function = tag_0;
    } else {
        context.current_function = tag_2;
        tag_2(context);
    }
}

fn tag_2(context: &mut Context) {
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
        context.next_function = tag_3;
    } else if c == SPACE {
        let t = context.temporary.drain(..).collect();
        context.parser.start_tag(t);
        context.current_function = series_space;
        context.next_function = attribute_0;
    } else {
        context.temporary.push(c);
    }
}

fn tag_3(context: &mut Context) {
    let c = context.c;
    if c == GT {
        context.current_function = tag_0;
    } else {
        context.parser.error(context.n);
    }
}

fn tag_4(context: &mut Context) {
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

fn attribute_0(context: &mut Context) {
    let c = context.c;
    if c == GT {
        context.current_function = tag_0;
    } else if c == SLASH {
        context.parser.end_slash();
    } else {
        context.current_function = attribute_1;
        attribute_1(context);
    }
}

fn attribute_1(context: &mut Context) {
    let c = context.c;
    if c == GT {
        context.attribute();
        context.current_function = tag_0;
    } else if c == SPACE {
        context.attribute();
        context.current_function = series_space;
        context.next_function = attribute_2;
    } else if c == EQUAL {
        context.current_function = series_space;
        context.next_function = attribute_3;
    } else {
        context.temporary_attr.0.push(c);
    }
}

fn attribute_2(context: &mut Context) {
    let c = context.c;
    if c == EQUAL {
        context.current_function = series_space;
        context.next_function = attribute_3;
    } else {
        context.current_function = attribute_1;
        attribute_1(context);
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
        context.next_function = attribute_0;
    } else if is_quotation(c) {
        context.current_function = tag_attribute_4;
    } else {
        context.temporary_attr.1.push(c);
    }
}

fn tag_attribute_4(context: &mut Context) {
    let c = context.c;
    if is_quotation(c) {
        context.attribute();
        context.current_function = series_space;
        context.next_function = attribute_0;
    } else {
        context.temporary_attr.1.push(c);
    }
}
