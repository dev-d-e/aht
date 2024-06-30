mod format;

use crate::content::{Audio, Button, Canv, Form, Iframe, Img, Inp, Opt, Pt, Select, Time, Video};
use crate::css::Css;
use crate::grid::{Area, Body, Dialog};
use crate::head::{Head, Title};
use crate::script::Script;
use crate::utils::{to_bool, to_i32};
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
    pub fn from(s: String) -> Result<Mark, String> {
        match s.as_str() {
            AHT => Ok(Mark::AHT),
            AREA => Ok(Mark::AREA),
            AUDIO => Ok(Mark::AUDIO),
            BODY => Ok(Mark::BODY),
            BUTTON => Ok(Mark::BUTTON),
            CANVAS => Ok(Mark::CANVAS),
            CSS => Ok(Mark::CSS),
            DIALOG => Ok(Mark::DIALOG),
            FORM => Ok(Mark::FORM),
            HEAD => Ok(Mark::HEAD),
            IFRAME => Ok(Mark::IFRAME),
            IMG => Ok(Mark::IMG),
            INP => Ok(Mark::INP),
            OPTION => Ok(Mark::OPTION),
            PT => Ok(Mark::PT),
            SCRIPT => Ok(Mark::SCRIPT),
            SELECT => Ok(Mark::SELECT),
            TIME => Ok(Mark::TIME),
            TITLE => Ok(Mark::TITLE),
            VIDEO => Ok(Mark::VIDEO),
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
    fn from(o: ValidElement) -> TypeEntity {
        let (n, s, mut attribute, mut subset) = o.take();
        match n {
            Mark::AHT => {
                let vec = subset.drain(..).map(|o| TypeEntity::from(o)).collect();
                TypeEntity::PAGE(Page::new(vec))
            }
            Mark::AREA => {
                to_type!(o, Area, s, attribute, subset);
                TypeEntity::AREA(o)
            }
            Mark::AUDIO => {
                to_type!(o, Audio, s, attribute, subset);
                TypeEntity::AUDIO(o)
            }
            Mark::BODY => {
                to_type!(o, Body, s, attribute, subset);
                TypeEntity::BODY(o)
            }
            Mark::BUTTON => {
                to_type!(o, Button, s, attribute, subset);
                TypeEntity::BUTTON(o)
            }
            Mark::CANVAS => {
                to_type!(o, Canv, s, attribute, subset);
                TypeEntity::CANVAS(o)
            }
            Mark::CSS => {
                to_type!(o, Css, s, attribute, subset);
                TypeEntity::CSS(o)
            }
            Mark::DIALOG => {
                to_type!(o, Dialog, s, attribute, subset);
                TypeEntity::DIALOG(o)
            }
            Mark::FORM => {
                to_type!(o, Form, s, attribute, subset);
                TypeEntity::FORM(o)
            }
            Mark::HEAD => {
                to_type!(o, Head, s, attribute, subset);
                TypeEntity::HEAD(o)
            }
            Mark::IFRAME => {
                to_type!(o, Iframe, s, attribute, subset);
                TypeEntity::IFRAME(o)
            }
            Mark::IMG => {
                to_type!(o, Img, s, attribute, subset);
                TypeEntity::IMG(o)
            }
            Mark::INP => {
                to_type!(o, Inp, s, attribute, subset);
                TypeEntity::INP(o)
            }
            Mark::OPTION => {
                let mut o = Opt::new();
                o.set_text(s);
                for attr in attribute.drain(..) {
                    o.attr(attr);
                }
                TypeEntity::OPTION(o)
            }
            Mark::PT => {
                to_type!(o, Pt, s, attribute, subset);
                TypeEntity::PT(o)
            }
            Mark::SCRIPT => {
                to_type!(o, Script, s, attribute, subset);
                TypeEntity::SCRIPT(o)
            }
            Mark::SELECT => {
                to_type!(o, Select, s, attribute, subset);
                TypeEntity::SELECT(o)
            }
            Mark::TIME => {
                to_type!(o, Time, s, attribute, subset);
                TypeEntity::TIME(o)
            }
            Mark::TITLE => {
                to_type!(o, Title, s, attribute, subset);
                TypeEntity::TITLE(o)
            }
            Mark::VIDEO => {
                to_type!(o, Video, s, attribute, subset);
                TypeEntity::VIDEO(o)
            }
        }
    }

    ///Parse a string slice to `TypeEntity`.
    pub fn from_str(buf: &str) -> Option<TypeEntity> {
        format::accept(buf).map(|o| TypeEntity::from(o))
    }
}

const ASYNCHRONOUS: &str = "async";
const CLASS: &str = "class";
const COLUMN: &str = "column";
const DISABLED: &str = "disabled";
const HEIGHT: &str = "height";
const HIDDEN: &str = "hidden";
const HREF: &str = "href";
const ID: &str = "id";
const MULTIPLE: &str = "multiple";
const NAME: &str = "name ";
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
    ASYNCHRONOUS(bool),
    CLASS(String),
    COLUMN(i32),
    DISABLED(bool),
    HEIGHT(i32),
    HIDDEN(bool),
    HREF(String),
    ID(String),
    MULTIPLE(bool),
    NAME(String),
    READONLY(bool),
    REQUIRED(bool),
    ROW(i32),
    SELECTED(bool),
    SRC(String),
    TIP(String),
    VALUE(String),
    WIDTH(i32),
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
    pub fn from(a: &str, s: Option<String>) -> Result<Attribute, Option<String>> {
        match a {
            ASYNCHRONOUS => match s {
                Some(s) => Ok(Attribute::ASYNCHRONOUS(to_bool(s))),
                None => Ok(Attribute::ASYNCHRONOUS(true)),
            },
            CLASS => match s {
                Some(s) => Ok(Attribute::CLASS(s)),
                None => Err(s),
            },
            COLUMN => match s {
                Some(s) => match to_i32(&s) {
                    Some(i) => Ok(Attribute::COLUMN(i)),
                    None => Err(Some(s)),
                },
                None => Err(s),
            },
            DISABLED => match s {
                Some(s) => Ok(Attribute::DISABLED(to_bool(s))),
                None => Ok(Attribute::DISABLED(true)),
            },
            HEIGHT => match s {
                Some(s) => match to_i32(&s) {
                    Some(i) => Ok(Attribute::HEIGHT(i)),
                    None => Err(Some(s)),
                },
                None => Err(s),
            },
            HIDDEN => match s {
                Some(s) => Ok(Attribute::HIDDEN(to_bool(s))),
                None => Ok(Attribute::HIDDEN(true)),
            },
            HREF => match s {
                Some(s) => Ok(Attribute::HREF(s)),
                None => Err(s),
            },
            ID => match s {
                Some(s) => Ok(Attribute::ID(s)),
                None => Err(s),
            },
            MULTIPLE => match s {
                Some(s) => Ok(Attribute::MULTIPLE(to_bool(s))),
                None => Ok(Attribute::MULTIPLE(true)),
            },
            NAME => match s {
                Some(s) => Ok(Attribute::NAME(s)),
                None => Err(s),
            },
            READONLY => match s {
                Some(s) => Ok(Attribute::READONLY(to_bool(s))),
                None => Ok(Attribute::READONLY(true)),
            },
            REQUIRED => match s {
                Some(s) => Ok(Attribute::REQUIRED(to_bool(s))),
                None => Ok(Attribute::REQUIRED(true)),
            },
            ROW => match s {
                Some(s) => match to_i32(&s) {
                    Some(i) => Ok(Attribute::ROW(i)),
                    None => Err(Some(s)),
                },
                None => Err(s),
            },
            SELECTED => match s {
                Some(s) => Ok(Attribute::SELECTED(to_bool(s))),
                None => Ok(Attribute::SELECTED(true)),
            },
            SRC => match s {
                Some(s) => Ok(Attribute::SRC(s)),
                None => Err(s),
            },
            TIP => match s {
                Some(s) => Ok(Attribute::TIP(s)),
                None => Err(s),
            },
            VALUE => match s {
                Some(s) => Ok(Attribute::VALUE(s)),
                None => Err(s),
            },
            WIDTH => match s {
                Some(s) => match to_i32(&s) {
                    Some(i) => Ok(Attribute::WIDTH(i)),
                    None => Err(Some(s)),
                },
                None => Err(s),
            },
            ONABORT => match s {
                Some(s) => Ok(Attribute::ONABORT(s)),
                None => Err(s),
            },
            ONBLUR => match s {
                Some(s) => Ok(Attribute::ONBLUR(s)),
                None => Err(s),
            },
            ONCANCEL => match s {
                Some(s) => Ok(Attribute::ONCANCEL(s)),
                None => Err(s),
            },
            ONCHANGE => match s {
                Some(s) => Ok(Attribute::ONCHANGE(s)),
                None => Err(s),
            },
            ONCLICK => match s {
                Some(s) => Ok(Attribute::ONCLICK(s)),
                None => Err(s),
            },
            ONCLOSE => match s {
                Some(s) => Ok(Attribute::ONCLOSE(s)),
                None => Err(s),
            },
            ONFOCUS => match s {
                Some(s) => Ok(Attribute::ONFOCUS(s)),
                None => Err(s),
            },
            ONINVALID => match s {
                Some(s) => Ok(Attribute::ONINVALID(s)),
                None => Err(s),
            },
            ONLOAD => match s {
                Some(s) => Ok(Attribute::ONLOAD(s)),
                None => Err(s),
            },
            ONRESIZE => match s {
                Some(s) => Ok(Attribute::ONRESIZE(s)),
                None => Err(s),
            },
            ONSCROLL => match s {
                Some(s) => Ok(Attribute::ONSCROLL(s)),
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
    pub fn from_str(buf: &str) -> Option<Page> {
        match TypeEntity::from_str(buf)? {
            TypeEntity::PAGE(p) => Some(p),
            _ => None,
        }
    }

    pub(crate) fn draw(&mut self, canvas: &Canvas) {
        self.body.draw(canvas);
    }
}
