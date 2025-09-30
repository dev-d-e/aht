mod format;

use crate::content::Body;
use crate::head::Head;
use crate::metadata::{Script, Style};
use crate::parts::*;
use crate::utils::*;
use skia_safe::Canvas;
use std::collections::{HashMap, VecDeque};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, RwLock};

const AHT: &str = "aht";
const AREA: &str = "area";
const AUDIO: &str = "audio";
const BODY: &str = "body";
const BUTTON: &str = "button";
const CANVAS: &str = "canvas";
const FORM: &str = "form";
const HEAD: &str = "head";
const IFRAME: &str = "iframe";
const IMG: &str = "img";
const INP: &str = "inp";
const OPTION: &str = "option";
const PT: &str = "pt";
const SCRIPT: &str = "script";
const SELECT: &str = "select";
const STYLE: &str = "style";
const TIME: &str = "time";
const TITLE: &str = "title";
const VIDEO: &str = "video";

///Represents markup.
#[derive(Debug, PartialEq)]
pub enum Mark {
    AHT,
    AREA,
    AUDIO,
    BODY,
    BUTTON,
    CANVAS,
    FORM,
    HEAD,
    IFRAME,
    IMG,
    INP,
    OPTION,
    PT,
    SCRIPT,
    SELECT,
    STYLE,
    TIME,
    TITLE,
    VIDEO,
}

impl Mark {
    ///Converts from a string slice.
    pub fn from(s: &str) -> Option<Self> {
        match s {
            AHT => Some(Self::AHT),
            AREA => Some(Self::AREA),
            AUDIO => Some(Self::AUDIO),
            BODY => Some(Self::BODY),
            BUTTON => Some(Self::BUTTON),
            CANVAS => Some(Self::CANVAS),
            FORM => Some(Self::FORM),
            HEAD => Some(Self::HEAD),
            IFRAME => Some(Self::IFRAME),
            IMG => Some(Self::IMG),
            INP => Some(Self::INP),
            OPTION => Some(Self::OPTION),
            PT => Some(Self::PT),
            SCRIPT => Some(Self::SCRIPT),
            SELECT => Some(Self::SELECT),
            STYLE => Some(Self::STYLE),
            TIME => Some(Self::TIME),
            TITLE => Some(Self::TITLE),
            VIDEO => Some(Self::VIDEO),
            _ => None,
        }
    }

    ///Returns a string slice.
    pub fn as_str(&self) -> &str {
        match self {
            Self::AHT => AHT,
            Self::AREA => AREA,
            Self::AUDIO => AUDIO,
            Self::BODY => BODY,
            Self::BUTTON => BUTTON,
            Self::CANVAS => CANVAS,
            Self::FORM => FORM,
            Self::HEAD => HEAD,
            Self::IFRAME => IFRAME,
            Self::IMG => IMG,
            Self::INP => INP,
            Self::OPTION => OPTION,
            Self::PT => PT,
            Self::SCRIPT => SCRIPT,
            Self::SELECT => SELECT,
            Self::STYLE => STYLE,
            Self::TIME => TIME,
            Self::TITLE => TITLE,
            Self::VIDEO => VIDEO,
        }
    }
}

const ACTION: &str = "action";
const CLASS: &str = "class";
const COLUMN: &str = "column";
const DISABLED: &str = "disabled";
const ENCTYPE: &str = "enctype";
const HEIGHT: &str = "height";
const HIDDEN: &str = "hidden";
const HREF: &str = "href";
const ID: &str = "id";
const LANG: &str = "lang";
const METHOD: &str = "method";
const MULTIPLE: &str = "multiple";
const NAME: &str = "name";
const ORDINAL: &str = "ordinal";
const POSITION: &str = "position";
const READONLY: &str = "readonly";
const REQUIRED: &str = "required";
const ROW: &str = "row";
const SELECTED: &str = "selected";
const SRC: &str = "src";
const TIP: &str = "tip";
const TYPE: &str = "type";
const VALUE: &str = "value";
const WIDTH: &str = "width";

const ONABORT: &str = "onabort";
const ONBLUR: &str = "onblur";
const ONCANCEL: &str = "oncancel";
const ONCHANGE: &str = "onchange";
const ONCLICK: &str = "onclick";
const ONCLOSE: &str = "onclose";
const ONFOCUS: &str = "onfocus";
const ONINVALID: &str = "oninvalid";
const ONLOAD: &str = "onload";
const ONRESIZE: &str = "onresize";
const ONSCROLL: &str = "onscroll";

///Represents attribute name.
#[derive(Debug, Eq, Hash, PartialEq)]
pub enum AttrName {
    ACTION,
    CLASS,
    COLUMN,
    DISABLED,
    ENCTYPE,
    HEIGHT,
    HIDDEN,
    HREF,
    ID,
    LANG,
    METHOD,
    MULTIPLE,
    NAME,
    ORDINAL,
    POSITION,
    READONLY,
    REQUIRED,
    ROW,
    SELECTED,
    SRC,
    TIP,
    TYPE,
    VALUE,
    WIDTH,
    ONABORT,
    ONBLUR,
    ONCANCEL,
    ONCHANGE,
    ONCLICK,
    ONCLOSE,
    ONFOCUS,
    ONINVALID,
    ONLOAD,
    ONRESIZE,
    ONSCROLL,
}

impl AttrName {
    ///Returns a string slice.
    pub fn as_str(&self) -> &str {
        match self {
            AttrName::ACTION => ACTION,
            AttrName::CLASS => CLASS,
            AttrName::COLUMN => COLUMN,
            AttrName::DISABLED => DISABLED,
            AttrName::ENCTYPE => ENCTYPE,
            AttrName::HEIGHT => HEIGHT,
            AttrName::HIDDEN => HIDDEN,
            AttrName::HREF => HREF,
            AttrName::ID => ID,
            AttrName::LANG => LANG,
            AttrName::METHOD => METHOD,
            AttrName::MULTIPLE => MULTIPLE,
            AttrName::NAME => NAME,
            AttrName::ORDINAL => ORDINAL,
            AttrName::POSITION => POSITION,
            AttrName::READONLY => READONLY,
            AttrName::REQUIRED => REQUIRED,
            AttrName::ROW => ROW,
            AttrName::SELECTED => SELECTED,
            AttrName::SRC => SRC,
            AttrName::TIP => TIP,
            AttrName::TYPE => TYPE,
            AttrName::VALUE => VALUE,
            AttrName::WIDTH => WIDTH,
            AttrName::ONABORT => ONABORT,
            AttrName::ONBLUR => ONBLUR,
            AttrName::ONCANCEL => ONCANCEL,
            AttrName::ONCHANGE => ONCHANGE,
            AttrName::ONCLICK => ONCLICK,
            AttrName::ONCLOSE => ONCLOSE,
            AttrName::ONFOCUS => ONFOCUS,
            AttrName::ONINVALID => ONINVALID,
            AttrName::ONLOAD => ONLOAD,
            AttrName::ONRESIZE => ONRESIZE,
            AttrName::ONSCROLL => ONSCROLL,
        }
    }

    ///Converts from a string slice.
    pub fn from(s: &str) -> Option<Self> {
        match s {
            ACTION => Some(Self::ACTION),
            CLASS => Some(Self::CLASS),
            COLUMN => Some(Self::COLUMN),
            DISABLED => Some(Self::DISABLED),
            ENCTYPE => Some(Self::ENCTYPE),
            HEIGHT => Some(Self::HEIGHT),
            HIDDEN => Some(Self::HIDDEN),
            HREF => Some(Self::HREF),
            ID => Some(Self::ID),
            LANG => Some(Self::LANG),
            METHOD => Some(Self::METHOD),
            MULTIPLE => Some(Self::MULTIPLE),
            NAME => Some(Self::NAME),
            ORDINAL => Some(Self::ORDINAL),
            READONLY => Some(Self::READONLY),
            REQUIRED => Some(Self::REQUIRED),
            ROW => Some(Self::ROW),
            SELECTED => Some(Self::SELECTED),
            SRC => Some(Self::SRC),
            TIP => Some(Self::TIP),
            TYPE => Some(Self::TYPE),
            VALUE => Some(Self::VALUE),
            WIDTH => Some(Self::WIDTH),
            ONABORT => Some(Self::ONABORT),
            ONBLUR => Some(Self::ONBLUR),
            ONCANCEL => Some(Self::ONCANCEL),
            ONCHANGE => Some(Self::ONCHANGE),
            ONCLICK => Some(Self::ONCLICK),
            ONCLOSE => Some(Self::ONCLOSE),
            ONFOCUS => Some(Self::ONFOCUS),
            ONINVALID => Some(Self::ONINVALID),
            ONLOAD => Some(Self::ONLOAD),
            ONRESIZE => Some(Self::ONRESIZE),
            ONSCROLL => Some(Self::ONSCROLL),
            _ => None,
        }
    }
}

///Represents attribute.
#[derive(Debug)]
pub enum Attribute {
    ACTION(String),
    CLASS(String),
    COLUMN(Points),
    DISABLED(bool),
    ENCTYPE(String),
    HEIGHT(Distance),
    HIDDEN(bool),
    HREF(String),
    ID(String),
    LANG(String),
    METHOD(String),
    MULTIPLE(bool),
    NAME(String),
    ORDINAL(Ordinal),
    POSITION(Coord),
    READONLY(bool),
    REQUIRED(bool),
    ROW(Points),
    SELECTED(bool),
    SRC(String),
    TIP(String),
    TYPE(ScriptType),
    VALUE(String),
    WIDTH(Distance),
    ONABORT(String),
    ONBLUR(String),
    ONCANCEL(String),
    ONCHANGE(String),
    ONCLICK(String),
    ONCLOSE(String),
    ONFOCUS(String),
    ONINVALID(String),
    ONLOAD(String),
    ONRESIZE(String),
    ONSCROLL(String),
}

impl Attribute {
    ///Converts from a pair of string. Returns Err when it's not attribute.
    pub fn from(a: &str, s: Option<String>) -> Result<Self, Option<String>> {
        match a {
            ACTION => match s {
                Some(s) => Ok(Self::ACTION(s)),
                None => Err(s),
            },
            CLASS => match s {
                Some(s) => Ok(Self::CLASS(s)),
                None => Err(s),
            },
            COLUMN => match s {
                Some(s) => Ok(Self::COLUMN(Points::from_str(&s))),
                None => Err(s),
            },
            DISABLED => match s {
                Some(s) => Ok(Self::DISABLED(to_bool(&s))),
                None => Ok(Self::DISABLED(true)),
            },
            ENCTYPE => match s {
                Some(s) => Ok(Self::ENCTYPE(s)),
                None => Err(s),
            },
            HEIGHT => match s {
                Some(s) => match Distance::from_str(&s) {
                    Some(s) => Ok(Self::HEIGHT(s)),
                    None => Err(Some(s)),
                },
                None => Err(s),
            },
            HIDDEN => match s {
                Some(s) => Ok(Self::HIDDEN(to_bool(&s))),
                None => Ok(Self::HIDDEN(true)),
            },
            HREF => match s {
                Some(s) => Ok(Self::HREF(s)),
                None => Err(s),
            },
            ID => match s {
                Some(s) => Ok(Self::ID(s)),
                None => Err(s),
            },
            LANG => match s {
                Some(s) => Ok(Self::LANG(s)),
                None => Err(s),
            },
            METHOD => match s {
                Some(s) => Ok(Self::METHOD(s)),
                None => Err(s),
            },
            MULTIPLE => match s {
                Some(s) => Ok(Self::MULTIPLE(to_bool(&s))),
                None => Ok(Self::MULTIPLE(true)),
            },
            NAME => match s {
                Some(s) => Ok(Self::NAME(s)),
                None => Err(s),
            },
            ORDINAL => match s {
                Some(s) => Ok(Self::ORDINAL(Ordinal::from_str(&s))),
                None => Err(s),
            },
            POSITION => match s {
                Some(s) => match Coord::from_str(&s) {
                    Some(c) => Ok(Self::POSITION(c)),
                    None => Err(Some(s)),
                },
                None => Err(s),
            },
            READONLY => match s {
                Some(s) => Ok(Self::READONLY(to_bool(&s))),
                None => Ok(Self::READONLY(true)),
            },
            REQUIRED => match s {
                Some(s) => Ok(Self::REQUIRED(to_bool(&s))),
                None => Ok(Self::REQUIRED(true)),
            },
            ROW => match s {
                Some(s) => Ok(Self::ROW(Points::from_str(&s))),
                None => Err(s),
            },
            SELECTED => match s {
                Some(s) => Ok(Self::SELECTED(to_bool(&s))),
                None => Ok(Self::SELECTED(true)),
            },
            SRC => match s {
                Some(s) => Ok(Self::SRC(s)),
                None => Err(s),
            },
            TIP => match s {
                Some(s) => Ok(Self::TIP(s)),
                None => Err(s),
            },
            TYPE => match s {
                Some(s) => match ScriptType::from_str(&s) {
                    Some(t) => Ok(Self::TYPE(t)),
                    None => Err(Some(s)),
                },
                None => Err(s),
            },
            VALUE => match s {
                Some(s) => Ok(Self::VALUE(s)),
                None => Err(s),
            },
            WIDTH => match s {
                Some(s) => match Distance::from_str(&s) {
                    Some(s) => Ok(Self::WIDTH(s)),
                    None => Err(Some(s)),
                },
                None => Err(s),
            },
            ONABORT => match s {
                Some(s) => Ok(Self::ONABORT(s)),
                None => Err(s),
            },
            ONBLUR => match s {
                Some(s) => Ok(Self::ONBLUR(s)),
                None => Err(s),
            },
            ONCANCEL => match s {
                Some(s) => Ok(Self::ONCANCEL(s)),
                None => Err(s),
            },
            ONCHANGE => match s {
                Some(s) => Ok(Self::ONCHANGE(s)),
                None => Err(s),
            },
            ONCLICK => match s {
                Some(s) => Ok(Self::ONCLICK(s)),
                None => Err(s),
            },
            ONCLOSE => match s {
                Some(s) => Ok(Self::ONCLOSE(s)),
                None => Err(s),
            },
            ONFOCUS => match s {
                Some(s) => Ok(Self::ONFOCUS(s)),
                None => Err(s),
            },
            ONINVALID => match s {
                Some(s) => Ok(Self::ONINVALID(s)),
                None => Err(s),
            },
            ONLOAD => match s {
                Some(s) => Ok(Self::ONLOAD(s)),
                None => Err(s),
            },
            ONRESIZE => match s {
                Some(s) => Ok(Self::ONRESIZE(s)),
                None => Err(s),
            },
            ONSCROLL => match s {
                Some(s) => Ok(Self::ONSCROLL(s)),
                None => Err(s),
            },
            _ => Err(s),
        }
    }

    pub fn name(&self) -> AttrName {
        match self {
            Self::ACTION(_) => AttrName::ACTION,
            Self::CLASS(_) => AttrName::CLASS,
            Self::COLUMN(_) => AttrName::COLUMN,
            Self::DISABLED(_) => AttrName::DISABLED,
            Self::ENCTYPE(_) => AttrName::ENCTYPE,
            Self::HEIGHT(_) => AttrName::HEIGHT,
            Self::HIDDEN(_) => AttrName::HIDDEN,
            Self::HREF(_) => AttrName::HREF,
            Self::ID(_) => AttrName::ID,
            Self::LANG(_) => AttrName::LANG,
            Self::METHOD(_) => AttrName::METHOD,
            Self::MULTIPLE(_) => AttrName::MULTIPLE,
            Self::NAME(_) => AttrName::NAME,
            Self::ORDINAL(_) => AttrName::ORDINAL,
            Self::POSITION(_) => AttrName::POSITION,
            Self::READONLY(_) => AttrName::READONLY,
            Self::REQUIRED(_) => AttrName::REQUIRED,
            Self::ROW(_) => AttrName::ROW,
            Self::SELECTED(_) => AttrName::SELECTED,
            Self::SRC(_) => AttrName::SRC,
            Self::TIP(_) => AttrName::TIP,
            Self::TYPE(_) => AttrName::TYPE,
            Self::VALUE(_) => AttrName::VALUE,
            Self::WIDTH(_) => AttrName::WIDTH,
            Self::ONABORT(_) => AttrName::ONABORT,
            Self::ONBLUR(_) => AttrName::ONBLUR,
            Self::ONCANCEL(_) => AttrName::ONCANCEL,
            Self::ONCHANGE(_) => AttrName::ONCHANGE,
            Self::ONCLICK(_) => AttrName::ONCLICK,
            Self::ONCLOSE(_) => AttrName::ONCLOSE,
            Self::ONFOCUS(_) => AttrName::ONFOCUS,
            Self::ONINVALID(_) => AttrName::ONINVALID,
            Self::ONLOAD(_) => AttrName::ONLOAD,
            Self::ONRESIZE(_) => AttrName::ONRESIZE,
            Self::ONSCROLL(_) => AttrName::ONSCROLL,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Attribute::ACTION(s) => s.to_string(),
            Attribute::CLASS(s) => s.to_string(),
            Attribute::COLUMN(s) => s.to_string(),
            Attribute::DISABLED(s) => s.to_string(),
            Attribute::ENCTYPE(s) => s.to_string(),
            Attribute::HEIGHT(s) => s.to_string(),
            Attribute::HIDDEN(s) => s.to_string(),
            Attribute::HREF(s) => s.to_string(),
            Attribute::ID(s) => s.to_string(),
            Attribute::LANG(s) => s.to_string(),
            Attribute::METHOD(s) => s.to_string(),
            Attribute::MULTIPLE(s) => s.to_string(),
            Attribute::NAME(s) => s.to_string(),
            Attribute::ORDINAL(s) => s.to_string(),
            Attribute::POSITION(s) => s.to_string(),
            Attribute::READONLY(s) => s.to_string(),
            Attribute::REQUIRED(s) => s.to_string(),
            Attribute::ROW(s) => s.to_string(),
            Attribute::SELECTED(s) => s.to_string(),
            Attribute::SRC(s) => s.to_string(),
            Attribute::TIP(s) => s.to_string(),
            Attribute::TYPE(s) => s.to_string(),
            Attribute::VALUE(s) => s.to_string(),
            Attribute::WIDTH(s) => s.to_string(),
            Attribute::ONABORT(s) => s.to_string(),
            Attribute::ONBLUR(s) => s.to_string(),
            Attribute::ONCANCEL(s) => s.to_string(),
            Attribute::ONCHANGE(s) => s.to_string(),
            Attribute::ONCLICK(s) => s.to_string(),
            Attribute::ONCLOSE(s) => s.to_string(),
            Attribute::ONFOCUS(s) => s.to_string(),
            Attribute::ONINVALID(s) => s.to_string(),
            Attribute::ONLOAD(s) => s.to_string(),
            Attribute::ONRESIZE(s) => s.to_string(),
            Attribute::ONSCROLL(s) => s.to_string(),
        }
    }
}

pub(crate) struct AttributeHolder(HashMap<AttrName, Attribute>);

impl AttributeHolder {
    fn new() -> Self {
        Self(HashMap::new())
    }
}

impl Deref for AttributeHolder {
    type Target = HashMap<AttrName, Attribute>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AttributeHolder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Debug for AttributeHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(&self.0).finish()
    }
}

pub(crate) struct ElementHolder(VecDeque<Arc<RwLock<Element>>>);

impl ElementHolder {
    fn new() -> Self {
        Self(VecDeque::new())
    }
}

impl Deref for ElementHolder {
    type Target = VecDeque<Arc<RwLock<Element>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ElementHolder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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

///Represents element.
pub struct Element {
    pub(crate) mark_type: Mark,
    pub(crate) text: String,
    pub(crate) attribute: AttributeHolder,
    pub(crate) subset: ElementHolder,
    pub(crate) upper: Option<Arc<RwLock<Element>>>,
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
            attribute: AttributeHolder::new(),
            subset: ElementHolder::new(),
            upper: None,
        }
    }

    ///Parse a string slice to many.
    pub fn many_from_str(buf: &str) -> VecDeque<Self> {
        format::accept(buf)
    }

    ///Parse a string slice to it.
    pub fn from_str(buf: &str) -> Option<Self> {
        format::accept(buf).pop_front()
    }

    ///Returns a string slice of this element's type.
    pub fn as_str(&self) -> &str {
        self.mark_type.as_str()
    }

    pub(crate) fn set_upper(
        &mut self,
        upper_arc: Arc<RwLock<Element>>,
        self_arc: Arc<RwLock<Element>>,
    ) {
        self.upper.replace(upper_arc);
        self.subset.iter_mut().for_each(|o| {
            if let Ok(mut e) = o.write() {
                e.set_upper(self_arc.clone(), o.clone());
            }
        });
    }

    pub(crate) fn subset_upper(&mut self) {
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

    fn equal_mark(&self, n: usize, mark_type: Mark) -> bool {
        if let Some(o) = self.subset.get(n) {
            if let Ok(e) = o.read() {
                if e.mark_type == mark_type {
                    return true;
                }
            }
        }
        false
    }

    fn page_element(
        &self,
    ) -> Option<(
        Arc<RwLock<Element>>,
        Arc<RwLock<Element>>,
        Arc<RwLock<Element>>,
        Arc<RwLock<Element>>,
    )> {
        if self.mark_type == Mark::AHT && self.subset.len() >= 4 {
            if self.equal_mark(0, Mark::HEAD)
                && self.equal_mark(1, Mark::BODY)
                && self.equal_mark(2, Mark::STYLE)
                && self.equal_mark(3, Mark::SCRIPT)
            {
                if let Some(head) = self.subset.get(0) {
                    if let Some(body) = self.subset.get(1) {
                        if let Some(style) = self.subset.get(2) {
                            if let Some(script) = self.subset.get(3) {
                                return Some((
                                    head.clone(),
                                    body.clone(),
                                    style.clone(),
                                    script.clone(),
                                ));
                            }
                        }
                    }
                }
            }
        }
        None
    }

    ///Converts to page.
    pub fn to_page(self) -> Result<Page, Self> {
        if let Some((head, body, style, script)) = self.page_element() {
            return Ok(Page::new(self, head, body, style, script));
        }
        Err(self)
    }

    fn subset_swap_remove(&mut self, o: Arc<RwLock<Element>>, a: Element) {
        if let Some(i) = self
            .subset
            .iter()
            .position(|k| Arc::as_ptr(&k) == Arc::as_ptr(&o))
        {
            self.subset.push_back(Arc::new(RwLock::new(a)));
            self.subset.swap_remove_back(i);
        }
    }

    fn subset_find(
        &self,
        self_arc: Arc<RwLock<Element>>,
        o: Arc<RwLock<Element>>,
    ) -> Option<Arc<RwLock<Element>>> {
        self.subset.iter().find_map(|k| {
            if Arc::as_ptr(&k) == Arc::as_ptr(&o) {
                Some(self_arc.clone())
            } else {
                if let Ok(e) = k.read() {
                    e.subset_find(k.clone(), o.clone())
                } else {
                    None
                }
            }
        })
    }
}

//------------------------------------------------------------------------------------------

///Represents page.
pub struct Page {
    root: Element,
    head_element: Arc<RwLock<Element>>,
    body_element: Arc<RwLock<Element>>,
    style_element: Arc<RwLock<Element>>,
    script_element: Arc<RwLock<Element>>,
    head: Head,
    body: Body,
    style: Style,
    script: Script,
    pub(crate) cursor: VisionAction,
    pub(crate) keyboard_input: Option<Arc<RwLock<String>>>,
}

impl std::fmt::Debug for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Page")
            .field("root", &self.root)
            .field("head", &self.head)
            .field("body", &self.body)
            .field("style", &self.style)
            .field("script", &self.script)
            .finish_non_exhaustive()
    }
}

impl Page {
    fn new(
        root: Element,
        head_element: Arc<RwLock<Element>>,
        body_element: Arc<RwLock<Element>>,
        style_element: Arc<RwLock<Element>>,
        script_element: Arc<RwLock<Element>>,
    ) -> Self {
        let head = Head::new(head_element.clone());
        let body = Body::new(body_element.clone());
        let style = Style::new(style_element.clone());
        let script = Script::new(script_element.clone());
        let mut page = Self {
            root,
            head_element,
            body_element,
            style_element,
            script_element,
            head,
            body,
            style,
            script,
            cursor: VisionAction::new(),
            keyboard_input: None,
        };

        let p = &mut page as *mut Self;
        page.style.build(unsafe { &mut *p });
        page.script.run(unsafe { &mut *p });
        page
    }

    ///Parse a string slice.
    pub fn from_str(buf: &str) -> Option<Self> {
        Element::from_str(buf)?.to_page().ok()
    }

    pub(crate) fn body_element(&self) -> Arc<RwLock<Element>> {
        self.body_element.clone()
    }

    pub(crate) fn body(&mut self) -> &mut Body {
        &mut self.body
    }

    pub fn set_zero(&mut self, x: isize, y: isize) {
        self.body.set_zero(x, y);
    }

    pub fn resize(&mut self, width: isize, height: isize) {
        self.body.resize(width, height);
    }

    ///Returns true if draw.
    pub fn draw(&mut self, canvas: &Canvas) -> bool {
        let p = self as *mut Self;
        self.body.draw(canvas, unsafe { &mut *p });
        false
    }

    pub fn find_mark(&mut self, s: Mark) -> Vec<Arc<RwLock<Element>>> {
        self.find(Conditions::new_with_mark(s))
    }

    pub fn find_class(&mut self, s: &str) -> Vec<Arc<RwLock<Element>>> {
        self.find(Conditions::new_with_class(s))
    }

    pub fn find_id(&mut self, s: &str) -> Option<Arc<RwLock<Element>>> {
        let mut v = self.find(Conditions::new_with_id(s));
        if v.is_empty() {
            None
        } else {
            Some(v.remove(0))
        }
    }

    ///Find `Element` references by `Conditions`.
    pub fn find(&mut self, conditions: Conditions) -> Vec<Arc<RwLock<Element>>> {
        let v = vec![self.head_element.clone(), self.body_element.clone()];
        find_elements(v, conditions)
    }

    pub fn find_in_body(&mut self, conditions: Conditions) -> Vec<Arc<RwLock<Element>>> {
        let v = vec![self.body_element()];
        find_elements(v, conditions)
    }

    pub fn set_cursor(&mut self, p: VisionPosition) {
        self.cursor.add(p);
    }

    pub fn keyboard_input(&mut self, s: &str) {
        if let Some(o) = &mut self.keyboard_input {
            if let Ok(mut o) = o.try_write() {
                o.push_str(s);
            }
        }
    }
}

pub(crate) fn find_elements(
    mut v: Vec<Arc<RwLock<Element>>>,
    conditions: Conditions,
) -> Vec<Arc<RwLock<Element>>> {
    for c in conditions.get() {
        let mut r = Vec::new();
        match c {
            Condition::MARK(s) => {
                v.into_iter().for_each(|t| find_mark(t, &s, &mut r));
            }
            Condition::CLASS(s) => {
                v.into_iter().for_each(|t| find_class(t, &s, &mut r));
            }
            Condition::ID(s) => {
                for t in v {
                    if let Some(p) = find_id(t, &s) {
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
        if e.mark_type == *s {
            v.push(o.clone())
        }
        for k in e.subset.iter() {
            find_mark(k.clone(), s, v)
        }
    }
}

fn find_class(o: Arc<RwLock<Element>>, s: &str, v: &mut Vec<Arc<RwLock<Element>>>) {
    if let Ok(e) = o.read() {
        if let Some(Attribute::CLASS(k)) = e.attribute.get(&AttrName::CLASS) {
            if k == s {
                v.push(o.clone());
            }
        }
        for k in e.subset.iter() {
            find_class(k.clone(), s, v)
        }
    }
}

fn find_id(o: Arc<RwLock<Element>>, s: &str) -> Option<Arc<RwLock<Element>>> {
    if let Ok(e) = o.read() {
        if let Some(Attribute::ID(k)) = e.attribute.get(&AttrName::ID) {
            if k == s {
                return Some(o.clone());
            }
        }
        for k in e.subset.iter() {
            let t = find_id(k.clone(), s);
            if t.is_some() {
                return t;
            }
        }
    }
    None
}

#[derive(Debug)]
enum Condition {
    MARK(Mark),
    CLASS(String),
    ID(String),
}

///Represents find conditions.
#[derive(Debug)]
pub struct Conditions(Vec<Condition>);

impl Conditions {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn new_with_mark(s: Mark) -> Self {
        let mut c = Self::new();
        c.mark(s);
        c
    }

    pub fn new_with_class(s: impl Into<String>) -> Self {
        let mut c = Self::new();
        c.class(s);
        c
    }

    pub fn new_with_id(s: impl Into<String>) -> Self {
        let mut c = Self::new();
        c.id(s);
        c
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn get(&self) -> &Vec<Condition> {
        &self.0
    }

    pub fn mark(&mut self, s: Mark) {
        self.0.push(Condition::MARK(s));
    }

    pub fn class(&mut self, s: impl Into<String>) {
        self.0.push(Condition::CLASS(s.into()))
    }

    pub fn id(&mut self, s: impl Into<String>) {
        self.0.push(Condition::ID(s.into()))
    }

    pub fn reverse(&mut self) {
        self.0.reverse()
    }
}

///Represents cursor action.
#[derive(Clone, Debug, PartialEq)]
pub enum CursorAction {
    Down(usize),
    Up,
}

///Represents cursor position and action.
#[derive(Debug)]
pub struct VisionPosition {
    xy: Coord2D,
    action: CursorAction,
}

impl VisionPosition {
    pub fn new(xy: Coord2D, action: CursorAction) -> Self {
        Self { xy, action }
    }

    pub fn with_xy(x: isize, y: isize, action: CursorAction) -> Self {
        Self::new(Coord2D::xy(x, y), action)
    }

    pub fn with_xy_up(x: isize, y: isize) -> Self {
        Self::with_xy(x, y, CursorAction::Up)
    }
}

#[derive(Debug)]
pub(crate) struct VisionAction {
    a: Option<VisionPosition>,
    b: Option<VisionPosition>,
    f: bool,
}

impl VisionAction {
    fn new() -> Self {
        Self {
            a: None,
            b: None,
            f: true,
        }
    }

    fn add(&mut self, v: VisionPosition) {
        if self.f {
            self.b.replace(v);
            self.f = false;
        } else {
            self.a.replace(v);
            self.f = true;
        }
    }

    fn ordered(&self) -> (&Option<VisionPosition>, &Option<VisionPosition>) {
        if self.f {
            (&self.a, &self.b)
        } else {
            (&self.b, &self.a)
        }
    }

    pub(crate) fn position(&self) -> Option<&Coord2D> {
        let (fir, _) = self.ordered();
        if let Some(fir) = fir {
            return Some(&fir.xy);
        }
        None
    }

    pub(crate) fn analyse(&self) -> Option<(&Coord2D, VisionActionResult)> {
        let (fir, sec) = self.ordered();
        if let Some(fir) = fir {
            match fir.action {
                CursorAction::Down(n) => {
                    if let Some(sec) = sec {
                        if let CursorAction::Down(_) = sec.action {
                            return Some((
                                &fir.xy,
                                VisionActionResult::PressSweep(fir.xy.away_from(&sec.xy)),
                            ));
                        }
                    }
                    return Some((&fir.xy, VisionActionResult::Press(n)));
                }
                CursorAction::Up => {
                    if let Some(sec) = sec {
                        if sec.action == CursorAction::Up {
                            return Some((
                                &fir.xy,
                                VisionActionResult::Sweep(fir.xy.away_from(&sec.xy)),
                            ));
                        }
                    }
                    return Some((&fir.xy, VisionActionResult::Loosen));
                }
            }
        }
        None
    }
}

#[derive(Debug)]
pub(crate) enum VisionActionResult {
    Press(usize),
    Sweep(RectSide),
    PressSweep(RectSide),
    Loosen,
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
        if let Some(e) = Element::from_str(&s) {
            println!("{:?}", e);
        }

        if let Some(e) = Page::from_str(&s) {
            println!("{:?}", e);
        }
    }
}
