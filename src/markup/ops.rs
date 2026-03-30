use super::*;

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
}

impl<T> FindDescendant<T>
where
    T: Requirement,
{
    ///Creates a new instance. `n` is nesting number, searches all descendant if it's 0.
    pub fn new(f: T, n: usize) -> Self {
        Self { f, n }
    }

    #[inline]
    fn all(&self, k: ElementKey, eh: &ElementHolder, v: &mut Vec<ElementKey>) {
        if let Some(e) = eh.get(k) {
            if self.f.satisfy(&e) {
                v.push(k)
            }
            e.subset().iter().for_each(|&k| self.all(k, eh, v));
        }
    }

    #[inline]
    fn subtract(&self, k: ElementKey, i: usize, eh: &ElementHolder, v: &mut Vec<ElementKey>) {
        if let Some(e) = eh.get(k) {
            if self.f.satisfy(&e) {
                v.push(k)
            }
            if i > 1 {
                let i = i - 1;
                e.subset().iter().for_each(|&k| self.subtract(k, i, eh, v));
            }
        }
    }

    ///Gets elements that satisfy the requirements by an element to the vec.
    pub fn element(&self, k: ElementKey, eh: &ElementHolder, v: &mut Vec<ElementKey>) {
        if self.n == 0 {
            if let Some(e) = eh.get(k) {
                e.subset().iter().for_each(|&k| self.all(k, eh, v));
            }
        } else {
            if let Some(e) = eh.get(k) {
                let i = self.n;
                e.subset().iter().for_each(|&k| self.subtract(k, i, eh, v));
            }
        }
    }

    ///Gets elements that satisfy the requirements by some elements to the vec.
    pub fn elements(&self, ks: &[ElementKey], eh: &ElementHolder, v: &mut Vec<ElementKey>) {
        for &k in ks {
            self.element(k, eh, v);
        }
    }

    ///Returns unique element that satisfy the requirements by an element.
    pub fn element_to_unique(&self, k: ElementKey, eh: &ElementHolder) -> Option<ElementKey> {
        let e = eh.get(k)?;
        if self.f.satisfy(&e) {
            return Some(k);
        }
        e.subset()
            .iter()
            .find_map(|&k| self.element_to_unique(k, eh))
    }

    ///Returns unique element that satisfy the requirements by some elements.
    pub fn elements_to_unique(&self, ks: &[ElementKey], eh: &ElementHolder) -> Option<ElementKey> {
        ks.iter().find_map(|&k| self.element_to_unique(k, eh))
    }

    #[inline]
    pub(crate) fn all_with_first_root(&self, eh: &ElementHolder, v: &mut Vec<ElementKey>) {
        if let Some(k) = eh.first_root() {
            self.all(k, eh, v);
        }
    }

    #[inline]
    pub(crate) fn unique_with_first_root(&self, eh: &ElementHolder) -> Option<ElementKey> {
        let k = eh.first_root()?;
        self.element_to_unique(k, eh)
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

    #[inline]
    fn get<'a>(&self, k: ElementKey, eh: &'a ElementHolder) -> Option<&'a [ElementKey]> {
        let e = eh.get(eh.get(k)?.upper().clone()?)?;
        let i = e.subset_element_index(k)?;
        let n = i + self.n;
        let i = i + 1;
        let subset_len = e.subset().len();
        if self.n == 0 || n >= subset_len {
            e.subset().get(i..)
        } else {
            e.subset().get(i..=n)
        }
    }

    ///Gets elements that satisfy the requirements by an element to the vec.
    pub fn element(&self, k: ElementKey, eh: &ElementHolder, v: &mut Vec<ElementKey>) {
        if let Some(r) = self.get(k, eh) {
            r.into_iter().for_each(|&j| {
                if let Some(e) = eh.get(j) {
                    if self.f.satisfy(&e) {
                        v.push(j);
                    }
                }
            });
        }
    }

    ///Gets elements that satisfy the requirements by some elements to the vec.
    pub fn elements(&self, ks: &[ElementKey], eh: &ElementHolder, v: &mut Vec<ElementKey>) {
        for &k in ks {
            self.element(k, eh, v);
        }
    }

    ///Returns unique element that satisfy the requirements by an element.
    pub fn element_to_unique(&self, k: ElementKey, eh: &ElementHolder) -> Option<ElementKey> {
        self.get(k, eh)?.into_iter().find_map(|&j| {
            let e = eh.get(j)?;
            if self.f.satisfy(&e) { Some(j) } else { None }
        })
    }

    ///Returns unique element that satisfy the requirements by some elements.
    pub fn elements_to_unique(&self, ks: &[ElementKey], eh: &ElementHolder) -> Option<ElementKey> {
        ks.iter().find_map(|&k| self.element_to_unique(k, eh))
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
}

impl<T> FindPrecedingSibling<T>
where
    T: Requirement,
{
    ///Creates a new instance. `n` is range, searches all preceding sibling if it's 0.
    pub fn new(f: T, n: usize) -> Self {
        Self { f, n }
    }

    #[inline]
    fn get<'a>(&self, k: ElementKey, eh: &'a ElementHolder) -> Option<&'a [ElementKey]> {
        let e = eh.get(eh.get(k)?.upper().clone()?)?;
        let i = e.subset_element_index(k)?;
        if self.n == 0 {
            e.subset().get(..i)
        } else {
            let n = i.saturating_sub(self.n);
            e.subset().get(n..i)
        }
    }

    ///Gets elements that satisfy the requirements by an element to the vec.
    pub fn element(&self, k: ElementKey, eh: &ElementHolder, v: &mut Vec<ElementKey>) {
        if let Some(r) = self.get(k, eh) {
            r.into_iter().for_each(|&j| {
                if let Some(e) = eh.get(j) {
                    if self.f.satisfy(&e) {
                        v.push(j);
                    }
                }
            })
        }
    }

    ///Gets elements that satisfy the requirements by some elements to the vec.
    pub fn elements(&self, ks: &[ElementKey], eh: &ElementHolder, v: &mut Vec<ElementKey>) {
        for &k in ks {
            self.element(k, eh, v);
        }
    }

    ///Returns unique element that satisfy the requirements by an element.
    pub fn element_to_unique(&self, k: ElementKey, eh: &ElementHolder) -> Option<ElementKey> {
        self.get(k, eh)?.into_iter().find_map(|&j| {
            let e = eh.get(j)?;
            if self.f.satisfy(&e) { Some(j) } else { None }
        })
    }

    ///Returns unique element that satisfy the requirements by some elements.
    pub fn elements_to_unique(&self, ks: &[ElementKey], eh: &ElementHolder) -> Option<ElementKey> {
        ks.iter().find_map(|&k| self.element_to_unique(k, eh))
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
