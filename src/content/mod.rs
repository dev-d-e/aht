mod common;
mod form;
mod media;
mod other;

use crate::grid::{AlignPattern, DrawUnit, Grid, Sides};
use crate::markup::{AttrName, Attribute, Element, Mark, Page, VisionActionResult};
use crate::parts::{Coord, Coord2D, FixedRect, Ordinal};
use common::*;
pub(crate) use form::{Button, Form, Inp, Opt, Pt, Select, Time};
pub(crate) use media::{Audio, Img, Video};
pub(crate) use other::{Canv, Iframe};
use skia_safe::Canvas;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, RwLock};

///"Body" is grid layout.
pub(crate) struct Body {
    element: Arc<RwLock<Element>>,
    subset: DrawUnitWrapperHolder,
    zero: Coord,
    side: Sides,
    background: Box<dyn Painter>,
    align_pattern: AlignPattern,
    grid: Grid,
    scroll_bar: ScrollBar,
}

impl std::fmt::Debug for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Body");
        if let Ok(o) = self.element.try_read() {
            f.field("element", &o.to_string());
        }
        f.field("zero", &self.zero)
            .field("side", &self.side)
            .field("background", &self.background)
            .field("align_pattern", &self.align_pattern)
            .field("grid", &self.grid)
            .field("scroll_bar", &self.scroll_bar)
            .field("subset", &self.subset)
            .finish()
    }
}

impl Body {
    pub(crate) fn new(element: Arc<RwLock<Element>>) -> Self {
        let mut o = Self {
            element,
            subset: DrawUnitWrapperHolder::new(),
            zero: Coord::new(),
            side: Sides::new(100, 100),
            background: Box::new(Range::new()),
            align_pattern: AlignPattern::left_middle(),
            grid: Grid::new(),
            scroll_bar: ScrollBar::new(),
        };
        o.renew_subset();
        o
    }

    pub(crate) fn renew_subset(&mut self) {
        renew_subset(&self.element, &mut self.subset);
    }

    element!();

    zero!();

    pub(crate) fn resize(&mut self, w: isize, h: isize) {
        self.side.reserve(w, h);
        if let Ok(e) = self.element.read() {
            self.side.get_attr(&e);
            self.grid.get_attr(&e, self.side.effect(), &self.zero);
        }
        self.subset.resize(&mut self.grid);
    }

    pub(crate) fn draw(&mut self, canvas: &Canvas, page: &mut Page) {
        let r = self.side.to_rect(&self.zero);
        if r.is_empty() {
            return;
        }

        self.subset.pop_cursor(page);

        self.background.as_mut().act(&r, canvas);
        canvas.save();

        subset_draw(&mut self.subset, page, &mut self.scroll_bar, &r, canvas);
    }

    pub(crate) fn find_wrapper(&mut self, e: Arc<RwLock<Element>>) -> Option<BodyOrWrapper> {
        if Arc::as_ptr(&self.element) == Arc::as_ptr(&e) {
            Some(BodyOrWrapper::BODY(self as *mut Self))
        } else {
            self.subset.find_wrapper(e)
        }
    }
}

fn renew_subset(element: &Arc<RwLock<Element>>, subset: &mut DrawUnitWrapperHolder) {
    if let Ok(e) = element.read() {
        for i in e.subset.iter() {
            if let Ok(o) = i.read() {
                let d = DrawUnitWrapper::new(&o.mark_type, i.clone());
                subset.push(d)
            }
        }
    }
}

///"Area" is grid layout.
#[derive(Debug)]
pub(crate) struct Area {
    background: Box<dyn Painter>,
    align_pattern: AlignPattern,
    grid: Grid,
    scroll_bar: ScrollBar,
}

impl Area {
    pub(crate) fn new() -> Self {
        Self {
            background: Box::new(Range::new()),
            align_pattern: AlignPattern::left_middle(),
            grid: Grid::new(),
            scroll_bar: ScrollBar::new(),
        }
    }

    pub(self) fn draw(&mut self, wrapper: &mut DrawUnitWrapper, page: &mut Page, canvas: &Canvas) {
        let r = wrapper.rect();
        self.background.as_mut().act(&r, canvas);
        canvas.save();

        subset_draw(wrapper.subset(), page, &mut self.scroll_bar, &r, canvas);
    }
}

fn subset_draw(
    subset: &mut DrawUnitWrapperHolder,
    page: &mut Page,
    scroll_bar: &mut ScrollBar,
    r: &FixedRect,
    canvas: &Canvas,
) {
    if let Some(mut surface) = unsafe { canvas.surface() } {
        let right_bottom = if let Some(right_bottom) = subset.right_bottom() {
            right_bottom
        } else {
            return;
        };
        let s = right_bottom.away_from(&r.pos);

        if let Some((c, VisionActionResult::PressSweep(a))) = page.cursor.analyse() {
            scroll_bar.cursor_move(c, &a);
        }

        let vision_start = scroll_bar.resize(&r, &s);
        let rr = r.move_xy(vision_start.width, vision_start.height);
        let info = surface.image_info().with_dimensions(rr.right_bottom());
        if let Some(mut surface2) = surface.new_surface(&info) {
            let canvas2 = surface2.canvas();
            subset.draw(page, canvas2);

            if let Some(image2) = surface2.image_snapshot_with_bounds(rr.to_irect()) {
                canvas.draw_image(image2, r.pos.clone(), None);
                scroll_bar.draw(canvas);
            }
        }
    }
}

#[derive(Debug)]
pub(crate) enum BodyOrWrapper {
    BODY(*mut Body),
    DRAWUNITWRAPPER(*mut DrawUnitWrapper),
}

//------------------------------------------------------------------------------------------

pub(crate) struct DrawUnitWrapper {
    element: Arc<RwLock<Element>>,
    draw_unit: DrawUnit,
    subset: DrawUnitWrapperHolder,
    cursor: bool,
    zero: Coord,
    side: Sides,
    hidden: bool,
}

impl std::fmt::Debug for DrawUnitWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("DrawUnitWrapper");
        if let Ok(o) = self.element.try_read() {
            f.field("element", &o.to_string());
        }
        f.field("cursor", &self.cursor)
            .field("zero", &self.zero)
            .field("side", &self.side)
            .field("hidden", &self.hidden)
            .field("draw_unit", &self.draw_unit)
            .field("subset", &self.subset)
            .finish()
    }
}

impl DrawUnitWrapper {
    pub(crate) fn new(mark_type: &Mark, element: Arc<RwLock<Element>>) -> Self {
        let mut o = Self {
            element,
            draw_unit: DrawUnit::from(mark_type),
            subset: DrawUnitWrapperHolder::new(),
            cursor: false,
            zero: Coord::new(),
            side: Sides::new(100, 100),
            hidden: false,
        };
        renew_subset(&o.element, &mut o.subset);
        o
    }

    pub(crate) fn subset(&mut self) -> &mut DrawUnitWrapperHolder {
        &mut self.subset
    }

    element!();

    pub(self) fn rect(&self) -> FixedRect {
        self.side.to_rect(&self.zero)
    }

    fn is_empty(&self) -> bool {
        self.side.effect().is_empty()
    }

    fn resize_subset(&mut self) {
        match &mut self.draw_unit {
            DrawUnit::AREA(o) => {
                self.subset.resize(&mut o.grid);
            }
            _ => {}
        }
    }

    pub(crate) fn resize_grid(&mut self, xy: &mut Grid) {
        if let Ok(e) = self.element.read() {
            let a = if let Some(Attribute::ORDINAL(a)) = e.attribute.get(&AttrName::ORDINAL) {
                a
            } else {
                &Ordinal::None
            };
            if let Some(r) = xy.next(&a) {
                self.zero.from_2d(&r.pos);
                self.side.replace(&r.side);
            } else {
                self.hidden = true;
            }
            if let DrawUnit::AREA(o) = &mut self.draw_unit {
                self.side.get_attr(&e);
                o.grid.get_attr(&e, self.side.effect(), &self.zero);
            }
        }
        if !self.hidden {
            self.resize_subset();
        }
    }

    pub(crate) fn right_bottom(&self) -> Option<Coord2D> {
        if self.hidden {
            return None;
        }
        match &self.draw_unit {
            DrawUnit::AREA(_)
            | DrawUnit::AUDIO(_)
            | DrawUnit::BUTTON(_)
            | DrawUnit::CANVAS(_)
            | DrawUnit::IFRAME(_)
            | DrawUnit::IMG(_)
            | DrawUnit::VIDEO(_) => Some(self.zero.move_rect_to_2d(self.side.effect())),
            DrawUnit::INP(o) => Some(o.right_bottom()),
            DrawUnit::OPTION(_) => None,
            DrawUnit::PT(o) => Some(o.right_bottom()),
            DrawUnit::SELECT(o) => Some(o.right_bottom()),
            DrawUnit::TIME(o) => Some(o.right_bottom()),
            _ => None,
        }
    }

    fn pop_cursor(&mut self, page: &mut Page) -> bool {
        self.cursor = false;
        if let Some(c) = page.cursor.position() {
            if self.rect().within(c) {
                self.cursor = true;
                return true;
            }
        }
        false
    }

    pub(crate) fn draw(&mut self, page: &mut Page, canvas: &Canvas) -> bool {
        if self.hidden {
            return false;
        }
        if let Ok(e) = self.element.read() {
            if let Some(Attribute::HIDDEN(a)) = e.attribute.get(&AttrName::HIDDEN) {
                if *a {
                    return false;
                }
            }

            if let Some(Attribute::DISABLED(a)) = e.attribute.get(&AttrName::DISABLED) {
                if *a {
                    return false;
                }
            }
        }

        if self.is_empty() {
            return false;
        }

        let p = self as *mut Self;
        match &mut self.draw_unit {
            DrawUnit::AREA(o) => {
                o.draw(unsafe { &mut *p }, page, canvas);
            }
            DrawUnit::AUDIO(o) => {
                o.draw(canvas, page, unsafe { &mut *p });
            }
            DrawUnit::BUTTON(o) => {
                o.draw(canvas, page, unsafe { &mut *p });
                self.subset.draw(page, canvas);
            }
            DrawUnit::CANVAS(o) => {
                o.draw(canvas, page, unsafe { &mut *p });
                self.subset.draw(page, canvas);
            }
            DrawUnit::IFRAME(o) => {
                o.draw(canvas, page, unsafe { &mut *p });
                self.subset.draw(page, canvas);
            }
            DrawUnit::IMG(o) => {
                o.draw(canvas, page, unsafe { &mut *p });
                self.subset.draw(page, canvas);
            }
            DrawUnit::INP(o) => {
                o.draw(canvas, page, unsafe { &mut *p });
                self.subset.draw(page, canvas);
            }
            DrawUnit::OPTION(_) => {}
            DrawUnit::PT(o) => {
                o.draw(canvas, page, unsafe { &mut *p });
                self.subset.draw(page, canvas);
            }
            DrawUnit::SELECT(o) => {
                o.draw(canvas, page, unsafe { &mut *p });
                self.subset.draw(page, canvas);
            }
            DrawUnit::TIME(o) => {
                o.draw(canvas, page, unsafe { &mut *p });
                self.subset.draw(page, canvas);
            }
            DrawUnit::VIDEO(o) => {
                o.draw(canvas, page, unsafe { &mut *p });
                self.subset.draw(page, canvas);
            }
            _ => {}
        }
        false
    }
}

pub(crate) struct DrawUnitWrapperHolder(Vec<DrawUnitWrapper>);

impl std::fmt::Debug for DrawUnitWrapperHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(&self.0).finish()
    }
}

impl DrawUnitWrapperHolder {
    fn new() -> Self {
        Self(Vec::new())
    }

    pub(crate) fn resize(&mut self, xy: &mut Grid) {
        for i in self.0.iter_mut() {
            i.resize_grid(xy);
        }
    }

    pub(crate) fn right_bottom(&self) -> Option<Coord2D> {
        let mut c: Option<Coord2D> = None;
        for i in self.0.iter() {
            if let Some(r) = i.right_bottom() {
                if let Some(c) = c.as_mut() {
                    if c.x < r.x {
                        c.x = r.x;
                    }
                    if c.y < r.y {
                        c.y = r.y;
                    }
                } else {
                    c.replace(r);
                }
            }
        }
        c
    }

    pub(crate) fn pop_cursor(&mut self, page: &mut Page) -> bool {
        let mut v = self.0.iter_mut();
        while let Some(i) = v.next_back() {
            if i.pop_cursor(page) {
                return true;
            }
        }
        false
    }

    pub(crate) fn draw(&mut self, page: &mut Page, canvas: &Canvas) {
        for e in self.0.iter_mut() {
            e.draw(page, canvas);
            e.cursor = false;
        }
    }

    pub(crate) fn find_wrapper(&mut self, p: Arc<RwLock<Element>>) -> Option<BodyOrWrapper> {
        for e in self.0.iter_mut() {
            if Arc::as_ptr(&e.element) == Arc::as_ptr(&p) {
                return Some(BodyOrWrapper::DRAWUNITWRAPPER(e));
            } else if let Some(e) = e.subset.find_wrapper(p.clone()) {
                return Some(e);
            }
        }
        None
    }
}

impl Deref for DrawUnitWrapperHolder {
    type Target = Vec<DrawUnitWrapper>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DrawUnitWrapperHolder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
