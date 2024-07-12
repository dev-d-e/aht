mod format;

use crate::content::{Audio, Button, Canv, Form, Iframe, Img, Inp, Opt, Pt, Select, Time, Video};
use crate::css::Css;
use crate::grid::{Area, Body, Dialog};
use crate::head::{Head, Title};
use crate::parts::{Ordinal, Points};
use crate::script::Script;
use crate::utils::{to_bool, to_isize};
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
pub(crate) const DIALOG: &str = "dialog";
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
    DIALOG,
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
            DIALOG => Ok(Self::DIALOG),
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
}

///TypeEntity.
#[derive(Debug)]
pub enum TypeEntity {
    PAGE(Page),
    AREA(Area),
    AUDIO(Audio),
    BODY(Body),
    BUTTON(Button),
    CANVAS(Canv),
    CSS(Css),
    DIALOG(Dialog),
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
        match n {
            Mark::AHT => Self::PAGE(Page::new(subset.drain(..).map(|o| Self::from(o)).collect())),
            Mark::AREA => Self::AREA(to_type!(Area, s, attribute, subset)),
            Mark::AUDIO => Self::AUDIO(to_type!(Audio, s, attribute, subset)),
            Mark::BODY => Self::BODY(to_type!(Body, s, attribute, subset)),
            Mark::BUTTON => Self::BUTTON(to_type!(Button, s, attribute, subset)),
            Mark::CANVAS => Self::CANVAS(to_type!(Canv, s, attribute, subset)),
            Mark::CSS => Self::CSS(to_type!(Css, s, attribute, subset)),
            Mark::DIALOG => Self::DIALOG(to_type!(Dialog, s, attribute, subset)),
            Mark::FORM => Self::FORM(to_type!(Form, s, attribute, subset)),
            Mark::HEAD => Self::HEAD(to_type!(Head, s, attribute, subset)),
            Mark::IFRAME => Self::IFRAME(to_type!(Iframe, s, attribute, subset)),
            Mark::IMG => Self::IMG(to_type!(Img, s, attribute, subset)),
            Mark::INP => Self::INP(to_type!(Inp, s, attribute, subset)),
            Mark::OPTION => Self::OPTION(to_type!(Opt, s, attribute)),
            Mark::PT => Self::PT(to_type!(Pt, s, attribute, subset)),
            Mark::SCRIPT => Self::SCRIPT(to_type!(Script, s, attribute, subset)),
            Mark::SELECT => Self::SELECT(to_type!(Select, s, attribute, subset)),
            Mark::TIME => Self::TIME(to_type!(Time, s, attribute, subset)),
            Mark::TITLE => Self::TITLE(to_type!(Title, s, attribute, subset)),
            Mark::VIDEO => Self::VIDEO(to_type!(Video, s, attribute, subset)),
        }
    }

    ///Parse a string slice to `TypeEntity`.
    pub fn from_str(buf: &str) -> Option<Self> {
        format::accept(buf).map(|o| Self::from(o))
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
const METHOD: &str = "method";
const MULTIPLE: &str = "multiple";
const NAME: &str = "name ";
const ORDINAL: &str = "ordinal ";
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
    HEIGHT(isize),
    HIDDEN(bool),
    HREF(String),
    ID(String),
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
    WIDTH(isize),
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
                Some(s) => match to_isize(&s) {
                    Some(i) => Ok(Self::HEIGHT(i)),
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
                Some(s) => match to_isize(&s) {
                    Some(i) => Ok(Self::WIDTH(i)),
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
    head: Head,
    body: Body,
    css: Css,
    script: Script,
}

impl Page {
    pub(crate) fn new(vd: VecDeque<TypeEntity>) -> Self {
        let mut head = Head::new();
        let mut body = Body::new();
        let mut css = Css::new();
        let mut script = Script::new();
        for (i, o) in vd.into_iter().enumerate() {
            match o {
                TypeEntity::HEAD(t) => {
                    if i == 0 {
                        head = t;
                    }
                }
                TypeEntity::BODY(t) => {
                    if i == 1 {
                        body = t;
                    }
                }
                TypeEntity::CSS(t) => {
                    if i == 2 {
                        css = t;
                    }
                }
                TypeEntity::SCRIPT(t) => {
                    if i == 3 {
                        script = t;
                    }
                }
                _ => {}
            }
        }
        Page {
            head,
            body,
            css,
            script,
        }
    }

    ///Parse a string slice to `Page`.
    pub fn from_str(buf: &str) -> Option<Self> {
        match TypeEntity::from_str(buf)? {
            TypeEntity::PAGE(p) => Some(p),
            _ => None,
        }
    }

    pub(crate) fn draw(&mut self, canvas: &Canvas) {
        self.body.draw(canvas);
    }
}
