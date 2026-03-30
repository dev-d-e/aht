use super::format::*;
use super::*;
use slotmap::{SlotMap, new_key_type};
use std::collections::HashMap;

///Represents element's attributes.
#[derive(Default)]
#[repr(transparent)]
pub struct AttributeHolder(HashMap<AttrName, Attribute>);

deref!(AttributeHolder, HashMap<AttrName, Attribute>, 0);

impl std::fmt::Debug for AttributeHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(&self.0).finish()
    }
}

impl AttributeHolder {
    fn add(&mut self, k: &str, s: &mut String) -> Option<Error> {
        AttrName::try_from(k)
            .and_then(|k| {
                Attribute::from(&k, s).map(|a| {
                    self.insert(k, a);
                })
            })
            .err()
    }

    pub(super) fn from(o: HashMap<String, String>, error: &mut ErrorHolder) -> Self {
        let mut r = Self::default();
        for (k, mut s) in o {
            if let Some(e) = r.add(&k, &mut s) {
                trace!("{e}");
                error.push(e);
            }
        }
        r
    }
}

///Represents element.
#[derive(Getters, MutGetters)]
pub struct Element {
    #[getset(get = "pub")]
    mark_type: Mark,
    #[getset(get = "pub", get_mut = "pub")]
    text: String,
    #[getset(get = "pub", get_mut = "pub")]
    attribute: AttributeHolder,
    #[getset(get = "pub", get_mut = "pub(crate)")]
    subset: Vec<ElementKey>,
    #[getset(get = "pub")]
    upper: Option<ElementKey>,
}

impl std::fmt::Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Element {{ ")?;
        write!(f, "mark_type: {:?}, ", self.mark_type)?;
        write!(f, "text: {:?}, ", self.text)?;
        write!(f, "attribute: {:?}, upper: ", self.attribute)?;
        if let Some(o) = self.upper {
            write!(f, "{:?}, ", o)?;
        } else {
            write!(f, "{:?}, ", self.upper)?;
        }
        write!(f, "subset: {:?} }}", self.subset)
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Element {{ ")?;
        write!(f, "mark_type: {:?}, upper: ", self.mark_type)?;
        if let Some(o) = self.upper {
            write!(f, "{:?}, ", o)?;
        } else {
            write!(f, "{:?}, ", self.upper)?;
        }
        write!(f, "subset: {:?} }}", self.subset)
    }
}

impl Element {
    ///Creates a new element.
    pub fn new(mark_type: Mark, text: String, attribute: AttributeHolder) -> Self {
        Self {
            mark_type,
            text,
            attribute,
            subset: Default::default(),
            upper: None,
        }
    }

    ///Returns a string slice of this element's type.
    pub fn as_str(&self) -> &str {
        self.mark_type.as_str()
    }

    ///Returns a reference to attribute.
    pub fn attribute_get(&self, a: &AttrName) -> Option<&Attribute> {
        self.attribute.get(a)
    }

    pub(crate) fn get_value_or_text(&self) -> &str {
        if let Some(a) = self.value() {
            if a.len() > 0 {
                return a;
            }
        }
        &self.text
    }

    ///Inserts an attribute.
    pub fn attribute_insert(&mut self, a: Attribute) {
        self.attribute.insert(a.name(), a);
    }

    attribute_get!(class, String, CLASS);
    attribute_get!(column, Points, COLUMN);
    attribute_get!(disabled, bool, DISABLED);
    attribute_get!(height, Distance, HEIGHT);
    attribute_get!(hidden, bool, HIDDEN);
    attribute_get!(id, String, ID);
    attribute_get!(multiple, bool, MULTIPLE);
    attribute_get!(ordinal, Ordinal, ORDINAL);
    attribute_get!(position, Coord, POSITION);
    attribute_get!(row, Points, ROW);
    attribute_get!(selected, bool, SELECTED);
    attribute_get!(script_type, ScriptType, TYPE);
    attribute_get!(value, String, VALUE);
    attribute_get_or_insert!(value_or_insert, String, VALUE, String::new());
    attribute_get!(width, Distance, WIDTH);

    ///Inserts an element into subset.
    pub fn subset_insert(&mut self, n: usize, a: ElementKey) {
        if n < self.subset.len() {
            self.subset.insert(n, a);
        }
    }

    ///Removes an element from subset.
    pub fn subset_remove(&mut self, n: usize) -> bool {
        if n < self.subset.len() {
            self.subset.remove(n);
            true
        } else {
            false
        }
    }

    ///Returns index of an element in subset.
    pub fn subset_element_index(&self, k: ElementKey) -> Option<usize> {
        self.subset.iter().position(|i| i == &k)
    }
}

new_key_type! {
///Represents element key.
pub struct ElementKey;
}

///Represents structure of all elements.
#[derive(Default)]
pub struct ElementHolder {
    data: SlotMap<ElementKey, Element>,
    root: Vec<ElementKey>,
}

impl std::fmt::Debug for ElementHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ElementHolder {{[")?;
        for o in &self.data {
            writeln!(f, "{:?}", o)?;
        }
        write!(f, "]}}")
    }
}

impl std::fmt::Display for ElementHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ElementHolder {{[")?;
        for o in &self.data {
            writeln!(f, "({:?}, {})", o.0, o.1)?;
        }
        write!(f, "]}}")
    }
}

impl ElementHolder {
    ///Adds a root.
    pub fn add_root(&mut self, e: Element) -> ElementKey {
        let key = self.data.insert(e);
        self.root.push(key);
        key
    }

    ///Adds an element with an upper key.
    pub fn add(&mut self, upper_key: ElementKey, mut e: Element) -> Option<ElementKey> {
        if !self.data.contains_key(upper_key) {
            return None;
        }
        e.upper.replace(upper_key);
        let key = self.data.insert(e);
        let upper = self.data.get_mut(upper_key)?;
        upper.subset.push(key);
        Some(key)
    }

    ///Removes an element corresponding to the key.
    pub fn remove(&mut self, key: ElementKey) {
        let subset = if let Some(e) = self.data.get(key) {
            e.subset.to_vec()
        } else {
            return;
        };
        for k in subset {
            self.remove(k);
        }

        if let Some(upper) = self
            .get(key)
            .and_then(|e| e.upper)
            .and_then(|upper_key| self.data.get_mut(upper_key))
        {
            upper.subset.retain(|k| k != &key);
        }

        self.data.remove(key);

        if let Some(n) = self.root.iter().position(|k| k == &key) {
            self.root.remove(n);
        }
    }

    ///Sets an upper for an element.
    pub fn set_upper(&mut self, key: ElementKey, upper_key: ElementKey) -> bool {
        if !self.data.contains_key(key) && !self.data.contains_key(upper_key) {
            return false;
        }
        if let Some(e) = self.data.get_mut(key) {
            e.upper.replace(upper_key);
        }
        if let Some(upper) = self.data.get_mut(upper_key) {
            upper.subset.push(key);
        }
        true
    }

    ///Returns a reference corresponding to the key.
    pub fn get(&self, key: ElementKey) -> Option<&Element> {
        self.data.get(key)
    }

    ///Returns a mutable reference corresponding to the key.
    pub fn get_mut(&mut self, key: ElementKey) -> Option<&mut Element> {
        self.data.get_mut(key)
    }

    ///Returns text corresponding to the key.
    pub fn text(&self, key: ElementKey) -> Option<&str> {
        self.data
            .get(key)
            .map(|e| e.text.as_str())
            .filter(|s| s.len() > 0)
    }

    ///Returns subset keys corresponding to the key.
    pub fn get_subset(&self, key: ElementKey) -> Vec<ElementKey> {
        self.data
            .get(key)
            .map(|e| e.subset().to_vec())
            .unwrap_or_else(|| Vec::new())
    }

    pub(crate) fn subset_with_mark(&self, key: ElementKey, mark_type: Mark) -> Vec<ElementKey> {
        let i = self.get_subset(key).into_iter();
        let mut r = Vec::new();
        for k in i {
            if let Some(e) = self.get(k) {
                if e.mark_type() == &mark_type {
                    r.push(k);
                }
            }
        }
        r
    }

    ///Parse a string slice to it.
    pub fn parse(buf: &str, o: MarkNumber) -> (Option<Self>, ErrorHolder) {
        match o {
            MarkNumber::Double => accept(buf),
            MarkNumber::Single => accept_s(buf),
        }
    }

    pub(crate) fn first_root(&self) -> Option<ElementKey> {
        self.root.first().copied()
    }

    fn is_mark_type(&self, key: ElementKey, mark_type: Mark) -> bool {
        self.data
            .get(key)
            .map(|e| e.mark_type == mark_type)
            .unwrap_or(false)
    }
}

impl TryFrom<ElementHolder> for Page {
    type Error = ElementHolder;

    fn try_from(eh: ElementHolder) -> Result<Self, Self::Error> {
        if let Some(a) = eh.root.first().and_then(|&root| eh.get(root)) {
            if a.mark_type == Mark::AHT && a.subset.len() >= 4 {
                let head = a.subset[0];
                let body = a.subset[1];
                let style = a.subset[2];
                let script = a.subset[3];
                if eh.is_mark_type(head, Mark::HEAD)
                    && eh.is_mark_type(body, Mark::BODY)
                    && eh.is_mark_type(style, Mark::STYLE)
                    && eh.is_mark_type(script, Mark::SCRIPT)
                {
                    return Ok(Self::new(eh, head, body, style, script));
                }
            }
        }
        Err(eh)
    }
}

///Represents mark number.
#[derive(Debug)]
pub enum MarkNumber {
    Double,
    Single,
}
