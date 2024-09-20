mod format;

use crate::content::{Audio, Button, Canv, Form, Iframe, Img, Inp, Opt, Pt, Select, Time, Video};
use crate::grid::{Area, Body};
use crate::head::{Aht, Head, Title};
use crate::metadata::{Script, Style};
use crate::parts::{Coord2D, Distance, Ordinal, Points, RectSide};
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
pub(crate) const FORM: &str = "form";
pub(crate) const HEAD: &str = "head";
pub(crate) const IFRAME: &str = "iframe";
pub(crate) const IMG: &str = "img";
pub(crate) const INP: &str = "inp";
pub(crate) const OPTION: &str = "option";
pub(crate) const PT: &str = "pt";
pub(crate) const SCRIPT: &str = "script";
pub(crate) const SELECT: &str = "select";
pub(crate) const STYLE: &str = "style";
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
    ///Converts a string slice to `Mark`.
    pub fn from(s: String) -> Result<Self, String> {
        match s.as_str() {
            AHT => Ok(Self::AHT),
            AREA => Ok(Self::AREA),
            AUDIO => Ok(Self::AUDIO),
            BODY => Ok(Self::BODY),
            BUTTON => Ok(Self::BUTTON),
            CANVAS => Ok(Self::CANVAS),
            FORM => Ok(Self::FORM),
            HEAD => Ok(Self::HEAD),
            IFRAME => Ok(Self::IFRAME),
            IMG => Ok(Self::IMG),
            INP => Ok(Self::INP),
            OPTION => Ok(Self::OPTION),
            PT => Ok(Self::PT),
            SCRIPT => Ok(Self::SCRIPT),
            SELECT => Ok(Self::SELECT),
            STYLE => Ok(Self::STYLE),
            TIME => Ok(Self::TIME),
            TITLE => Ok(Self::TITLE),
            VIDEO => Ok(Self::VIDEO),
            _ => Err(s),
        }
    }

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

///TypeEntity.
#[derive(Debug)]
pub enum TypeEntity {
    AHT(Aht),
    AREA(Area),
    AUDIO(Audio),
    BODY(Body),
    BUTTON(Button),
    CANVAS(Canv),
    FORM(Form),
    HEAD(Head),
    IFRAME(Iframe),
    IMG(Img),
    INP(Inp),
    OPTION(Opt),
    PT(Pt),
    SCRIPT(Script),
    SELECT(Select),
    STYLE(Style),
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
                    .for_each(|sub| o.subset.push_back(Self::from(sub)));
                o
            }};
        }
        macro_rules! to_type1 {
            ($t:ty) => {{
                let mut o = <$t>::new();
                attribute.drain(..).for_each(|a| o.attr(a));
                subset
                    .drain(..)
                    .for_each(|sub| o.subset.push_back(Self::from(sub)));
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
            Mark::FORM => Self::FORM(to_type!(Form)),
            Mark::HEAD => Self::HEAD(to_type!(Head)),
            Mark::IFRAME => Self::IFRAME(to_type!(Iframe)),
            Mark::IMG => Self::IMG(to_type!(Img)),
            Mark::INP => Self::INP(to_type!(Inp)),
            Mark::OPTION => Self::OPTION(to_type2!(Opt)),
            Mark::PT => Self::PT(to_type!(Pt)),
            Mark::SCRIPT => Self::SCRIPT(to_type!(Script)),
            Mark::SELECT => Self::SELECT(to_type!(Select)),
            Mark::STYLE => Self::STYLE(to_type!(Style)),
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
            let mut t = (None, None, None, None);
            for (i, o) in o.subset.vec.into_iter().enumerate() {
                match o {
                    TypeEntity::HEAD(o) => {
                        if i == 0 {
                            t.0 = Some(o)
                        }
                    }
                    TypeEntity::BODY(o) => {
                        if i == 1 {
                            t.1 = Some(o);
                        }
                    }
                    TypeEntity::STYLE(o) => {
                        if i == 2 {
                            t.2 = Some(o);
                        }
                    }
                    TypeEntity::SCRIPT(o) => {
                        if i == 3 {
                            t.3 = Some(o);
                        }
                    }
                    _ => {}
                }
            }
            Ok(Page::new(t.0, t.1, t.2, t.3))
        } else {
            Err(self)
        }
    }

    fn find_mark(&mut self, s: &str, v: &mut Vec<&mut Self>) {
        let t = self as *mut Self;

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
            Self::AHT(o) => find!(o),
            Self::AREA(o) => find!(o),
            Self::AUDIO(o) => find!(o),
            Self::BODY(o) => find!(o),
            Self::BUTTON(o) => find!(o),
            Self::CANVAS(o) => find!(o),
            Self::FORM(o) => find!(o),
            Self::HEAD(o) => find!(o),
            Self::IFRAME(o) => find!(o),
            Self::IMG(o) => find!(o),
            Self::INP(o) => find!(o),
            Self::OPTION(_) => {}
            Self::PT(o) => find!(o),
            Self::SCRIPT(o) => find!(o),
            Self::SELECT(o) => find!(o),
            Self::STYLE(o) => find!(o),
            Self::TIME(o) => find!(o),
            Self::TITLE(o) => find!(o),
            Self::VIDEO(o) => find!(o),
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
            Self::AHT(o) => find!(o),
            Self::AREA(o) => find!(o),
            Self::AUDIO(o) => find!(o),
            Self::BODY(o) => find!(o),
            Self::BUTTON(o) => find!(o),
            Self::CANVAS(o) => find!(o),
            Self::FORM(o) => find!(o),
            Self::HEAD(o) => find!(o),
            Self::IFRAME(o) => find!(o),
            Self::IMG(o) => find!(o),
            Self::INP(o) => find!(o),
            Self::OPTION(_) => {}
            Self::PT(o) => find!(o),
            Self::SCRIPT(o) => find!(o),
            Self::SELECT(o) => find!(o),
            Self::STYLE(o) => find!(o),
            Self::TIME(o) => find!(o),
            Self::TITLE(o) => find!(o),
            Self::VIDEO(o) => find!(o),
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
            Self::AHT(o) => find!(o),
            Self::AREA(o) => find!(o),
            Self::AUDIO(o) => find!(o),
            Self::BODY(o) => find!(o),
            Self::BUTTON(o) => find!(o),
            Self::CANVAS(o) => find!(o),
            Self::FORM(o) => find!(o),
            Self::HEAD(o) => find!(o),
            Self::IFRAME(o) => find!(o),
            Self::IMG(o) => find!(o),
            Self::INP(o) => find!(o),
            Self::OPTION(_) => None,
            Self::PT(o) => find!(o),
            Self::SCRIPT(o) => find!(o),
            Self::SELECT(o) => find!(o),
            Self::STYLE(o) => find!(o),
            Self::TIME(o) => find!(o),
            Self::TITLE(o) => find!(o),
            Self::VIDEO(o) => find!(o),
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
        Self {
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
    style: Option<Box<TypeEntity>>,
    script: Option<Box<TypeEntity>>,
    pub(crate) cursor: VisionAction,
}

impl Page {
    fn new(
        head: Option<Head>,
        body: Option<Body>,
        style: Option<Style>,
        script: Option<Script>,
    ) -> Self {
        let mut page = Self {
            head: head.map(|o| Box::new(TypeEntity::HEAD(o))),
            body: body.map(|o| Box::new(TypeEntity::BODY(o))),
            style: style.map(|o| Box::new(TypeEntity::STYLE(o))),
            script: script.map(|o| Box::new(TypeEntity::SCRIPT(o))),
            cursor: VisionAction::new(),
        };

        let p = &mut page as *mut Self;
        if let Some(t) = &mut page.body {
            let t = t.as_mut();
            let o = t as *mut TypeEntity;
            if let TypeEntity::BODY(body) = t {
                unsafe {
                    body.set_parent(&mut *o);
                    page.style(|o| o.build(&mut *p));
                }
            }
        }
        page
    }

    ///Parse a string slice to `Page`.
    pub fn from_str(buf: &str) -> EntityResult {
        if let Some(e) = TypeEntity::from_str(buf).pop_front() {
            match e.to_page() {
                Ok(p) => EntityResult::Page(p),
                Err(e) => EntityResult::TypeEntity(e),
            }
        } else {
            EntityResult::None
        }
    }

    ///Returns true if the `Page` doesn't contain `Head` or `Body` or `Style` or `Script`.
    pub fn incomplete(&self) -> bool {
        self.head.is_none() || self.body.is_none() || self.style.is_none() || self.script.is_none()
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

    fn style(&mut self, f: impl Fn(&mut Style)) -> bool {
        if let Some(t) = &mut self.style {
            if let TypeEntity::STYLE(style) = t.as_mut() {
                f(style);
                return true;
            }
        }
        false
    }

    fn script(&mut self, f: impl Fn(&mut Script)) -> bool {
        if let Some(t) = &mut self.script {
            if let TypeEntity::SCRIPT(script) = t.as_mut() {
                f(script);
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
        let p = self as *mut Self;
        self.body(|o| o.draw(canvas, unsafe { &mut *p }))
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

    pub fn set_cursor(&mut self, p: VisionPosition) {
        self.cursor.add(p);
    }
}

///EntityResult.
#[derive(Debug)]
pub enum EntityResult {
    Page(Page),
    TypeEntity(TypeEntity),
    None,
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

///CursorAction.
#[derive(Clone, Debug, PartialEq)]
pub enum CursorAction {
    Down(usize),
    Up,
}

///VisionPosition.
#[derive(Debug)]
pub struct VisionPosition {
    pub xy: Coord2D,
    pub action: CursorAction,
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

    pub(crate) fn analyse(&self) -> VisionActionResult {
        let (fir, sec) = self.ordered();
        if let Some(fir) = fir {
            match fir.action {
                CursorAction::Down(n) => {
                    if let Some(sec) = sec {
                        if fir.action == sec.action {
                            return VisionActionResult::PressSweep(
                                &fir.xy,
                                fir.xy.away_from(&sec.xy),
                            );
                        }
                    }
                    return VisionActionResult::Press(&fir.xy, n);
                }
                CursorAction::Up => {
                    if let Some(sec) = sec {
                        if fir.action == sec.action {
                            return VisionActionResult::Sweep(&fir.xy, fir.xy.away_from(&sec.xy));
                        }
                    }
                    return VisionActionResult::Loosen(&fir.xy);
                }
            }
        }
        VisionActionResult::None
    }
}

#[derive(Debug)]
pub(crate) enum VisionActionResult<'a> {
    Press(&'a Coord2D, usize),
    Sweep(&'a Coord2D, RectSide),
    PressSweep(&'a Coord2D, RectSide),
    Loosen(&'a Coord2D),
    None,
}
