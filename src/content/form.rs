use super::{Border, DrawUnitWrapper, OutPainter};
use crate::global::*;
use crate::grid::{AlignPattern, ApplyFont, Painter, Range, ScrollBar};
use crate::markup::{Mark, Page, VisionActionResult};
use crate::parts::{Chronograph, Coord2D};
use skia_safe::Canvas;
use std::sync::{Arc, RwLock};

///"Button" represents a button.
#[derive(Debug)]
pub(crate) struct Button {
    background: Box<dyn Painter>,
    align_pattern: AlignPattern,
    apply_font: ApplyFont,
    time_meter: Chronograph,
}

impl Button {
    pub(crate) fn new() -> Self {
        Self {
            background: Box::new(range!(*default_surface_color(), 10, 10)),
            align_pattern: AlignPattern::center_middle(),
            apply_font: ApplyFont::new(),
            time_meter: Chronograph::new(1000),
        }
    }

    pub(crate) fn draw(&mut self, canvas: &Canvas, page: &mut Page, wrapper: &mut DrawUnitWrapper) {
        let r = wrapper.rect();

        if self.time_meter.elapsed() {
            self.background.as_mut().set_color(*default_surface_color());
        }
        if wrapper.cursor {
            if let Some(a) = page.cursor.analyse() {
                match a.1 {
                    VisionActionResult::Press(_) => {
                        self.background.as_mut().set_color(*default_button_color());
                        self.time_meter.refresh();
                    }
                    _ => {}
                }
            }
        }

        self.background.as_mut().act(&r, canvas);

        if let Ok(e) = wrapper.element.read() {
            self.apply_font
                .draw(&r, &self.align_pattern, &e.text, canvas);
        }
    }
}

///"Form" represents form.
#[derive(Debug)]
pub(crate) struct Form {}

impl Form {
    pub(crate) fn new() -> Self {
        Self {}
    }

    fn fit(&mut self, mark_type: &Mark) -> bool {
        match mark_type {
            Mark::INP | Mark::PT | Mark::SELECT | Mark::TIME => true,
            _ => false,
        }
    }
}

///"Inp" represents input.
pub(crate) struct Inp {
    input: Arc<RwLock<String>>,
    background: Box<dyn Painter>,
    align_pattern: AlignPattern,
    apply_font: ApplyFont,
    outside: Box<dyn OutPainter>,
    scroll_bar: ScrollBar,
}

impl std::fmt::Debug for Inp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Inp");
        if let Ok(o) = self.input.try_read() {
            f.field("input", &o);
        }
        f.field("background", &self.background)
            .field("align_pattern", &self.align_pattern)
            .field("apply_font", &self.apply_font)
            .field("outside", &self.outside)
            .field("scroll_bar", &self.scroll_bar)
            .finish()
    }
}

impl Inp {
    pub(crate) fn new() -> Self {
        Self {
            input: Arc::new(RwLock::new(String::new())),
            background: Box::new(Range::new()),
            align_pattern: AlignPattern::left_middle(),
            apply_font: ApplyFont::new(),
            outside: Box::new(Border::new()),
            scroll_bar: ScrollBar::new(),
        }
    }

    right_bottom!();

    pub(crate) fn draw(&mut self, canvas: &Canvas, page: &mut Page, wrapper: &mut DrawUnitWrapper) {
        let r = wrapper.rect();

        self.outside.as_mut().act(&r, canvas);
        self.background.as_mut().act(&r, canvas);

        if let Some(a) = page.cursor.analyse() {
            match a.1 {
                VisionActionResult::Press(_) => {
                    if wrapper.cursor {
                        self.apply_font.set_cursor(true);
                        page.keyboard_input.replace(self.input.clone());
                    } else {
                        self.apply_font.set_cursor(false);
                        page.keyboard_input.take();
                    }
                }
                _ => {}
            }
        }

        if let Ok(mut i) = self.input.try_write() {
            if i.len() > 0 {
                if let Ok(mut e) = wrapper.element.try_write() {
                    e.text.push_str(i.drain(..).as_str());
                }
            }
        }
        if let Ok(e) = wrapper.element.read() {
            self.apply_font
                .draw(&r, &self.align_pattern, &e.text, canvas);
        }
    }
}

///"Opt" represents an option.
#[derive(Debug)]
pub(crate) struct Opt {
    background: Box<dyn Painter>,
    align_pattern: AlignPattern,
    apply_font: ApplyFont,
    outside: Box<dyn OutPainter>,
}

impl Opt {
    pub(crate) fn new() -> Self {
        Self {
            background: Box::new(Range::new()),
            align_pattern: AlignPattern::left_middle(),
            apply_font: ApplyFont::new(),
            outside: Box::new(Border::new()),
        }
    }

    pub(crate) fn draw(&mut self, canvas: &Canvas, page: &mut Page, wrapper: &mut DrawUnitWrapper) {
        let r = wrapper.rect();

        self.outside.as_mut().act(&r, canvas);
        self.background.as_mut().act(&r, canvas);

        if let Ok(e) = wrapper.element.read() {
            self.apply_font
                .draw(&r, &self.align_pattern, &e.text, canvas);
        }
    }
}

///"Pt" represents plain text.
#[derive(Debug)]
pub(crate) struct Pt {
    background: Box<dyn Painter>,
    align_pattern: AlignPattern,
    apply_font: ApplyFont,
    outside: Box<dyn OutPainter>,
}

impl Pt {
    pub(crate) fn new() -> Self {
        Self {
            background: Box::new(Range::new()),
            align_pattern: AlignPattern::left_middle(),
            apply_font: ApplyFont::new(),
            outside: Box::new(Border::new()),
        }
    }

    right_bottom!();

    pub(crate) fn draw(&mut self, canvas: &Canvas, page: &mut Page, wrapper: &mut DrawUnitWrapper) {
        let r = wrapper.rect();

        self.outside.as_mut().act(&r, canvas);
        self.background.as_mut().act(&r, canvas);

        if let Ok(e) = wrapper.element.read() {
            self.apply_font
                .draw(&r, &self.align_pattern, &e.text, canvas);
        }
    }
}

///"Select" represents a select.
#[derive(Debug)]
pub(crate) struct Select {
    background: Box<dyn Painter>,
    align_pattern: AlignPattern,
    apply_font: ApplyFont,
    outside: Box<dyn OutPainter>,
    scroll_bar: ScrollBar,
}

impl Select {
    pub(crate) fn new() -> Self {
        Self {
            background: Box::new(Range::new()),
            align_pattern: AlignPattern::left_middle(),
            apply_font: ApplyFont::new(),
            outside: Box::new(Border::new()),
            scroll_bar: ScrollBar::new(),
        }
    }

    right_bottom!();

    pub(crate) fn draw(&mut self, canvas: &Canvas, page: &mut Page, wrapper: &mut DrawUnitWrapper) {
        let r = wrapper.rect();

        self.outside.as_mut().act(&r, canvas);
        self.background.as_mut().act(&r, canvas);

        if let Ok(e) = wrapper.element.read() {
            self.apply_font
                .draw(&r, &self.align_pattern, &e.text, canvas);
        }
    }
}

///"Time" represents date time.
#[derive(Debug)]
pub(crate) struct Time {
    background: Box<dyn Painter>,
    align_pattern: AlignPattern,
    apply_font: ApplyFont,
    outside: Box<dyn OutPainter>,
}

impl Time {
    pub(crate) fn new() -> Self {
        Self {
            background: Box::new(Range::new()),
            align_pattern: AlignPattern::left_middle(),
            apply_font: ApplyFont::new(),
            outside: Box::new(Border::new()),
        }
    }

    right_bottom!();

    pub(crate) fn draw(&mut self, canvas: &Canvas, page: &mut Page, wrapper: &mut DrawUnitWrapper) {
        let r = wrapper.rect();

        self.outside.as_mut().act(&r, canvas);
        self.background.as_mut().act(&r, canvas);

        if let Ok(e) = wrapper.element.read() {
            self.apply_font
                .draw(&r, &self.align_pattern, &e.text, canvas);
        }
    }
}
