use super::{Border, DrawUnitWrapper, OutPainter};
use crate::grid::{AlignPattern, Painter, Range, ScrollBar};
use crate::markup::Page;
use skia_safe::Canvas;

///"Audio" represents audio stream.
#[derive(Debug)]
pub(crate) struct Audio {
    background: Box<dyn Painter>,
    align_pattern: AlignPattern,
}

impl Audio {
    pub(crate) fn new() -> Self {
        Self {
            background: Box::new(Range::new()),
            align_pattern: AlignPattern::center_middle(),
        }
    }

    pub(crate) fn draw(&mut self, canvas: &Canvas, page: &mut Page, wrapper: &mut DrawUnitWrapper) {
        let r = wrapper.rect();

        self.background.as_mut().act(&r, canvas);
    }
}

///"Img" represents an image.
#[derive(Debug)]
pub(crate) struct Img {
    background: Box<dyn Painter>,
    align_pattern: AlignPattern,
    outside: Box<dyn OutPainter>,
    scroll_bar: ScrollBar,
}

impl Img {
    pub(crate) fn new() -> Self {
        Self {
            background: Box::new(Range::new()),
            align_pattern: AlignPattern::center_middle(),
            outside: Box::new(Border::new()),
            scroll_bar: ScrollBar::new(),
        }
    }

    pub(crate) fn draw(&mut self, canvas: &Canvas, page: &mut Page, wrapper: &mut DrawUnitWrapper) {
        let r = wrapper.rect();

        self.outside.as_mut().act(&r, canvas);
        self.background.as_mut().act(&r, canvas);
    }
}

///"Video" represents video.
#[derive(Debug)]
pub(crate) struct Video {
    background: Box<dyn Painter>,
    align_pattern: AlignPattern,
}

impl Video {
    pub(crate) fn new() -> Self {
        Self {
            background: Box::new(Range::new()),
            align_pattern: AlignPattern::center_middle(),
        }
    }

    pub(crate) fn draw(&mut self, canvas: &Canvas, page: &mut Page, wrapper: &mut DrawUnitWrapper) {
        let r = wrapper.rect();

        self.background.as_mut().act(&r, canvas);
    }
}
