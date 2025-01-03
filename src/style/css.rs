use crate::content::BodyOrWrapper;
use crate::markup::{Attribute, Conditions, Mark, Page};
use crate::parts::Distance;
use lightningcss::properties::custom::{
    CustomProperty, CustomPropertyName, TokenList, TokenOrValue,
};
use lightningcss::properties::size::Size;
use lightningcss::properties::Property;
use lightningcss::rules::style::StyleRule;
use lightningcss::rules::CssRule;
use lightningcss::selector::{Component, Selector};
use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet};
use lightningcss::traits::ToCss;
use lightningcss::values::length::LengthValue;
use lightningcss::values::percentage::DimensionPercentage;

#[derive(Debug)]
pub(super) struct CssParser {}

impl CssParser {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn parse(&mut self, s: &str, page: &mut Page) {
        match StyleSheet::parse(s, ParserOptions::default()) {
            Ok(mut stylesheet) => {
                if let Err(_) = stylesheet.minify(MinifyOptions::default()) {}
                for r in &stylesheet.rules.0 {
                    css_rule(r, page)
                }
            }
            Err(_) => {}
        };
    }
}

#[inline]
fn css_rule(r: &CssRule, page: &mut Page) {
    match r {
        CssRule::Style(o) => {
            style_rule(o, page);
        }
        _ => {}
    }
}

#[inline]
fn style_rule(o: &StyleRule, page: &mut Page) {
    if let Some(c) = o.selectors.0.iter().find_map(|s| selector(s)) {
        let mut vec = find_draw(c, page);
        if vec.is_empty() {
            return;
        }
        for d in &o.declarations.declarations {
            property(d, &mut vec);
        }
    }
}

#[inline]
fn selector(s: &Selector) -> Option<Conditions> {
    let mut c = Conditions::new();
    for a in s.iter_raw_match_order() {
        match a {
            Component::Class(o) => {
                c.class(o.as_ref());
            }
            Component::Combinator(_) => {}
            Component::ID(o) => {
                c.id(o.as_ref());
            }
            Component::LocalName(o) => {
                if let Some(m) = Mark::from(&o.lower_name) {
                    c.mark(m);
                }
            }
            _ => {}
        }
    }
    c.reverse();
    if c.is_empty() {
        None
    } else {
        Some(c)
    }
}

#[inline]
fn find_draw(c: Conditions, page: &mut Page) -> Vec<BodyOrWrapper> {
    let elements = page.find_in_body(c);
    let body = page.body();
    elements
        .into_iter()
        .filter_map(|e| body.find_wrapper(e))
        .collect()
}

#[inline]
fn property(p: &Property, wrappers: &mut Vec<BodyOrWrapper>) {
    match p {
        Property::Custom(o) => {
            property_custom(o, wrappers);
        }
        Property::Height(o) => {
            property_height(o, wrappers);
        }
        Property::Width(o) => {
            property_width(o, wrappers);
        }
        _ => {}
    }
}

#[inline]
fn property_custom(c: &CustomProperty, wrappers: &mut Vec<BodyOrWrapper>) {
    let n = custom_property_name(&c.name);
    let v = custom_property_value(&c.value);
    for wrapper in wrappers.iter_mut() {
        if let Ok(a) = Attribute::from(&n, Some(v.clone())) {
            set_attribute(a, wrapper);
        }
    }
}

#[inline]
fn custom_property_name(n: &CustomPropertyName) -> String {
    match n {
        CustomPropertyName::Custom(i) => i.to_string(),
        CustomPropertyName::Unknown(i) => i.to_string(),
    }
}

#[inline]
fn custom_property_value(value: &TokenList) -> String {
    let mut s = String::new();
    for t in &value.0 {
        match t {
            TokenOrValue::Color(o) => {
                if let Ok(c) = o.to_css_string(PrinterOptions::default()) {
                    s.push_str(&c);
                }
            }
            TokenOrValue::Length(o) => {
                if let Ok(c) = o.to_css_string(PrinterOptions::default()) {
                    s.push_str(&c);
                }
            }
            TokenOrValue::Token(o) => {
                if let Ok(c) = o.to_css_string(PrinterOptions::default()) {
                    s.push_str(&c);
                }
            }
            TokenOrValue::Url(o) => {
                if let Ok(c) = o.to_css_string(PrinterOptions::default()) {
                    s.push_str(&c);
                }
            }
            _ => {}
        }
    }
    s
}

#[inline]
fn property_height(s: &Size, wrappers: &mut Vec<BodyOrWrapper>) {
    match s {
        Size::LengthPercentage(o) => match o {
            DimensionPercentage::Dimension(d) => match d {
                LengthValue::Px(n) => {
                    let n = *n as isize;
                    for wrapper in wrappers {
                        set_attribute(Attribute::HEIGHT(Distance::Pixel(n)), wrapper);
                    }
                }
                _ => {}
            },
            DimensionPercentage::Percentage(d) => {
                for wrapper in wrappers {
                    let n = d.0 as usize;
                    set_attribute(Attribute::HEIGHT(Distance::Percentage(n)), wrapper);
                }
            }
            DimensionPercentage::Calc(_) => {}
        },
        _ => {}
    }
}

#[inline]
fn property_width(s: &Size, wrappers: &mut Vec<BodyOrWrapper>) {
    match s {
        Size::LengthPercentage(o) => match o {
            DimensionPercentage::Dimension(d) => match d {
                LengthValue::Px(n) => {
                    let n = *n as isize;
                    for wrapper in wrappers {
                        set_attribute(Attribute::WIDTH(Distance::Pixel(n)), wrapper);
                    }
                }
                _ => {}
            },
            DimensionPercentage::Percentage(d) => {
                for wrapper in wrappers {
                    let n = d.0 as usize;
                    set_attribute(Attribute::WIDTH(Distance::Percentage(n)), wrapper);
                }
            }
            DimensionPercentage::Calc(_) => {}
        },
        _ => {}
    }
}

#[inline]
fn set_attribute(a: Attribute, o: &mut BodyOrWrapper) {
    match o {
        BodyOrWrapper::BODY(o) => unsafe {
            if let Some(o) = o.as_mut() {
                if let Ok(mut e) = o.element().write() {
                    e.attribute.insert(a.name(), a);
                }
            }
        },
        BodyOrWrapper::DRAWUNITWRAPPER(o) => unsafe {
            if let Some(o) = o.as_mut() {
                if let Ok(mut e) = o.element().write() {
                    e.attribute.insert(a.name(), a);
                }
            }
        },
    }
}
