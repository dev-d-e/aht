use super::*;
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
use std::sync::{Arc, RwLock};

#[derive(Debug, Default)]
pub(super) struct CssParser {}

impl CssParser {
    pub(crate) fn parse(&mut self, s: &str, context: &mut PageContext) {
        match StyleSheet::parse(s, ParserOptions::default()) {
            Ok(mut stylesheet) => {
                if let Err(e) = stylesheet.minify(MinifyOptions::default()) {
                    error!("{e}");
                }
                for r in &stylesheet.rules.0 {
                    css_rule(r, context)
                }
            }
            Err(e) => {
                error!("{e}");
            }
        };
    }
}

#[inline]
fn css_rule(r: &CssRule, context: &mut PageContext) {
    match r {
        CssRule::Style(o) => {
            style_rule(o, context);
        }
        _ => {}
    }
}

#[inline]
fn style_rule(o: &StyleRule, context: &mut PageContext) {
    if let Some(c) = o.selectors.0.iter().find_map(|s| selector(s)) {
        let vec = context.find_in_body(c);
        if vec.is_empty() {
            return;
        }
        for d in &o.declarations.declarations {
            property(d, &vec);
        }
    }
}

#[inline]
fn selector<'a>(s: &'a Selector) -> Option<Conditions<'a>> {
    let mut c = Conditions::default();
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
                if let Ok(m) = Mark::try_from(o.lower_name.as_ref()) {
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
fn property(p: &Property, vec: &Vec<Arc<RwLock<Element>>>) {
    match p {
        Property::Custom(o) => {
            property_custom(o, vec);
        }
        Property::Height(o) => {
            property_height(o, vec);
        }
        Property::Width(o) => {
            property_width(o, vec);
        }
        _ => {}
    }
}

#[inline]
fn property_custom(c: &CustomProperty, vec: &Vec<Arc<RwLock<Element>>>) {
    let n = custom_property_name(&c.name);
    let mut v = custom_property_value(&c.value);
    if let Ok(n) = AttrName::try_from(&n) {
        if let Ok(a) = Attribute::from(&n, &mut v) {
            for e in vec {
                set_attribute(a.clone(), e);
            }
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
fn property_height(s: &Size, vec: &Vec<Arc<RwLock<Element>>>) {
    match s {
        Size::LengthPercentage(o) => match o {
            DimensionPercentage::Dimension(d) => match d {
                LengthValue::Px(n) => {
                    let n = *n;
                    for e in vec {
                        set_attribute(Attribute::HEIGHT(Distance::Pixel(n)), e);
                    }
                }
                _ => {}
            },
            DimensionPercentage::Percentage(d) => {
                let n = d.0;
                for e in vec {
                    set_attribute(Attribute::HEIGHT(Distance::Percentage(n)), e);
                }
            }
            DimensionPercentage::Calc(_) => {}
        },
        _ => {}
    }
}

#[inline]
fn property_width(s: &Size, vec: &Vec<Arc<RwLock<Element>>>) {
    match s {
        Size::LengthPercentage(o) => match o {
            DimensionPercentage::Dimension(d) => match d {
                LengthValue::Px(n) => {
                    let n = *n;
                    for e in vec {
                        set_attribute(Attribute::WIDTH(Distance::Pixel(n)), e);
                    }
                }
                _ => {}
            },
            DimensionPercentage::Percentage(d) => {
                let n = d.0;
                for e in vec {
                    set_attribute(Attribute::WIDTH(Distance::Percentage(n)), e);
                }
            }
            DimensionPercentage::Calc(_) => {}
        },
        _ => {}
    }
}

#[inline]
fn set_attribute(a: Attribute, e: &Arc<RwLock<Element>>) {
    if let Ok(mut e) = e.write() {
        e.attribute_insert(a);
    }
}
