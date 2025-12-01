use super::*;
use std::sync::{Arc, RwLock};

///"Head" represents head.
pub(crate) struct Head {
    element: Arc<RwLock<Element>>,
}

impl std::fmt::Debug for Head {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Head");
        if let Ok(o) = self.element.try_read() {
            f.field("element", &o.to_string());
        }
        f.finish()
    }
}

impl Head {
    pub(crate) fn new(element: Arc<RwLock<Element>>) -> Self {
        Self { element }
    }
}

///"Title" represents title.
pub(crate) struct Title {
    element: Arc<RwLock<Element>>,
}

impl std::fmt::Debug for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Title");
        if let Ok(o) = self.element.try_read() {
            f.field("element", &o.to_string());
        }
        f.finish()
    }
}

impl Title {
    pub(crate) fn new(element: Arc<RwLock<Element>>) -> Self {
        Self { element }
    }
}

///"Aht" represents root.
pub(crate) struct Aht {
    element: Arc<RwLock<Element>>,
}

impl std::fmt::Debug for Aht {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Aht");
        if let Ok(o) = self.element.try_read() {
            f.field("element", &o.to_string());
        }
        f.finish()
    }
}

impl Aht {
    pub(crate) fn new(element: Arc<RwLock<Element>>) -> Self {
        Self { element }
    }
}
