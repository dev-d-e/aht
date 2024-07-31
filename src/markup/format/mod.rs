mod x;

use self::x::{Context, XParser};
use super::{Attribute, Mark, TypeEntity, ValidElement};
use crate::utils::ascii::*;
use std::collections::{HashMap, VecDeque};

#[inline]
fn is_crlf(c: char) -> bool {
    c == CR || c == LF
}

#[derive(Debug)]
struct UnclearElement {
    key: String,
    text: String,
    attribute: HashMap<String, Option<String>>,
    subset: VecDeque<UnclearElement>,
}

impl UnclearElement {
    fn new(key: String) -> Self {
        UnclearElement {
            key,
            text: String::new(),
            attribute: HashMap::new(),
            subset: VecDeque::new(),
        }
    }

    fn empty() -> Self {
        Self::new(String::new())
    }

    fn to_valid(self) -> Option<ValidElement> {
        match Mark::from(self.key) {
            Ok(k) => {
                let mut e = ValidElement::new(k, self.text);
                let mut attribute = self.attribute;
                for (k, s) in attribute.drain() {
                    match Attribute::from(&k, s) {
                        Ok(a) => {
                            e.attribute.push(a);
                        }
                        Err(_) => {}
                    }
                }
                let mut subset = self.subset;
                for o in subset.drain(..) {
                    if let Some(t) = o.to_valid() {
                        e.subset.push(t);
                    }
                }
                Some(e)
            }
            Err(_) => None,
        }
    }

    fn push_text(&mut self, step: usize, s: &str) {
        if s.is_empty() {
            return;
        }
        if step == 0 {
            self.text.push_str(s);
        } else if let Some(o) = self.subset.back_mut() {
            o.push_text(step - 1, s);
        }
    }

    fn add_attribute(&mut self, k: String, s: Option<String>) -> bool {
        if self.attribute.contains_key(&k) {
            false
        } else {
            self.attribute.insert(k, s);
            true
        }
    }

    fn add_subset(&mut self, step: usize, n: UnclearElement) -> Option<*mut UnclearElement> {
        if step == 0 {
            self.subset.push_back(n);
            self.subset.back_mut().map(|p| p as *mut UnclearElement)
        } else if let Some(o) = self.subset.back_mut() {
            o.add_subset(step - 1, n)
        } else {
            None
        }
    }
}

struct Builder {
    temporary: Vec<String>,
    rst: UnclearElement,
    last_one: Option<*mut UnclearElement>,
    last_step: usize,
}

impl Builder {
    pub(crate) fn new() -> Self {
        Builder {
            temporary: Vec::new(),
            rst: UnclearElement::empty(),
            last_one: None,
            last_step: 0,
        }
    }

    fn step(&mut self) -> usize {
        self.temporary.len().saturating_sub(1)
    }

    fn get_step(&mut self, s: String) -> Option<usize> {
        let mut n = self.step();
        while let Some(k) = self.temporary.get(n) {
            if k == &s {
                self.temporary.drain(n..);
                return Some(n);
            }
            if n > 0 {
                n -= 1;
            } else {
                break;
            }
        }
        None
    }
}

impl XParser for Builder {
    fn start_tag(&mut self, s: String) {
        if s.is_empty() {
            return;
        }
        self.temporary.push(s.clone());
        let o = UnclearElement::new(s);
        let step = self.step();
        if step == self.last_step + 1 {
            if let Some(p) = self.last_one {
                unsafe {
                    self.last_one = (*p).add_subset(0, o);
                    self.last_step = step;
                    return;
                }
            }
        }
        self.last_one = self.rst.add_subset(step, o);
        self.last_step = step;
    }

    fn end_tag(&mut self, s: String) {
        if let Some(_) = self.get_step(s) {}
    }

    fn end_slash(&mut self) {
        self.temporary.pop();
    }

    fn tag_text(&mut self, s: String) {
        if s.is_empty() {
            return;
        }
        let step = self.step();
        if step == self.last_step {
            if let Some(p) = self.last_one {
                unsafe {
                    (*p).push_text(0, &s);
                    return;
                }
            }
        }
        self.rst.push_text(step + 1, &s);
    }

    fn attribute(&mut self, k: String, s: Option<String>) {
        if let Some(p) = self.last_one {
            unsafe {
                (*p).add_attribute(k, s);
            }
        }
    }

    fn error(&mut self, n: usize) {
        println!("error {}", n);
    }
}

pub(super) fn accept(buf: &str) -> VecDeque<TypeEntity> {
    let mut parser = Builder::new();
    let mut context = Context::new(&mut parser);
    x::accept(&mut context, &buf);
    let v = parser.rst.subset.drain(..);
    v.filter_map(|o| o.to_valid())
        .map(|o| TypeEntity::from(o))
        .collect()
}
