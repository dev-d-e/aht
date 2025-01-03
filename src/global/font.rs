use skia_safe::{Font, FontMgr, FontStyle};
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, OnceLock, RwLock};

static DEFAULT_FONT: OnceLock<Arc<Font>> = OnceLock::new();
static APPLIED_FONTS: LazyLock<RwLock<FontHolder>> =
    LazyLock::new(|| RwLock::new(FontHolder::new()));

#[derive(Debug)]
struct FontHolder(HashMap<String, Arc<Font>>);

impl FontHolder {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn contains_key(&self, k: &str) -> bool {
        self.0.contains_key(k)
    }

    fn get(&self, k: &str) -> Option<Arc<Font>> {
        self.0.get(k).cloned()
    }

    fn insert(&mut self, k: impl Into<String>, v: Font) {
        self.0.insert(k.into(), Arc::new(v));
    }

    fn check_remove(&mut self, k: &str) {
        if let Some(v) = self.0.get(k) {
            if Arc::strong_count(v) == 2 {
                self.0.remove(k);
            }
        }
    }
}

pub(super) fn set_default_font(font: Font) {
    let _ = DEFAULT_FONT.set(Arc::new(font));
}

pub(crate) fn default_font() -> Arc<Font> {
    DEFAULT_FONT
        .get_or_init(|| Arc::new(get_default_font()))
        .clone()
}

pub(crate) fn get_applied(s: &str) -> Option<Arc<Font>> {
    let mut n = false;
    if let Ok(o) = APPLIED_FONTS.read() {
        n = !o.contains_key(s);
    }
    if n {
        if let Ok(mut o) = APPLIED_FONTS.write() {
            if let Some(f) = get_font(s) {
                o.insert(s, f);
            }
        }
    }
    if let Ok(o) = APPLIED_FONTS.read() {
        return o.get(s);
    }
    None
}

pub(crate) fn check_applied(s: &str) {
    if let Ok(mut o) = APPLIED_FONTS.write() {
        o.check_remove(s);
    }
}

pub(super) fn get_font(s: &str) -> Option<Font> {
    FontMgr::default()
        .match_family_style(s, FontStyle::normal())
        .map(|tf| Font::from_typeface(tf, None))
}

fn get_default_font() -> Font {
    let fm = FontMgr::default();
    let tf = fm
        .family_names()
        .find_map(|n| fm.match_family_style(n, FontStyle::normal()))
        .unwrap();
    Font::from_typeface(tf, None)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn font_0() {
        let font = default_font();
        println!("font: {:?} ", font);
        println!("strong_count: {:?} ", Arc::strong_count(&font));
    }
}
