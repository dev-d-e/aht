use super::*;
use getset::{CopyGetters, Setters};
use std::sync::{Arc, RwLock};

///Represents requirement of searching elements.
pub trait Requirement {
    fn satisfy(&self, e: &Element) -> bool;
}

///Represents searching elements in descendant.
#[derive(Debug, CopyGetters, Setters)]
pub struct FindDescendant<T>
where
    T: Requirement,
{
    f: T,
    #[getset(get_copy = "pub", set = "pub")]
    n: usize,
    i: usize,
}

impl<T> FindDescendant<T>
where
    T: Requirement,
{
    ///Creates a new instance. `n` is nesting number, searches all descendant if it's 0.
    pub fn new(f: T, n: usize) -> Self {
        Self { f, n, i: n }
    }

    fn all(&self, o: &Arc<RwLock<Element>>, v: &mut Vec<Arc<RwLock<Element>>>) {
        if let Ok(e) = o.read() {
            if self.f.satisfy(&e) {
                v.push(o.clone())
            }
            e.subset().iter().for_each(|k| self.all(k, v));
        }
    }

    fn subtract(&mut self, o: &Arc<RwLock<Element>>, v: &mut Vec<Arc<RwLock<Element>>>) {
        self.i -= 1;
        if let Ok(e) = o.read() {
            if self.f.satisfy(&e) {
                v.push(o.clone())
            }
            if self.i > 0 {
                e.subset().iter().for_each(|k| self.subtract(k, v));
            }
        }
    }

    ///Gets elements that satisfy the requirements by an element to the vec.
    pub fn find(&mut self, o: &Arc<RwLock<Element>>, v: &mut Vec<Arc<RwLock<Element>>>) {
        if self.n == 0 {
            self.all(o, v)
        } else {
            self.i = self.n;
            self.subtract(o, v)
        }
    }

    ///Returns unique element that satisfy the requirements by an element.
    pub fn find_unique(&mut self, o: &Arc<RwLock<Element>>) -> Option<Arc<RwLock<Element>>> {
        if let Ok(e) = o.read() {
            if self.f.satisfy(&e) {
                return Some(o.clone());
            }
            return e.subset().iter().find_map(|k| self.find_unique(k));
        }
        None
    }
}

///Represents searching elements in next sibling.
#[derive(Debug, CopyGetters, Setters)]
pub struct FindNextSibling<T>
where
    T: Requirement,
{
    f: T,
    #[getset(get_copy = "pub", set = "pub")]
    n: usize,
}

impl<T> FindNextSibling<T>
where
    T: Requirement,
{
    ///Creates a new instance. `n` is range, searches all next sibling if it's 0.
    pub fn new(f: T, n: usize) -> Self {
        Self { f, n }
    }

    fn subset_end_index(&self, i: usize, subset_len: usize) -> usize {
        if self.n == 0 {
            subset_len
        } else {
            subset_len.min(i + self.n + 1)
        }
    }

    ///Gets elements that satisfy the requirements by an element to the vec.
    pub fn find(&mut self, o: &Arc<RwLock<Element>>, v: &mut Vec<Arc<RwLock<Element>>>) {
        let mut p = None;
        if let Ok(e) = o.read() {
            p = e.upper().as_ref().cloned();
        }
        if let Some(p) = p {
            if let Ok(e) = p.read() {
                if let Some(i) = e.subset_element_index(o) {
                    let n = self.subset_end_index(i, e.subset().len());
                    e.subset().range(i..n).for_each(|o| {
                        if let Ok(e) = o.read() {
                            if self.f.satisfy(&e) {
                                v.push(o.clone());
                            }
                        }
                    });
                }
            }
        }
    }

    ///Returns unique element that satisfy the requirements by an element.
    pub fn find_unique(&mut self, o: &Arc<RwLock<Element>>) -> Option<Arc<RwLock<Element>>> {
        let mut p = None;
        if let Ok(e) = o.read() {
            p = e.upper().as_ref().cloned();
        }
        if let Some(p) = p {
            if let Ok(e) = p.read() {
                if let Some(i) = e.subset_element_index(o) {
                    let n = self.subset_end_index(i, e.subset().len());
                    return e.subset().range(i..n).find_map(|o| {
                        if let Ok(e) = o.read() {
                            if self.f.satisfy(&e) {
                                return Some(o.clone());
                            }
                        }
                        None
                    });
                }
            }
        }
        None
    }
}

///Represents searching elements in preceding sibling.
#[derive(Debug, CopyGetters, Setters)]
pub struct FindPrecedingSibling<T>
where
    T: Requirement,
{
    f: T,
    #[getset(get_copy = "pub", set = "pub")]
    n: usize,
    i: usize,
}

impl<T> FindPrecedingSibling<T>
where
    T: Requirement,
{
    ///Creates a new instance. `n` is range, searches all preceding sibling if it's 0.
    pub fn new(f: T, n: usize) -> Self {
        Self { f, n, i: n }
    }

    fn subset_start_index(&self, i: usize) -> usize {
        if self.n == 0 {
            0
        } else {
            i.saturating_sub(self.n)
        }
    }

    ///Gets elements that satisfy the requirements by an element to the vec.
    pub fn find(&mut self, o: &Arc<RwLock<Element>>, v: &mut Vec<Arc<RwLock<Element>>>) {
        let mut p = None;
        if let Ok(e) = o.read() {
            p = e.upper().as_ref().cloned();
        }
        if let Some(p) = p {
            if let Ok(e) = p.read() {
                if let Some(i) = e.subset_element_index(o) {
                    let n = self.subset_start_index(i);
                    e.subset().range(n..=i).for_each(|o| {
                        if let Ok(e) = o.read() {
                            if self.f.satisfy(&e) {
                                v.push(o.clone());
                            }
                        }
                    })
                }
            }
        }
    }

    ///Returns unique element that satisfy the requirements by an element.
    pub fn find_unique(&mut self, o: &Arc<RwLock<Element>>) -> Option<Arc<RwLock<Element>>> {
        let mut p = None;
        if let Ok(e) = o.read() {
            p = e.upper().as_ref().cloned();
        }
        if let Some(p) = p {
            if let Ok(e) = p.read() {
                if let Some(i) = e.subset_element_index(o) {
                    let n = self.subset_start_index(i);
                    return e.subset().range(n..=i).find_map(|o| {
                        if let Ok(e) = o.read() {
                            if self.f.satisfy(&e) {
                                return Some(o.clone());
                            }
                        }
                        None
                    });
                }
            }
        }
        None
    }
}

///Represents whether element's mark equals the mark.
pub struct MarkEq<'a> {
    m: &'a Mark,
}

impl<'a> MarkEq<'a> {
    ///Creates a new instance.
    pub fn new(m: &'a Mark) -> Self {
        Self { m }
    }
}

impl<'a> Requirement for MarkEq<'a> {
    fn satisfy(&self, e: &Element) -> bool {
        e.mark_type() == self.m
    }
}

///Represents whether element's attribute exists.
pub struct AttrExists<'a> {
    n: &'a AttrName,
}

impl<'a> AttrExists<'a> {
    ///Creates a new instance.
    pub fn new(n: &'a AttrName) -> Self {
        Self { n }
    }
}

impl<'a> Requirement for AttrExists<'a> {
    fn satisfy(&self, e: &Element) -> bool {
        e.attribute().contains_key(self.n)
    }
}

///Represents whether element's attribute matches the pattern.
pub struct AttrMatch<'a> {
    n: &'a AttrName,
    p: &'a AttrPattern,
}

impl<'a> AttrMatch<'a> {
    ///Creates a new instance.
    pub fn new(n: &'a AttrName, p: &'a AttrPattern) -> Self {
        Self { n, p }
    }
}

impl<'a> Requirement for AttrMatch<'a> {
    fn satisfy(&self, e: &Element) -> bool {
        e.attribute_get(self.n)
            .map(|a| a.matches(self.p))
            .unwrap_or(false)
    }
}

///Represents match any element.
pub struct AnyMatch;

impl Requirement for AnyMatch {
    fn satisfy(&self, _: &Element) -> bool {
        true
    }
}
