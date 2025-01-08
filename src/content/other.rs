use super::*;
use crate::markup::Page;
use skia_safe::Canvas;

///"Canv" represents canvas.
#[derive(Debug)]
pub(crate) struct Canv {
    background: Box<dyn Painter>,
    scroll_bar: ScrollBar,
}

impl Canv {
    pub(crate) fn new() -> Self {
        Self {
            background: Box::new(Range::new()),
            scroll_bar: ScrollBar::new(),
        }
    }

    pub(crate) fn draw(&mut self, canvas: &Canvas, page: &mut Page, wrapper: &mut DrawUnitWrapper) {
        let r = wrapper.rect();

        self.background.as_mut().act(&r, canvas);
    }
}

///"Iframe" represents iframe.
#[derive(Debug)]
pub(crate) struct Iframe {
    background: Box<dyn Painter>,
}

impl Iframe {
    pub(crate) fn new() -> Self {
        Self {
            background: Box::new(Range::new()),
        }
    }

    pub(crate) fn draw(&mut self, canvas: &Canvas, page: &mut Page, wrapper: &mut DrawUnitWrapper) {
        let r = wrapper.rect();

        self.background.as_mut().act(&r, canvas);
    }
}
