use super::*;
use crate::utils::ascii::*;
use std::collections::HashMap;
use std::mem::take;
use std::str::FromStr;

#[derive(Debug, Default, Getters, MutGetters)]
pub(super) struct StyleSheet {
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    meta_rules: MetaRules,
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    style_rules: StyleRules,
}

impl StyleSheet {
    pub(super) fn new(meta_rules: MetaRules, style_rules: StyleRules) -> Self {
        Self {
            meta_rules,
            style_rules,
        }
    }
}

#[derive(Debug, Default)]
#[repr(transparent)]
pub(super) struct MetaRules(Vec<MetaRule>);

deref!(MetaRules, Vec<MetaRule>, 0);

#[derive(Debug, Default, Getters, MutGetters)]
pub(super) struct MetaRule {
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    key: String,
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    attribute: HashMap<AttrName, Attribute>,
}

impl MetaRule {
    fn new(key: String) -> Self {
        Self {
            key,
            attribute: Default::default(),
        }
    }
}

#[derive(Debug, Default)]
#[repr(transparent)]
pub(super) struct StyleRules(Vec<StyleRule>);

deref!(StyleRules, Vec<StyleRule>, 0);

impl StyleRules {
    pub(super) fn new(v: Vec<StyleRule>) -> Self {
        Self(v)
    }
}

#[derive(Debug, Default, Getters, MutGetters)]
pub(super) struct StyleRule {
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    key: SelectorHolder,
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    attribute: HashMap<AttrName, Attribute>,
}

impl StyleRule {
    fn new(key: SelectorHolder) -> Self {
        Self {
            key,
            attribute: Default::default(),
        }
    }
}

#[derive(Debug)]
pub(crate) enum Selector {
    Mark(Mark),
    Attribute(AttrName, AttrPattern),
    AttrName(AttrName),
    Any,
}

#[derive(Debug)]
pub(crate) enum Combiner {
    Descendant(usize),
    NextSibling(usize),
    PrecedingSibling(usize),
}

impl Combiner {
    fn find(&self, s: &Selector, ks: &[ElementKey], eh: &ElementHolder) -> Vec<ElementKey> {
        let mut r = Vec::new();
        match self {
            &Self::Descendant(n) => match s {
                Selector::Mark(o) => {
                    FindDescendant::new(MarkEq::new(o), n).elements(ks, eh, &mut r);
                }
                Selector::AttrName(o) => {
                    FindDescendant::new(AttrExists::new(o), n).elements(ks, eh, &mut r);
                }
                Selector::Attribute(o, p) => {
                    let f = FindDescendant::new(AttrMatch::new(o, p), n);
                    if matches!(o, AttrName::ID) {
                        if let Some(o) = f.elements_to_unique(ks, eh) {
                            r.push(o)
                        }
                    } else {
                        f.elements(ks, eh, &mut r);
                    }
                }
                Selector::Any => {
                    FindDescendant::new(AnyMatch, n).elements(ks, eh, &mut r);
                }
            },
            &Self::NextSibling(n) => match s {
                Selector::Mark(o) => {
                    FindNextSibling::new(MarkEq::new(o), n).elements(ks, eh, &mut r);
                }
                Selector::AttrName(o) => {
                    FindNextSibling::new(AttrExists::new(o), n).elements(ks, eh, &mut r);
                }
                Selector::Attribute(o, p) => {
                    let f = FindNextSibling::new(AttrMatch::new(o, p), n);
                    if matches!(o, AttrName::ID) {
                        if let Some(o) = f.elements_to_unique(ks, eh) {
                            r.push(o)
                        }
                    } else {
                        f.elements(ks, eh, &mut r);
                    }
                }
                Selector::Any => {
                    FindNextSibling::new(AnyMatch, n).elements(ks, eh, &mut r);
                }
            },
            &Self::PrecedingSibling(n) => match s {
                Selector::Mark(o) => {
                    FindPrecedingSibling::new(MarkEq::new(o), n).elements(ks, eh, &mut r);
                }
                Selector::AttrName(o) => {
                    FindPrecedingSibling::new(AttrExists::new(o), n).elements(ks, eh, &mut r);
                }
                Selector::Attribute(o, p) => {
                    let f = FindPrecedingSibling::new(AttrMatch::new(o, p), n);
                    if matches!(o, AttrName::ID) {
                        if let Some(o) = f.elements_to_unique(ks, eh) {
                            r.push(o)
                        }
                    } else {
                        f.elements(ks, eh, &mut r);
                    }
                }
                Selector::Any => {
                    FindPrecedingSibling::new(AnyMatch, n).elements(ks, eh, &mut r);
                }
            },
        }
        r
    }
}

impl FromStr for Combiner {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut o = s.chars();
        if let Some(c) = o.next() {
            let n = o.as_str();
            let n = if n.is_empty() { 0 } else { to_usize(n)? };
            match c {
                QUESTION => {
                    return Ok(Self::Descendant(n));
                }
                PLUS => {
                    return Ok(Self::NextSibling(n));
                }
                HYPHEN => {
                    return Ok(Self::PrecedingSibling(n));
                }
                _ => return Err((ErrorKind::Style, "invalid combiner").into()),
            }
        }
        Ok(Self::Descendant(0))
    }
}

#[derive(Debug, Default)]
#[repr(transparent)]
pub(crate) struct SelectorHolder(Vec<(Combiner, Selector)>);

deref!(SelectorHolder, Vec<(Combiner, Selector)>, 0);

impl SelectorHolder {
    pub(crate) fn find(&self, eh: &ElementHolder) -> Vec<ElementKey> {
        let mut v = Vec::new();
        let mut i = self.iter();
        if let Some((_, s)) = i.next() {
            match s {
                Selector::Mark(o) => {
                    FindDescendant::new(MarkEq::new(o), 0).all_with_first_root(eh, &mut v);
                }
                Selector::AttrName(o) => {
                    FindDescendant::new(AttrExists::new(o), 0).all_with_first_root(eh, &mut v);
                }
                Selector::Attribute(o, p) => {
                    let f = FindDescendant::new(AttrMatch::new(o, p), 0);
                    if matches!(o, AttrName::ID) {
                        if let Some(o) = f.unique_with_first_root(eh) {
                            v.push(o)
                        }
                    } else {
                        f.all_with_first_root(eh, &mut v);
                    }
                }
                Selector::Any => {
                    FindDescendant::new(AnyMatch, 0).all_with_first_root(eh, &mut v);
                }
            }
        }
        for (c, s) in i {
            v = c.find(s, &v, eh);
        }
        v
    }
}

#[derive(Debug)]
enum RuleKind {
    MetaRule,
    StyleRule,
}

impl Default for RuleKind {
    fn default() -> Self {
        Self::StyleRule
    }
}

impl RuleKind {}

#[derive(Debug, Default)]
pub(super) struct StyleSheetBuilder {
    a: MetaRule,
    b: StyleRule,
    k: RuleKind,
    rst: StyleSheet,
    error: ErrorHolder,
}

impl StyleSheetBuilder {
    pub(super) fn take(self) -> (StyleSheet, ErrorHolder) {
        (self.rst, self.error)
    }

    pub(super) fn build(s: &str) -> (StyleSheet, ErrorHolder) {
        Parser::new(Self::default()).parse_str(s).take()
    }
}

impl Output for StyleSheetBuilder {
    fn meta(&mut self, s: String) {
        self.a.key_mut().push_str(&s);
        self.k = RuleKind::MetaRule;
    }

    fn mark_selector(&mut self, c: String, s: String) {
        let c = result_return!(Combiner::from_str(&c).map_err(|e| self.error(e)));
        let m = result_return!(Mark::try_from(&s).map_err(|e| self.error(e)));
        let k = (c, Selector::Mark(m));
        self.b.key_mut().push(k);
    }

    fn attribute_selector(&mut self, c: String, k: String, v: String) {
        let c = result_return!(Combiner::from_str(&c).map_err(|e| self.error(e)));
        let a = if k.is_empty() {
            AttrName::CLASS
        } else {
            result_return!(AttrName::from_str(&k).map_err(|e| self.error(e)))
        };
        let k = (c, Selector::Attribute(a, AttrPattern::Contain(v)));
        self.b.key_mut().push(k);
    }

    fn attribute(&mut self, k: String, mut v: String) {
        let a = result_return!(Attribute::from_s(&k, &mut v).map_err(|e| self.error(e)));
        match self.k {
            RuleKind::MetaRule => {
                self.a.attribute_mut().insert(a.name(), a);
            }
            RuleKind::StyleRule => {
                self.b.attribute_mut().insert(a.name(), a);
            }
        }
    }

    fn end_block(&mut self) {
        match self.k {
            RuleKind::MetaRule => {
                self.rst.meta_rules_mut().push(take(&mut self.a));
            }
            RuleKind::StyleRule => {
                self.rst.style_rules_mut().push(take(&mut self.b));
            }
        }
        self.k = RuleKind::StyleRule;
    }

    fn error(&mut self, e: Error) {
        self.error.push(e)
    }
}
