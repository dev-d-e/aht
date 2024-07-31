mod format;

use crate::content::{Audio, Button, Canv, Form, Iframe, Img, Inp, Opt, Pt, Select, Time, Video};
use crate::css::Css;
use crate::grid::{Area, Body};
use crate::head::{Aht, Head, Title};
use crate::parts::{Distance, Ordinal, Points};
use crate::script::Script;
use crate::utils::to_bool;
use skia_safe::Canvas;
use std::collections::VecDeque;
use std::fmt::Debug;

pub(crate) const AHT: &str = "aht";
pub(crate) const AREA: &str = "area";
pub(crate) const AUDIO: &str = "audio";
pub(crate) const BODY: &str = "body";
pub(crate) const BUTTON: &str = "button";
pub(crate) const CANVAS: &str = "canvas";
pub(crate) const CSS: &str = "css";
pub(crate) const FORM: &str = "form";
pub(crate) const HEAD: &str = "head";
pub(crate) const IFRAME: &str = "iframe";
pub(crate) const IMG: &str = "img";
pub(crate) const INP: &str = "inp";
pub(crate) const OPTION: &str = "option";
pub(crate) const PT: &str = "pt";
pub(crate) const SCRIPT: &str = "script";
pub(crate) const SELECT: &str = "select";
pub(crate) const TIME: &str = "time";
pub(crate) const TITLE: &str = "title";
pub(crate) const VIDEO: &str = "video";

///Mark.
#[derive(Debug, PartialEq)]
pub enum Mark {
    AHT,
    AREA,
    AUDIO,
    BODY,
    BUTTON,
    CANVAS,
    CSS,
    FORM,
    HEAD,
    IFRAME,
    IMG,
    INP,
    OPTION,
    PT,
    SCRIPT,
    SELECT,
    TIME,
    TITLE,
    VIDEO,
}

impl Mark {
    ///Converts a string slice to `Mark`.
    pub fn from(s: String) -> Result<Self, String> {
        match s.as_str() {
            AHT => Ok(Self::AHT),
            AREA => Ok(Self::AREA),
            AUDIO => Ok(Self::AUDIO),
            BODY => Ok(Self::BODY),
            BUTTON => Ok(Self::BUTTON),
            CANVAS => Ok(Self::CANVAS),
            CSS => Ok(Self::CSS),
            FORM => Ok(Self::FORM),
            HEAD => Ok(Self::HEAD),
            IFRAME => Ok(Self::IFRAME),
            IMG => Ok(Self::IMG),
            INP => Ok(Self::INP),
            OPTION => Ok(Self::OPTION),
            PT => Ok(Self::PT),
            SCRIPT => Ok(Self::SCRIPT),
            SELECT => Ok(Self::SELECT),
            TIME => Ok(Self::TIME),
            TITLE => Ok(Self::TITLE),
            VIDEO => Ok(Self::VIDEO),
            _ => Err(s),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Mark::AHT => AHT,
            Mark::AREA => AREA,
            Mark::AUDIO => AUDIO,
            Mark::BODY => BODY,
            Mark::BUTTON => BUTTON,
            Mark::CANVAS => CANVAS,
            Mark::CSS => CSS,
            Mark::FORM => FORM,
            Mark::HEAD => HEAD,
            Mark::IFRAME => IFRAME,
            Mark::IMG => IMG,
            Mark::INP => INP,
            Mark::OPTION => OPTION,
            Mark::PT => PT,
            Mark::SCRIPT => SCRIPT,
            Mark::SELECT => SELECT,
            Mark::TIME => TIME,
            Mark::TITLE => TITLE,
            Mark::VIDEO => VIDEO,
        }
    }
}

///TypeEntity.
#[derive(Debug)]
pub enum TypeEntity {
    AHT(Aht),
    AREA(Area),
    AUDIO(Audio),
    BODY(Body),
    BUTTON(Button),
    CANVAS(Canv),
    CSS(Css),
    FORM(Form),
    HEAD(Head),
    IFRAME(Iframe),
    IMG(Img),
    INP(Inp),
    OPTION(Opt),
    PT(Pt),
    SCRIPT(Script),
    SELECT(Select),
    TIME(Time),
    TITLE(Title),
    VIDEO(Video),
}

impl TypeEntity {
    fn from(o: ValidElement) -> Self {
        let (n, s, mut attribute, mut subset) = o.take();

        macro_rules! to_type {
            ($t:ty) => {{
                let mut o = <$t>::new();
                o.text = s;
                attribute.drain(..).for_each(|a| o.attr(a));
                subset
                    .drain(..)
                    .for_each(|sub| o.subset.push_back(TypeEntity::from(sub)));
                o
            }};
        }
        macro_rules! to_type1 {
            ($t:ty) => {{
                let mut o = <$t>::new();
                attribute.drain(..).for_each(|a| o.attr(a));
                subset
                    .drain(..)
                    .for_each(|sub| o.subset.push_back(TypeEntity::from(sub)));
                o
            }};
        }
        macro_rules! to_type2 {
            ($t:ty) => {{
                let mut o = <$t>::new();
                o.text = s;
                attribute.drain(..).for_each(|a| o.attr(a));
                o
            }};
        }

        match n {
            Mark::AHT => Self::AHT(to_type1!(Aht)),
            Mark::AREA => Self::AREA(to_type!(Area)),
            Mark::AUDIO => Self::AUDIO(to_type!(Audio)),
            Mark::BODY => Self::BODY(to_type!(Body)),
            Mark::BUTTON => Self::BUTTON(to_type!(Button)),
            Mark::CANVAS => Self::CANVAS(to_type!(Canv)),
            Mark::CSS => Self::CSS(to_type!(Css)),
            Mark::FORM => Self::FORM(to_type!(Form)),
            Mark::HEAD => Self::HEAD(to_type!(Head)),
            Mark::IFRAME => Self::IFRAME(to_type!(Iframe)),
            Mark::IMG => Self::IMG(to_type!(Img)),
            Mark::INP => Self::INP(to_type!(Inp)),
            Mark::OPTION => Self::OPTION(to_type2!(Opt)),
            Mark::PT => Self::PT(to_type!(Pt)),
            Mark::SCRIPT => Self::SCRIPT(to_type!(Script)),
            Mark::SELECT => Self::SELECT(to_type!(Select)),
            Mark::TIME => Self::TIME(to_type!(Time)),
            Mark::TITLE => Self::TITLE(to_type!(Title)),
            Mark::VIDEO => Self::VIDEO(to_type!(Video)),
        }
    }

    ///Parse a string slice to `TypeEntity`.
    pub fn from_str(buf: &str) -> VecDeque<Self> {
        format::accept(buf)
    }

    ///Parse to `Page`.
    pub fn to_page(self) -> Result<Page, Self> {
        if let Self::AHT(o) = self {
            Ok(Page::new(o.take()))
        } else {
            Err(self)
        }
    }

    fn find_mark(&mut self, s: &str, v: &mut Vec<&mut TypeEntity>) {
        let t = self as *mut TypeEntity;

        macro_rules! find {
            ($o:ident) => {{
                if $o.element() == s {
                    unsafe { v.push(&mut *t) }
                }
                for e in &mut $o.subset.vec {
                    e.find_mark(s, v)
                }
            }};
        }
        match self {
            TypeEntity::AHT(o) => find!(o),
            TypeEntity::AREA(o) => find!(o),
            TypeEntity::AUDIO(o) => find!(o),
            TypeEntity::BODY(o) => find!(o),
            TypeEntity::BUTTON(o) => find!(o),
            TypeEntity::CANVAS(o) => find!(o),
            TypeEntity::CSS(o) => find!(o),
            TypeEntity::FORM(o) => find!(o),
            TypeEntity::HEAD(o) => find!(o),
            TypeEntity::IFRAME(o) => find!(o),
            TypeEntity::IMG(o) => find!(o),
            TypeEntity::INP(o) => find!(o),
            TypeEntity::OPTION(_) => {}
            TypeEntity::PT(o) => find!(o),
            TypeEntity::SCRIPT(o) => find!(o),
            TypeEntity::SELECT(o) => find!(o),
            TypeEntity::TIME(o) => find!(o),
            TypeEntity::TITLE(o) => find!(o),
            TypeEntity::VIDEO(o) => find!(o),
        }
    }

    fn find_class(&mut self, s: &str, v: &mut Vec<&mut TypeEntity>) {
        let t = self as *mut TypeEntity;
        macro_rules! find {
            ($o:ident) => {{
                if $o.class == s {
                    unsafe { v.push(&mut *t) }
                }
                for e in &mut $o.subset.vec {
                    e.find_class(s, v)
                }
            }};
        }
        match self {
            TypeEntity::AHT(o) => find!(o),
            TypeEntity::AREA(o) => find!(o),
            TypeEntity::AUDIO(o) => find!(o),
            TypeEntity::BODY(o) => find!(o),
            TypeEntity::BUTTON(o) => find!(o),
            TypeEntity::CANVAS(o) => find!(o),
            TypeEntity::CSS(o) => find!(o),
            TypeEntity::FORM(o) => find!(o),
            TypeEntity::HEAD(o) => find!(o),
            TypeEntity::IFRAME(o) => find!(o),
            TypeEntity::IMG(o) => find!(o),
            TypeEntity::INP(o) => find!(o),
            TypeEntity::OPTION(_) => {}
            TypeEntity::PT(o) => find!(o),
            TypeEntity::SCRIPT(o) => find!(o),
            TypeEntity::SELECT(o) => find!(o),
            TypeEntity::TIME(o) => find!(o),
            TypeEntity::TITLE(o) => find!(o),
            TypeEntity::VIDEO(o) => find!(o),
        }
    }

    fn find_id(&mut self, s: &str) -> Option<&mut TypeEntity> {
        let t = self as *mut TypeEntity;
        macro_rules! find {
            ($o:ident) => {{
                if $o.id == s {
                    unsafe { Some(&mut *t) }
                } else {
                    for e in &mut $o.subset.vec {
                        let t = e.find_id(s);
                        if t.is_some() {
                            return t;
                        }
                    }
                    None
                }
            }};
        }
        match self {
            TypeEntity::AHT(o) => find!(o),
            TypeEntity::AREA(o) => find!(o),
            TypeEntity::AUDIO(o) => find!(o),
            TypeEntity::BODY(o) => find!(o),
            TypeEntity::BUTTON(o) => find!(o),
            TypeEntity::CANVAS(o) => find!(o),
            TypeEntity::CSS(o) => find!(o),
            TypeEntity::FORM(o) => find!(o),
            TypeEntity::HEAD(o) => find!(o),
            TypeEntity::IFRAME(o) => find!(o),
            TypeEntity::IMG(o) => find!(o),
            TypeEntity::INP(o) => find!(o),
            TypeEntity::OPTION(_) => None,
            TypeEntity::PT(o) => find!(o),
            TypeEntity::SCRIPT(o) => find!(o),
            TypeEntity::SELECT(o) => find!(o),
            TypeEntity::TIME(o) => find!(o),
            TypeEntity::TITLE(o) => find!(o),
            TypeEntity::VIDEO(o) => find!(o),
        }
    }
}

const ACTION: &str = "action";
const ASYNCHRONOUS: &str = "async";
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
const NAME: &str = "name ";
const ORDINAL: &str = "ordinal";
const READONLY: &str = "readonly";
const REQUIRED: &str = "required";
const ROW: &str = "row";
const SELECTED: &str = "selected";
const SRC: &str = "src";
const TIP: &str = "tip";
const VALUE: &str = "value ";
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

///Attribute.
#[derive(Debug)]
pub enum Attribute {
    ACTION(String),
    ASYNCHRONOUS(bool),
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
    READONLY(bool),
    REQUIRED(bool),
    ROW(Points),
    SELECTED(bool),
    SRC(String),
    TIP(String),
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
    ///Converts a pair of string slice to an `Attribute`.
    pub fn from(a: &str, s: Option<String>) -> Result<Self, Option<String>> {
        match a {
            ACTION => match s {
                Some(s) => Ok(Self::ACTION(s)),
                None => Err(s),
            },
            ASYNCHRONOUS => match s {
                Some(s) => Ok(Self::ASYNCHRONOUS(to_bool(&s))),
                None => Ok(Self::ASYNCHRONOUS(true)),
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
}

#[derive(Debug)]
struct ValidElement {
    mark_type: Mark,
    text: String,
    attribute: Vec<Attribute>,
    subset: Vec<ValidElement>,
}

impl ValidElement {
    fn new(mark_type: Mark, text: String) -> Self {
        ValidElement {
            mark_type,
            text,
            attribute: Vec::new(),
            subset: Vec::new(),
        }
    }

    fn take(self) -> (Mark, String, Vec<Attribute>, Vec<ValidElement>) {
        (self.mark_type, self.text, self.attribute, self.subset)
    }
}

///"Page" represents page.
#[derive(Debug)]
pub struct Page {
    head: Option<Box<TypeEntity>>,
    body: Option<Box<TypeEntity>>,
    css: Option<Box<TypeEntity>>,
    script: Option<Box<TypeEntity>>,
}

impl Page {
    fn new(
        t: (
            Option<TypeEntity>,
            Option<TypeEntity>,
            Option<TypeEntity>,
            Option<TypeEntity>,
        ),
    ) -> Self {
        let mut page = Page {
            head: t.0.map(|o| Box::new(o)),
            body: t.1.map(|o| Box::new(o)),
            css: t.2.map(|o| Box::new(o)),
            script: t.3.map(|o| Box::new(o)),
        };
        let p = &mut page as *mut Page;
        if let Some(t) = &mut page.body {
            let t = t.as_mut();
            let o = t as *mut TypeEntity;
            if let TypeEntity::BODY(body) = t {
                unsafe {
                    body.set_parent(&mut *p, &mut *o);
                }
            }
        }
        page
    }

    ///Parse a string slice to `Page`.
    pub fn from_str(buf: &str) -> Option<Result<Self, TypeEntity>> {
        Some(TypeEntity::from_str(buf).pop_front()?.to_page())
    }

    ///Returns true if the `Page` doesn't contain `Head` or `Body` or `Css` or `Script`.
    pub fn incomplete(&self) -> bool {
        self.head.is_none() || self.body.is_none() || self.css.is_none() || self.script.is_none()
    }

    fn body(&mut self, f: impl Fn(&mut Body)) -> bool {
        if let Some(t) = &mut self.body {
            if let TypeEntity::BODY(body) = t.as_mut() {
                f(body);
                return true;
            }
        }
        false
    }

    pub fn set_zero(&mut self, x: isize, y: isize) {
        self.body(|o| o.set_zero(x, y));
    }

    pub fn resize(&mut self, width: isize, height: isize) {
        self.body(|o| o.resize(width, height));
    }

    ///Returns true if draw.
    pub fn draw(&mut self, canvas: &Canvas) -> bool {
        self.body(|o| o.draw(canvas))
    }

    pub fn find_mark(&mut self, s: Mark) -> Vec<&mut TypeEntity> {
        self.find(Conditions::new_with_mark(s))
    }

    pub fn find_class(&mut self, s: &str) -> Vec<&mut TypeEntity> {
        self.find(Conditions::new_with_class(s))
    }

    pub fn find_id(&mut self, s: &str) -> Option<&mut TypeEntity> {
        let mut v = self.find(Conditions::new_with_id(s));
        if v.is_empty() {
            None
        } else {
            Some(v.remove(0))
        }
    }

    fn head_body(&mut self) -> Vec<&mut TypeEntity> {
        let mut v = Vec::new();
        if let Some(t) = &mut self.head {
            v.push(t.as_mut())
        }
        if let Some(t) = &mut self.body {
            v.push(t.as_mut())
        }
        v
    }

    ///Find `TypeEntity` references by `Conditions`.
    pub fn find(&mut self, conditions: Conditions) -> Vec<&mut TypeEntity> {
        let mut v = self.head_body();
        for c in conditions.v {
            let mut r = Vec::new();
            match c {
                Condition::MARK(s) => {
                    v.into_iter().for_each(|t| t.find_mark(s.as_str(), &mut r));
                }
                Condition::CLASS(s) => {
                    v.into_iter().for_each(|t| t.find_class(&s, &mut r));
                }
                Condition::ID(s) => {
                    for t in v {
                        if let Some(p) = t.find_id(&s) {
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
}

enum Condition {
    MARK(Mark),
    CLASS(String),
    ID(String),
}

///find conditions.
pub struct Conditions {
    v: Vec<Condition>,
}

impl Conditions {
    fn new() -> Self {
        Self { v: Vec::new() }
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

    pub fn mark(&mut self, s: Mark) {
        self.v.push(Condition::MARK(s));
    }

    pub fn class(&mut self, s: impl Into<String>) {
        self.v.push(Condition::CLASS(s.into()))
    }

    pub fn id(&mut self, s: impl Into<String>) {
        self.v.push(Condition::ID(s.into()))
    }
}
