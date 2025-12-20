use super::format::*;
use super::*;
use getset::{Getters, MutGetters};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};

#[derive(Default)]
pub(crate) struct AttributeHolder(HashMap<AttrName, Attribute>);

deref!(AttributeHolder,HashMap<AttrName, Attribute>,0);

impl std::fmt::Debug for AttributeHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(&self.0).finish()
    }
}

impl AttributeHolder {
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
}

#[derive(Default)]
pub(crate) struct ElementHolder(VecDeque<Arc<RwLock<Element>>>);

deref!(ElementHolder, VecDeque<Arc<RwLock<Element>>>, 0);

impl std::fmt::Debug for ElementHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_list();
        self.0.iter().for_each(|o| {
            if let Ok(o) = o.try_read() {
                f.entry(&o);
            }
        });
        f.finish()
    }
}

impl ElementHolder {}

///Represents element.
#[derive(Getters, MutGetters)]
pub struct Element {
    #[getset(get = "pub(crate)")]
    mark_type: Mark,
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    text: String,
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    attribute: AttributeHolder,
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    subset: ElementHolder,
    #[getset(get = "pub(crate)")]
    upper: Option<Arc<RwLock<Self>>>,
}

impl std::fmt::Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Element")
            .field("mark_type", &self.mark_type)
            .field("text", &self.text)
            .field("attribute", &self.attribute)
            .field("subset", &self.subset)
            .field("upper", &self.upper.is_some())
            .finish()
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Element")
            .field("mark_type", &self.mark_type)
            .field("text", &self.text)
            .field("attribute", &!self.attribute.is_empty())
            .field("subset", &!self.subset.is_empty())
            .field("upper", &self.upper.is_some())
            .finish()
    }
}

impl Element {
    ///Creates a new element.
    pub fn new(mark_type: Mark, text: String) -> Self {
        Self {
            mark_type,
            text,
            attribute: Default::default(),
            subset: Default::default(),
            upper: None,
        }
    }

    ///Parse a string slice to many.
    pub fn parse_many(buf: &str, o: MarkNumber) -> (VecDeque<Self>, ErrorHolder) {
        let (v, mut error) = match o {
            MarkNumber::Double => accept(buf),
            MarkNumber::Single => accept_s(buf),
        };
        let v = v
            .into_iter()
            .filter_map(|o| {
                CheckElement::from(o, &mut error)
                    .map_err(|e| {
                        trace!("{e}");
                        error.push(e);
                    })
                    .ok()
            })
            .map(|o| o.into())
            .collect();
        (v, error)
    }

    ///Parse a string slice to it.
    pub fn parse_one(buf: &str, o: MarkNumber) -> (Option<Self>, ErrorHolder) {
        let (mut v, error) = Self::parse_many(buf, o);
        (v.pop_front(), error)
    }

    ///Parse a string slice to it.
    pub fn parse_d_one(buf: &str) -> (Option<Self>, ErrorHolder) {
        Self::parse_one(buf, MarkNumber::Double)
    }

    ///Parse a string slice to it.
    pub fn parse_s_one(buf: &str) -> (Option<Self>, ErrorHolder) {
        Self::parse_one(buf, MarkNumber::Single)
    }

    ///Returns a string slice of this element's type.
    pub fn as_str(&self) -> &str {
        self.mark_type.as_str()
    }

    ///Converts to page.
    pub fn to_page(self) -> Result<Page, Self> {
        if self.mark_type == Mark::AHT && self.subset.len() >= 4 {
            let head = self.subset.0[0].clone();
            let body = self.subset.0[1].clone();
            let style = self.subset.0[2].clone();
            let script = self.subset.0[3].clone();
            if head
                .read()
                .map(|e| e.mark_type == Mark::HEAD)
                .unwrap_or(false)
                && body
                    .read()
                    .map(|e| e.mark_type == Mark::BODY)
                    .unwrap_or(false)
                && style
                    .read()
                    .map(|e| e.mark_type == Mark::STYLE)
                    .unwrap_or(false)
                && script
                    .read()
                    .map(|e| e.mark_type == Mark::SCRIPT)
                    .unwrap_or(false)
            {
                return Ok(Page::new(self, head, body, style, script));
            }
        }
        Err(self)
    }

    ///Returns a reference to attribute.
    pub fn attribute_get(&self, a: &AttrName) -> Option<&Attribute> {
        self.attribute.get(a)
    }

    pub(crate) fn get_value_or_text(&self) -> &str {
        if let Some(a) = self.attribute.value() {
            if a.len() > 0 {
                return a;
            }
        }
        self.text()
    }

    ///Inserts an attribute.
    pub fn attribute_insert(&mut self, a: Attribute) {
        self.attribute.insert(a.name(), a);
    }

    ///Inserts an element into subset.
    pub fn subset_insert(&mut self, n: usize, a: Self) {
        if n < self.subset.len() {
            self.subset.insert(n, Arc::new(RwLock::new(a)));
        }
    }

    ///Removes an element from subset.
    pub fn subset_remove(&mut self, n: usize) -> Option<Arc<RwLock<Self>>> {
        self.subset.remove(n)
    }

    ///Removes an element from subset.
    pub fn subset_swap_remove(&mut self, o: Arc<RwLock<Self>>, a: Self) {
        if let Some(i) = self
            .subset
            .iter()
            .position(|k| Arc::as_ptr(&k) == Arc::as_ptr(&o))
        {
            self.subset.push_back(Arc::new(RwLock::new(a)));
            self.subset.swap_remove_back(i);
        }
    }

    fn set_upper(&mut self, upper_arc: Arc<RwLock<Self>>, self_arc: Arc<RwLock<Self>>) {
        self.upper.replace(upper_arc);
        self.subset.iter_mut().for_each(|o| {
            if let Ok(mut e) = o.write() {
                e.set_upper(self_arc.clone(), o.clone());
            }
        });
    }

    fn subset_upper(&mut self) {
        self.subset.iter_mut().for_each(|a| {
            if let Ok(mut e) = a.write() {
                e.subset.iter_mut().for_each(|b| {
                    if let Ok(mut e) = b.write() {
                        e.set_upper(a.clone(), b.clone());
                    }
                });
            }
        });
    }

    ///Returns index of an element in subset.
    pub fn subset_element_index(&self, o: &Arc<RwLock<Self>>) -> Option<usize> {
        self.subset
            .iter()
            .position(|k| Arc::as_ptr(k) == Arc::as_ptr(o))
    }
}

impl From<CheckElement> for Element {
    fn from(check_element: CheckElement) -> Self {
        let mut e = Self::new(check_element.mark_type, check_element.text);
        e.attribute.0 = check_element.attribute;
        for o in check_element.subset.into_iter() {
            e.subset.push_back(Arc::new(RwLock::new(o.into())));
        }
        e.subset_upper();
        e
    }
}

#[derive(Debug)]
struct CheckElement {
    mark_type: Mark,
    text: String,
    attribute: HashMap<AttrName, Attribute>,
    subset: Vec<Self>,
}

impl CheckElement {
    fn new(mark_type: Mark, text: String) -> Self {
        Self {
            mark_type,
            text,
            attribute: HashMap::new(),
            subset: Vec::new(),
        }
    }

    fn from(unclear_element: UnclearElement, error: &mut Vec<Error>) -> Result<Self> {
        Mark::try_from(&unclear_element.key).map(|m| {
            let mut check_element = Self::new(m, unclear_element.text);
            for (k, s) in unclear_element.attribute.into_iter() {
                match AttrName::try_from(&k) {
                    Ok(k) => {
                        if let Some(mut s) = s {
                            if s.len() > 0 {
                                match Attribute::from(&k, &mut s) {
                                    Ok(a) => {
                                        check_element.attribute.insert(k, a);
                                    }
                                    Err(e) => {
                                        trace!("{e}");
                                        error.push(e);
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        trace!("{e}");
                        error.push(e);
                    }
                }
            }
            for o in unclear_element.subset.into_iter() {
                match Self::from(o, error) {
                    Ok(c) => check_element.subset.push(c),
                    Err(e) => {
                        trace!("{e}");
                        error.push(e);
                    }
                }
            }
            check_element
        })
    }
}

///Represents mark number.
#[derive(Debug)]
pub enum MarkNumber {
    Double,
    Single,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn element() {
        let s = "<aht>
        <head lang=en>
            <title>1</title>
        </head>
        <body column=\"\" row=\"2\">
            <inp name=\"\" value=\"\" readonly required>input</inp>
            <button href=\"\" async=true>button</button>
            <area class=\"\" id=\"\" width=\"1000\" height=\"100\" column=2 row=\"\"></area>
        </body>
        <style>
        </style>
        <script>
        </script>
     </aht>";
        let (e, err) = Element::parse_d_one(&s);
        for o in err.iter() {
            println!("{:?}", o);
        }
        println!("{}", e.unwrap());

        let (p, err) = Page::parse(&s);
        for o in err.iter() {
            println!("{:?}", o);
        }
        println!("{:?}", p.unwrap());
    }
}
