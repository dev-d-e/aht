use super::*;
use std::sync::{Arc, RwLock};

///Represents find condition.
#[derive(Debug)]
pub enum Condition<'a> {
    MARK(Mark),
    CLASS(&'a str),
    ID(&'a str),
}

///Represents find conditions.
#[derive(Debug, Default)]
pub struct Conditions<'a>(Vec<Condition<'a>>);

impl<'a> Conditions<'a> {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn mark(&mut self, s: Mark) {
        self.0.push(Condition::MARK(s));
    }

    pub fn class(&mut self, s: &'a str) {
        self.0.push(Condition::CLASS(s))
    }

    pub fn id(&mut self, s: &'a str) {
        self.0.push(Condition::ID(s))
    }

    pub fn reverse(&mut self) {
        self.0.reverse()
    }
}

impl<'a> From<Condition<'a>> for Conditions<'a> {
    fn from(c: Condition<'a>) -> Self {
        Self(vec![c])
    }
}

impl<'a> From<Mark> for Conditions<'a> {
    fn from(o: Mark) -> Self {
        Self::from(Condition::MARK(o))
    }
}

pub(crate) fn find_elements(
    mut v: Vec<Arc<RwLock<Element>>>,
    conditions: Conditions,
) -> Vec<Arc<RwLock<Element>>> {
    for c in conditions.0 {
        let mut r = Vec::new();
        match c {
            Condition::MARK(s) => {
                v.into_iter().for_each(|k| find_mark(k, &s, &mut r));
            }
            Condition::CLASS(s) => {
                v.into_iter().for_each(|k| find_class(k, s, &mut r));
            }
            Condition::ID(s) => {
                for k in v {
                    if let Some(p) = find_id(k, s) {
                        r.push(p);
                        break;
                    }
                }
            }
        }
        v = r;
    }
    v
}

fn find_mark(o: Arc<RwLock<Element>>, s: &Mark, v: &mut Vec<Arc<RwLock<Element>>>) {
    if let Ok(e) = o.read() {
        if e.mark_type() == s {
            v.push(o.clone())
        }
        e.subset().iter().for_each(|k| find_mark(k.clone(), s, v))
    }
}

fn find_class(o: Arc<RwLock<Element>>, s: &str, v: &mut Vec<Arc<RwLock<Element>>>) {
    if let Ok(e) = o.read() {
        if let Some(c) = e.attribute().class() {
            if c == s {
                v.push(o.clone());
            }
        }
        e.subset().iter().for_each(|k| find_class(k.clone(), s, v))
    }
}

fn find_id(o: Arc<RwLock<Element>>, s: &str) -> Option<Arc<RwLock<Element>>> {
    if let Ok(e) = o.read() {
        if let Some(i) = e.attribute().id() {
            if i == s {
                return Some(o.clone());
            }
        }
        for k in e.subset().iter() {
            let t = find_id(k.clone(), s);
            if t.is_some() {
                return t;
            }
        }
    }
    None
}
