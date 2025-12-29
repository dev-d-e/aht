use super::*;
use getset::{Getters, MutGetters};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Default, Getters, MutGetters)]
pub(super) struct StyleSheet {
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    at_rules: AtRules,
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    style_rules: StyleRules,
}

impl StyleSheet {
    pub(super) fn new(at_rules: AtRules, style_rules: StyleRules) -> Self {
        Self {
            at_rules,
            style_rules,
        }
    }
}

#[derive(Debug, Default)]
pub(super) struct AtRules(Vec<AtRule>);

deref!(AtRules, Vec<AtRule>, 0);

#[derive(Debug, Default, Getters, MutGetters)]
pub(super) struct AtRule {
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    key: String,
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    attribute: HashMap<AttrName, Attribute>,
}

impl AtRule {
    fn new(key: String) -> Self {
        Self {
            key,
            attribute: Default::default(),
        }
    }
}

#[derive(Debug, Default)]
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

#[derive(Default)]
pub(crate) struct SelectorHolder(Vec<(Selector, Combiner)>);

deref!(SelectorHolder, Vec<(Selector, Combiner)>, 0);

impl std::fmt::Debug for SelectorHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(&self.0).finish()
    }
}

impl SelectorHolder {
    pub(crate) fn find(&self, mut v: Vec<Arc<RwLock<Element>>>) -> Vec<Arc<RwLock<Element>>> {
        let mut o = &Combiner::Descendant(0);
        for (k, c) in self.iter() {
            let mut r = Vec::new();
            match o {
                Combiner::Descendant(n) => match k {
                    Selector::Mark(o) => {
                        let mut f = FindDescendant::new(MarkEq::new(o), *n);
                        v.iter().for_each(|k| f.find(k, &mut r));
                    }
                    Selector::AttrName(o) => {
                        let mut f = FindDescendant::new(AttrExists::new(o), *n);
                        v.iter().for_each(|k| f.find(k, &mut r));
                    }
                    Selector::Attribute(o, s) => {
                        let mut f = FindDescendant::new(AttrMatch::new(o, s), *n);
                        if matches!(o, AttrName::ID) {
                            if let Some(o) = v.iter().find_map(|k| f.find_unique(k)) {
                                r.push(o)
                            }
                        } else {
                            v.iter().for_each(|k| f.find(k, &mut r));
                        }
                    }
                    Selector::Any => {
                        let mut f = FindDescendant::new(AnyMatch, *n);
                        v.iter().for_each(|k| f.find(k, &mut r));
                    }
                },
                Combiner::NextSibling(n) => match k {
                    Selector::Mark(o) => {
                        let mut f = FindNextSibling::new(MarkEq::new(o), *n);
                        v.iter().for_each(|k| f.find(k, &mut r));
                    }
                    Selector::AttrName(o) => {
                        let mut f = FindNextSibling::new(AttrExists::new(o), *n);
                        v.iter().for_each(|k| f.find(k, &mut r));
                    }
                    Selector::Attribute(o, s) => {
                        let mut f = FindNextSibling::new(AttrMatch::new(o, s), *n);
                        if matches!(o, AttrName::ID) {
                            if let Some(o) = v.iter().find_map(|k| f.find_unique(k)) {
                                r.push(o)
                            }
                        } else {
                            v.iter().for_each(|k| f.find(k, &mut r));
                        }
                    }
                    Selector::Any => {
                        let mut f = FindNextSibling::new(AnyMatch, *n);
                        v.iter().for_each(|k| f.find(k, &mut r));
                    }
                },
                Combiner::PrecedingSibling(n) => match k {
                    Selector::Mark(o) => {
                        let mut f = FindPrecedingSibling::new(MarkEq::new(o), *n);
                        v.iter().for_each(|k| f.find(k, &mut r));
                    }
                    Selector::AttrName(o) => {
                        let mut f = FindPrecedingSibling::new(AttrExists::new(o), *n);
                        v.iter().for_each(|k| f.find(k, &mut r));
                    }
                    Selector::Attribute(o, s) => {
                        let mut f = FindPrecedingSibling::new(AttrMatch::new(o, s), *n);
                        if matches!(o, AttrName::ID) {
                            if let Some(o) = v.iter().find_map(|k| f.find_unique(k)) {
                                r.push(o)
                            }
                        } else {
                            v.iter().for_each(|k| f.find(k, &mut r));
                        }
                    }
                    Selector::Any => {
                        let mut f = FindPrecedingSibling::new(AnyMatch, *n);
                        v.iter().for_each(|k| f.find(k, &mut r));
                    }
                },
            }
            v = r;
            o = c;
        }
        v
    }
}

#[derive(Debug)]
enum RuleKind {
    AtRule,
    StyleRule,
}

impl Default for RuleKind {
    fn default() -> Self {
        Self::StyleRule
    }
}

impl RuleKind {}

#[derive(Debug, Default, Getters, MutGetters)]
pub(super) struct StyleSheetBuilder {
    o: StyleRule,
    a: AtRule,
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
    fn at(&mut self, s: String) {
        self.a.key_mut().push_str(&s);
        self.k = RuleKind::AtRule;
    }

    fn mark_selector(&mut self, s: String) {
        if let Ok(m) = Mark::try_from(&s) {
            let k = (Selector::Mark(m), Combiner::Descendant(0));
            self.o.key_mut().push(k);
        }
    }

    fn attribute_selector(&mut self, k: String, v: String) {
        if k.is_empty() {
            let k = (
                Selector::Attribute(AttrName::CLASS, AttrPattern::Contain(v)),
                Combiner::Descendant(0),
            );
            self.o.key_mut().push(k);
        } else {
            match AttrName::from_str(&k) {
                Ok(a) => {
                    let k = (
                        Selector::Attribute(a, AttrPattern::Contain(v)),
                        Combiner::Descendant(0),
                    );
                    self.o.key_mut().push(k);
                }
                Err(e) => {
                    error!("{e}");
                }
            }
        }
    }

    fn attribute(&mut self, k: String, mut v: String) {
        match Attribute::from_s(&k, &mut v) {
            Ok(a) => match self.k {
                RuleKind::AtRule => {
                    self.a.attribute_mut().insert(a.name(), a);
                }
                RuleKind::StyleRule => {
                    self.o.attribute_mut().insert(a.name(), a);
                }
            },
            Err(e) => {
                error!("{e}");
            }
        }
    }

    fn end_block(&mut self) {
        match self.k {
            RuleKind::AtRule => {
                let a = std::mem::take(&mut self.a);
                self.rst.at_rules_mut().push(a);
            }
            RuleKind::StyleRule => {
                let o = std::mem::take(&mut self.o);
                self.rst.style_rules_mut().push(o);
            }
        }
        self.k = RuleKind::StyleRule;
    }

    fn error(&mut self, e: Error) {
        self.error.push(e)
    }
}
