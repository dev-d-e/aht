/*!
A module for markup.
*/

mod entity;
mod format;
mod ops;
mod parts;

pub use self::entity::*;
pub use self::ops::*;
pub use self::parts::*;
use crate::error::*;
use crate::page::*;
use crate::utils::*;
use std::mem::take;
use std::str::FromStr;

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
#[derive(Clone, Debug, PartialEq)]
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

impl FromStr for Mark {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
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
            _ => Err(ErrorKind::InvalidMark.into()),
        }
    }
}

impl TryFrom<&str> for Mark {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

impl TryFrom<&String> for Mark {
    type Error = Error;

    fn try_from(s: &String) -> Result<Self> {
        Self::from_str(s.as_str())
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
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
            Self::ACTION => ACTION,
            Self::CLASS => CLASS,
            Self::COLUMN => COLUMN,
            Self::DISABLED => DISABLED,
            Self::ENCTYPE => ENCTYPE,
            Self::HEIGHT => HEIGHT,
            Self::HIDDEN => HIDDEN,
            Self::HREF => HREF,
            Self::ID => ID,
            Self::LANG => LANG,
            Self::METHOD => METHOD,
            Self::MULTIPLE => MULTIPLE,
            Self::NAME => NAME,
            Self::ORDINAL => ORDINAL,
            Self::POSITION => POSITION,
            Self::READONLY => READONLY,
            Self::REQUIRED => REQUIRED,
            Self::ROW => ROW,
            Self::SELECTED => SELECTED,
            Self::SRC => SRC,
            Self::TIP => TIP,
            Self::TYPE => TYPE,
            Self::VALUE => VALUE,
            Self::WIDTH => WIDTH,
            Self::ONABORT => ONABORT,
            Self::ONBLUR => ONBLUR,
            Self::ONCANCEL => ONCANCEL,
            Self::ONCHANGE => ONCHANGE,
            Self::ONCLICK => ONCLICK,
            Self::ONCLOSE => ONCLOSE,
            Self::ONFOCUS => ONFOCUS,
            Self::ONINVALID => ONINVALID,
            Self::ONLOAD => ONLOAD,
            Self::ONRESIZE => ONRESIZE,
            Self::ONSCROLL => ONSCROLL,
        }
    }
}

impl FromStr for AttrName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            ACTION => Ok(Self::ACTION),
            CLASS => Ok(Self::CLASS),
            COLUMN => Ok(Self::COLUMN),
            DISABLED => Ok(Self::DISABLED),
            ENCTYPE => Ok(Self::ENCTYPE),
            HEIGHT => Ok(Self::HEIGHT),
            HIDDEN => Ok(Self::HIDDEN),
            HREF => Ok(Self::HREF),
            ID => Ok(Self::ID),
            LANG => Ok(Self::LANG),
            METHOD => Ok(Self::METHOD),
            MULTIPLE => Ok(Self::MULTIPLE),
            NAME => Ok(Self::NAME),
            ORDINAL => Ok(Self::ORDINAL),
            POSITION => Ok(Self::POSITION),
            READONLY => Ok(Self::READONLY),
            REQUIRED => Ok(Self::REQUIRED),
            ROW => Ok(Self::ROW),
            SELECTED => Ok(Self::SELECTED),
            SRC => Ok(Self::SRC),
            TIP => Ok(Self::TIP),
            TYPE => Ok(Self::TYPE),
            VALUE => Ok(Self::VALUE),
            WIDTH => Ok(Self::WIDTH),
            ONABORT => Ok(Self::ONABORT),
            ONBLUR => Ok(Self::ONBLUR),
            ONCANCEL => Ok(Self::ONCANCEL),
            ONCHANGE => Ok(Self::ONCHANGE),
            ONCLICK => Ok(Self::ONCLICK),
            ONCLOSE => Ok(Self::ONCLOSE),
            ONFOCUS => Ok(Self::ONFOCUS),
            ONINVALID => Ok(Self::ONINVALID),
            ONLOAD => Ok(Self::ONLOAD),
            ONRESIZE => Ok(Self::ONRESIZE),
            ONSCROLL => Ok(Self::ONSCROLL),
            _ => Err(ErrorKind::InvalidAttribute.into()),
        }
    }
}

impl TryFrom<&str> for AttrName {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

impl TryFrom<&String> for AttrName {
    type Error = Error;

    fn try_from(s: &String) -> Result<Self> {
        Self::from_str(s.as_str())
    }
}

impl From<&Attribute> for AttrName {
    fn from(a: &Attribute) -> Self {
        match a {
            Attribute::ACTION(_) => Self::ACTION,
            Attribute::CLASS(_) => Self::CLASS,
            Attribute::COLUMN(_) => Self::COLUMN,
            Attribute::DISABLED(_) => Self::DISABLED,
            Attribute::ENCTYPE(_) => Self::ENCTYPE,
            Attribute::HEIGHT(_) => Self::HEIGHT,
            Attribute::HIDDEN(_) => Self::HIDDEN,
            Attribute::HREF(_) => Self::HREF,
            Attribute::ID(_) => Self::ID,
            Attribute::LANG(_) => Self::LANG,
            Attribute::METHOD(_) => Self::METHOD,
            Attribute::MULTIPLE(_) => Self::MULTIPLE,
            Attribute::NAME(_) => Self::NAME,
            Attribute::ORDINAL(_) => Self::ORDINAL,
            Attribute::POSITION(_) => Self::POSITION,
            Attribute::READONLY(_) => Self::READONLY,
            Attribute::REQUIRED(_) => Self::REQUIRED,
            Attribute::ROW(_) => Self::ROW,
            Attribute::SELECTED(_) => Self::SELECTED,
            Attribute::SRC(_) => Self::SRC,
            Attribute::TIP(_) => Self::TIP,
            Attribute::TYPE(_) => Self::TYPE,
            Attribute::VALUE(_) => Self::VALUE,
            Attribute::WIDTH(_) => Self::WIDTH,
            Attribute::ONABORT(_) => Self::ONABORT,
            Attribute::ONBLUR(_) => Self::ONBLUR,
            Attribute::ONCANCEL(_) => Self::ONCANCEL,
            Attribute::ONCHANGE(_) => Self::ONCHANGE,
            Attribute::ONCLICK(_) => Self::ONCLICK,
            Attribute::ONCLOSE(_) => Self::ONCLOSE,
            Attribute::ONFOCUS(_) => Self::ONFOCUS,
            Attribute::ONINVALID(_) => Self::ONINVALID,
            Attribute::ONLOAD(_) => Self::ONLOAD,
            Attribute::ONRESIZE(_) => Self::ONRESIZE,
            Attribute::ONSCROLL(_) => Self::ONSCROLL,
        }
    }
}

///Represents attribute.
#[derive(Clone, Debug)]
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
    pub fn from(a: &AttrName, s: &mut String) -> Result<Self> {
        let t = s.as_str();
        match a {
            AttrName::ACTION => Ok(Self::ACTION(take(s))),
            AttrName::CLASS => Ok(Self::CLASS(take(s))),
            AttrName::COLUMN => Points::try_from(t).map(|o| Self::COLUMN(o)),
            AttrName::DISABLED => to_bool(t).map(|o| Self::DISABLED(o)),
            AttrName::ENCTYPE => Ok(Self::ENCTYPE(take(s))),
            AttrName::HEIGHT => Distance::try_from(t).map(|o| Self::HEIGHT(o)),
            AttrName::HIDDEN => to_bool(t).map(|o| Self::HIDDEN(o)),
            AttrName::HREF => Ok(Self::HREF(take(s))),
            AttrName::ID => Ok(Self::ID(take(s))),
            AttrName::LANG => Ok(Self::LANG(take(s))),
            AttrName::METHOD => Ok(Self::METHOD(take(s))),
            AttrName::MULTIPLE => to_bool(t).map(|o| Self::MULTIPLE(o)),
            AttrName::NAME => Ok(Self::NAME(take(s))),
            AttrName::ORDINAL => Ordinal::try_from(t).map(|o| Self::ORDINAL(o)),
            AttrName::POSITION => Coord::try_from(t).map(|c| Self::POSITION(c)),
            AttrName::READONLY => to_bool(t).map(|o| Self::READONLY(o)),
            AttrName::REQUIRED => to_bool(t).map(|o| Self::REQUIRED(o)),
            AttrName::ROW => Points::try_from(t).map(|o| Self::ROW(o)),
            AttrName::SELECTED => to_bool(t).map(|o| Self::SELECTED(o)),
            AttrName::SRC => Ok(Self::SRC(take(s))),
            AttrName::TIP => Ok(Self::TIP(take(s))),
            AttrName::TYPE => ScriptType::try_from(t).map(|t| Self::TYPE(t)),
            AttrName::VALUE => Ok(Self::VALUE(take(s))),
            AttrName::WIDTH => Distance::try_from(t).map(|o| Self::WIDTH(o)),
            AttrName::ONABORT => Ok(Self::ONABORT(take(s))),
            AttrName::ONBLUR => Ok(Self::ONBLUR(take(s))),
            AttrName::ONCANCEL => Ok(Self::ONCANCEL(take(s))),
            AttrName::ONCHANGE => Ok(Self::ONCHANGE(take(s))),
            AttrName::ONCLICK => Ok(Self::ONCLICK(take(s))),
            AttrName::ONCLOSE => Ok(Self::ONCLOSE(take(s))),
            AttrName::ONFOCUS => Ok(Self::ONFOCUS(take(s))),
            AttrName::ONINVALID => Ok(Self::ONINVALID(take(s))),
            AttrName::ONLOAD => Ok(Self::ONLOAD(take(s))),
            AttrName::ONRESIZE => Ok(Self::ONRESIZE(take(s))),
            AttrName::ONSCROLL => Ok(Self::ONSCROLL(take(s))),
        }
    }

    ///Returns attribute name.
    pub fn name(&self) -> AttrName {
        self.into()
    }
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Attribute::ACTION(o) => o,
            Attribute::CLASS(o) => o,
            Attribute::COLUMN(o) => &o.to_string(),
            Attribute::DISABLED(o) => &o.to_string(),
            Attribute::ENCTYPE(o) => o,
            Attribute::HEIGHT(o) => &o.to_string(),
            Attribute::HIDDEN(o) => &o.to_string(),
            Attribute::HREF(o) => o,
            Attribute::ID(o) => o,
            Attribute::LANG(o) => o,
            Attribute::METHOD(o) => o,
            Attribute::MULTIPLE(o) => &o.to_string(),
            Attribute::NAME(o) => o,
            Attribute::ORDINAL(o) => &o.to_string(),
            Attribute::POSITION(o) => &o.to_string(),
            Attribute::READONLY(o) => &o.to_string(),
            Attribute::REQUIRED(o) => &o.to_string(),
            Attribute::ROW(o) => &o.to_string(),
            Attribute::SELECTED(o) => &o.to_string(),
            Attribute::SRC(o) => o,
            Attribute::TIP(o) => o,
            Attribute::TYPE(o) => &o.to_string(),
            Attribute::VALUE(o) => o,
            Attribute::WIDTH(o) => &o.to_string(),
            Attribute::ONABORT(o) => o,
            Attribute::ONBLUR(o) => o,
            Attribute::ONCANCEL(o) => o,
            Attribute::ONCHANGE(o) => o,
            Attribute::ONCLICK(o) => o,
            Attribute::ONCLOSE(o) => o,
            Attribute::ONFOCUS(o) => o,
            Attribute::ONINVALID(o) => o,
            Attribute::ONLOAD(o) => o,
            Attribute::ONRESIZE(o) => o,
            Attribute::ONSCROLL(o) => o,
        };
        f.write_str(s)
    }
}
