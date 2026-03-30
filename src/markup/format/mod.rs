mod s;
mod x;

use self::x::*;
use super::*;
use crate::utils::ascii::*;
use slotmap::{DefaultKey, SecondaryMap, SlotMap};
use std::collections::HashMap;

#[derive(Default)]
struct TempResult {
    data: SlotMap<DefaultKey, (String, String, HashMap<String, String>)>,
    root: Vec<DefaultKey>,
    subset: SecondaryMap<DefaultKey, Vec<DefaultKey>>,
}

impl std::fmt::Display for TempResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "TempResult {{ data: [")?;
        for o in &self.data {
            writeln!(f, "{:?}", o)?;
        }
        writeln!(f, "], subset: [")?;
        for o in &self.subset {
            writeln!(f, "{:?}", o)?;
        }
        write!(f, "] }}")
    }
}

impl TempResult {
    fn has_root(&self) -> bool {
        self.root.len() > 0
    }

    fn add_root(&mut self, e: (String, String, HashMap<String, String>)) -> DefaultKey {
        let key = self.data.insert(e);
        self.root.push(key);
        key
    }

    fn add(
        &mut self,
        upper_key: DefaultKey,
        e: (String, String, HashMap<String, String>),
    ) -> DefaultKey {
        let key = self.data.insert(e);
        if let Some(upper) = self.subset.get_mut(upper_key) {
            upper.push(key);
        } else {
            self.subset.insert(upper_key, vec![key]);
        }
        key
    }

    fn get_key(&mut self, step: usize) -> Option<DefaultKey> {
        let mut k = self.root.last()?;
        if step == 0 {
            return Some(*k);
        }
        let mut n = step;
        while let Some(o) = self.subset.get(*k) {
            n -= 1;
            if n == 0 {
                return o.last().copied();
            } else if let Some(o) = o.last() {
                k = o;
            } else {
                break;
            }
        }
        None
    }

    fn to_element(mut self, eh: &mut ElementHolder, error: &mut ErrorHolder) {
        let root = self.root.clone();
        for k in root {
            if let Some(o) = self.data.remove(k) {
                match Mark::try_from(&o.0) {
                    Ok(m) => {
                        let e = Element::new(m, o.1, AttributeHolder::from(o.2, error));
                        let rk = eh.add_root(e);
                        if let Some(j) = self.subset.remove(k) {
                            self.to_element0(j, eh, rk, error);
                        }
                    }
                    Err(e) => {
                        trace!("{e}");
                        error.push(e);
                    }
                }
            }
        }
    }

    fn to_element0(
        &mut self,
        v: Vec<DefaultKey>,
        eh: &mut ElementHolder,
        upper_key: ElementKey,
        error: &mut ErrorHolder,
    ) {
        for k in v {
            if let Some(o) = self.data.remove(k) {
                match Mark::try_from(&o.0) {
                    Ok(m) => {
                        let e = Element::new(m, o.1, AttributeHolder::from(o.2, error));
                        if let Some(ek) = eh.add(upper_key, e) {
                            if let Some(j) = self.subset.remove(k) {
                                self.to_element0(j, eh, ek, error);
                            }
                        }
                    }
                    Err(e) => {
                        trace!("{e}");
                        error.push(e);
                    }
                }
            }
        }
    }
}

#[derive(Default)]
struct Builder {
    temporary: Vec<String>,
    rst: TempResult,
    last_one: Option<DefaultKey>,
    last_step: usize,
    error: ErrorHolder,
    f: bool,
}

impl Builder {
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

    fn to_element(mut self) -> (Option<ElementHolder>, ErrorHolder) {
        if self.rst.has_root() {
            let mut eh = ElementHolder::default();
            self.rst.to_element(&mut eh, &mut self.error);
            (Some(eh), self.error)
        } else {
            (None, self.error)
        }
    }
}

impl XParser for Builder {
    fn start_tag(&mut self, s: String) {
        if s.is_empty() {
            return;
        }
        self.temporary.push(s.clone());
        let step = self.step();
        let e = (s, String::new(), Default::default());
        if step == 0 {
            self.last_one.replace(self.rst.add_root(e));
            self.last_step = step;
            return;
        } else if step == self.last_step + 1 {
            if let Some(p) = self.last_one {
                self.last_one.replace(self.rst.add(p, e));
                self.last_step = step;
                return;
            }
        }
        self.last_one = self.rst.get_key(step - 1).map(|k| self.rst.add(k, e));
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
                if let Some(o) = self.rst.data.get_mut(p) {
                    o.1.push_str(&s);
                    return;
                }
            }
        }
        if let Some(k) = self.rst.get_key(step) {
            if let Some(o) = self.rst.data.get_mut(k) {
                o.1.push_str(&s);
            }
        }
    }

    fn attribute(&mut self, k: String, s: String) {
        if let Some(p) = self.last_one {
            if let Some(o) = self.rst.data.get_mut(p) {
                if o.2.contains_key(&k) {
                } else {
                    o.2.insert(k, s);
                }
            }
        }
    }

    fn error(&mut self, e: Error) {
        self.error.push(e)
    }
}

impl s::Output for Builder {
    fn child(&mut self) {
        self.f = true;
    }

    fn root(&mut self, n: usize) {
        self.temporary.truncate(n);
        self.f = true;
    }

    fn upper(&mut self, n: usize) {
        let i = self.temporary.len();
        if n < i {
            self.temporary.truncate(i - n);
        }
        self.f = false;
    }

    fn tag(&mut self, s: String) {
        if self.f {
            self.f = false;
        } else {
            self.temporary.pop();
        }

        self.start_tag(s);
    }

    fn attribute(&mut self, k: String, v: String) {
        if let Some(p) = self.last_one {
            if let Some(o) = self.rst.data.get_mut(p) {
                if o.2.contains_key(&k) {
                } else {
                    o.2.insert(k, v);
                }
            }
        }
    }

    fn text(&mut self, s: String) {
        self.tag_text(s);
    }

    fn error(&mut self, e: Error) {
        self.error.push(e)
    }
}

pub(super) fn accept(s: &str) -> (Option<ElementHolder>, ErrorHolder) {
    let mut o = Builder::default();
    Context::new(&mut o).parse_str(s);
    o.to_element()
}

pub(super) fn accept_s(s: &str) -> (Option<ElementHolder>, ErrorHolder) {
    s::Parser::new(Builder::default()).parse_str(s).to_element()
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: &str = "<a b=0 c d= '1' e='0&amp;0&lt;0&gt;0&quot;0&nbsp;0&apos;' f=\"2\">a</a>";
    const B: &str = "<a b=0 c d= '1' e= f=\"2\">a</a>";
    const C: &str = "<a b=0 c d= '1' e=' f=\"2\"/>";
    const D: &str = "<a b=0 c d= '1' e='' f=\"2\">a&amp;0&lt;0&gt;0&quot;0&nbsp;0&apos;  0";

    #[test]
    fn build() {
        println!("{:?}", accept(A));
        println!("{:?}", accept(B));
        println!("{:?}", accept(C));
        println!("{:?}", accept_s(D));
    }
}
