use std::str::FromStr;

pub(crate) mod ascii {
    //'<'
    pub(crate) const LT: char = '<';
    //'>'
    pub(crate) const GT: char = '>';
    //'/'
    pub(crate) const SLASH: char = '/';
    //' '
    pub(crate) const SPACE: char = ' ';
    //'\r'
    pub(crate) const CR: char = '\r';
    //'\n'
    pub(crate) const LF: char = '\n';
    //'='
    pub(crate) const EQUAL: char = '=';
    //'"'
    pub(crate) const QUOTATION: char = '"';
    //'''
    pub(crate) const APOSTROPHE: char = '\'';
    //','
    pub(crate) const COMMA: char = ',';
    //'['
    pub(crate) const LEFT_SQUARE_BRACKET: char = '[';
    //']'
    pub(crate) const RIGHT_SQUARE_BRACKET: char = ']';
    //'%'
    pub(crate) const PER_CENT: char = '%';
    //'&'
    pub(crate) const AMPERSAND: char = '&';
    //';'
    pub(crate) const SEMICOLON: char = ';';
}

pub(crate) fn to_bool(s: &str) -> bool {
    bool::from_str(s.trim()).unwrap_or(false)
}

pub(crate) fn to_usize(s: &str) -> Option<usize> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    match usize::from_str_radix(s, 10) {
        Ok(i) => Some(i),
        Err(e) => {
            println!("to_usize({:?}) {:?}", s, e);
            None
        }
    }
}

pub(crate) fn to_isize(s: &str) -> Option<isize> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    match isize::from_str_radix(s, 10) {
        Ok(i) => Some(i),
        Err(e) => {
            println!("to_isize({:?}) {:?}", s, e);
            None
        }
    }
}

#[inline]
pub fn between<T: PartialOrd>(o: T, min: T, max: T) -> bool {
    o >= min && o <= max
}
