use super::*;
use getset::{Getters, MutGetters};
use std::collections::{HashMap, VecDeque};

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
pub(super) struct AtRules {}

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
