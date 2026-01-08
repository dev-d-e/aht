mod appearance;
mod common;
mod form;
mod media;
mod other;

use self::appearance::*;
use self::common::*;
use self::form::*;
use self::media::*;
use self::other::*;
use crate::global::*;
use crate::markup::*;
use crate::page::*;
use crate::utils::*;
use getset::{Getters, MutGetters};
use skia_safe::{Paint, Surface};
use std::sync::{Arc, RwLock};

#[derive(Getters, MutGetters)]
pub(crate) struct Body {
    #[getset(get = "pub(crate)")]
    element: Arc<RwLock<Element>>,
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    subset: DrawUnitHolder,
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    rect: FixedRect,
    painter: AppearanceComposite,
    align_pattern: AlignPattern,
    layout: LayoutCoord,
    scroll_bar: ScrollBar,
}

impl std::fmt::Debug for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Body");
        if let Ok(o) = self.element.try_read() {
            f.field("element", &o.to_string());
        }
        f.field("rect", &self.rect)
            .field("painter", &self.painter)
            .field("align_pattern", &self.align_pattern)
            .field("layout", &self.layout)
            .field("scroll_bar", &self.scroll_bar)
            .field("subset", &self.subset)
            .finish()
    }
}

impl Body {
    pub(crate) fn new(element: Arc<RwLock<Element>>) -> Self {
        let mut o = Self {
            element,
            subset: Default::default(),
            rect: (100.0, 100.0).into(),
            painter: Rectangle::default().into(),
            align_pattern: Default::default(),
            layout: Default::default(),
            scroll_bar: Default::default(),
        };
        o.renew_subset();
        o
    }

    pub(crate) fn renew_subset(&mut self) {
        renew_subset(&self.element, &mut self.subset);
    }

    pub(crate) fn resize(&mut self, w: f32, h: f32) {
        let k = RectSide::new(w, h);
        self.rect.set_side(k.clone());
        if let Ok(e) = self.element.read() {
            self.rect.side_mut().get_attr(&e, &k);
            self.layout.get_attr(&e, self.rect.clone());
        }
        self.subset.resize(&mut self.layout);
    }

    pub(crate) fn reset(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.rect.set_x(x);
        self.rect.set_y(y);
        self.resize(w, h);
    }

    pub(crate) fn draw(&mut self, mut t: DrawCtx) {
        if self.rect.is_empty() {
            return;
        }

        t.surface.canvas().clear(*default_bg_color());

        self.painter.draw(&self.rect, &mut t);
        t.surface.canvas().save();

        subset_draw(&mut self.subset, &self.rect, &mut self.scroll_bar, &mut t);
        t.surface.canvas().save();
    }

    pub(crate) fn consume_action(&mut self, mut t: ActionCtx) {
        match &t.kind {
            ActionKind::Click(c, _)
            | ActionKind::DoubleClick(c, _)
            | ActionKind::Cursor(c)
            | ActionKind::CursorWithoutFocus(c)
            | ActionKind::DeleteFront(c, _)
            | ActionKind::DeleteBack(c, _) => {
                if self.scroll_bar.within(c) {
                    return;
                }
                let (x, y) = self.scroll_bar.vision_var();
                t.kind.set_var_cursor(x, y);
            }
            ActionKind::Sweep(a, b) => {
                if self.scroll_bar.within(b) {
                    self.scroll_bar.move_a_to_b(a, b);
                    return;
                }
            }
            ActionKind::InputStr(_) => {
                return;
            }
            _ => {}
        }
        self.subset.consume_action(&mut t);
    }

    fn callback_action(&mut self) -> impl FnMut(&ActionKind, &mut PageContext) {
        let o = self as *mut Self;
        move |a, context| match &a {
            ActionKind::Click(c, _) | ActionKind::DoubleClick(c, _) => {
                if let Some(o) = unsafe { o.as_mut() } {
                    if o.rect.within(c) {}
                }
            }
            _ => {}
        }
    }
}

#[derive(Getters, MutGetters)]
pub(crate) struct Area {
    #[getset(get = "pub(crate)")]
    element: Arc<RwLock<Element>>,
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    subset: DrawUnitHolder,
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    rect: FixedRect,
    painter: AppearanceComposite,
    align_pattern: AlignPattern,
    layout: LayoutCoord,
    scroll_bar: ScrollBar,
}

impl std::fmt::Debug for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Area");
        if let Ok(o) = self.element.try_read() {
            f.field("element", &o.to_string());
        }
        f.field("rect", &self.rect)
            .field("subset", &self.subset)
            .finish()
    }
}

impl Area {
    pub(crate) fn new(element: Arc<RwLock<Element>>) -> Self {
        let mut o = Self {
            element,
            subset: Default::default(),
            rect: (100.0, 100.0).into(),
            painter: Default::default(),
            align_pattern: Default::default(),
            layout: Default::default(),
            scroll_bar: Default::default(),
        };
        o.renew_subset();
        o
    }

    pub(crate) fn renew_subset(&mut self) {
        renew_subset(&self.element, &mut self.subset);
    }

    pub(crate) fn resize(&mut self, c: &mut LayoutCoord) {
        if let Ok(e) = self.element.read() {
            self.rect.get_attr(&e, c);
            self.layout.get_attr(&e, self.rect.clone());
        }
        self.subset.resize(&mut self.layout);
    }

    right_bottom!();

    pub(crate) fn draw(&mut self, t: &mut DrawCtx) {
        draw_check!(self);

        self.painter.draw(&self.rect, t);
        t.surface.canvas().save();

        subset_draw(&mut self.subset, &self.rect, &mut self.scroll_bar, t);
    }

    pub(crate) fn consume_action(&mut self, t: &mut ActionCtx) {
        match &t.kind {
            ActionKind::Click(c, _)
            | ActionKind::DoubleClick(c, _)
            | ActionKind::Cursor(c)
            | ActionKind::CursorWithoutFocus(c)
            | ActionKind::DeleteFront(c, _)
            | ActionKind::DeleteBack(c, _) => {
                if self.rect.within(c) {
                    if self.scroll_bar.within(c) {
                        t.finish = true;
                        return;
                    }
                    let (x, y) = self.scroll_bar.vision_var();
                    t.kind.set_var_cursor(x, y);
                    self.subset.consume_action(t);
                    t.kind.set_var_cursor(-x, -y);
                    return;
                }
            }
            ActionKind::Sweep(a, b) => {
                if self.scroll_bar.within(b) {
                    self.scroll_bar.move_a_to_b(a, b);
                    t.finish = true;
                    return;
                }
                if self.rect.within(a) || self.rect.within(b) {
                    let (x, y) = self.scroll_bar.vision_var();
                    t.kind.set_var_cursor(x, y);
                    self.subset.consume_action(t);
                    t.kind.set_var_cursor(-x, -y);
                    return;
                }
            }
            _ => {
                return self.subset.consume_action(t);
            }
        }
    }

    fn callback_action(&mut self) -> impl FnMut(&ActionKind, &mut PageContext) {
        let o = self as *mut Self;
        move |a, context| match &a {
            _ => {}
        }
    }
}

fn renew_subset(element: &Arc<RwLock<Element>>, subset: &mut DrawUnitHolder) {
    if let Ok(e) = element.read() {
        e.subset().iter().for_each(|i| {
            if let Some(draw_unit) = DrawUnit::new(i.clone()) {
                subset.push(draw_unit)
            } else {
                renew_subset(i, subset)
            }
        })
    }
}

fn subset_draw(
    subset: &mut DrawUnitHolder,
    rect: &FixedRect,
    scroll_bar: &mut ScrollBar,
    t: &mut DrawCtx,
) {
    if let Some(right_bottom) = subset.right_bottom() {
        let max = RectSide::away_from(&right_bottom, rect);
        let vision = scroll_bar.resize(rect, &max);
        let surface = &mut t.surface;
        let info = surface.image_info().with_dimensions(vision.right_bottom());
        if let Some(surface2) = surface.new_surface(&info) {
            let mut t2 = DrawCtx::new(surface2, t.context);
            subset.draw(&mut t2);

            if let Some(image2) = t2.surface.image_snapshot_with_bounds(vision.to_irect()) {
                surface.canvas().draw_image(image2, &***rect, None);
                scroll_bar.draw(t);
            }
        }
    }
}

//------------------------------------------------------------------------------------------

#[derive(Debug)]
enum DrawUnit {
    AREA(Area),
    AUDIO(Audio),
    BUTTON(Button),
    CANVAS(Canv),
    IFRAME(Iframe),
    IMG(Img),
    INP(Inp),
    PT(Pt),
    SELECT(Select),
    TIME(Time),
    VIDEO(Video),
}

impl DrawUnit {
    pub(crate) fn new(element: Arc<RwLock<Element>>) -> Option<Self> {
        let e = element.read().ok()?;
        let mark_type = e.mark_type().clone();
        drop(e);
        match mark_type {
            Mark::AREA => Some(Self::AREA(Area::new(element))),
            Mark::AUDIO => Some(Self::AUDIO(Audio::new(element))),
            Mark::BUTTON => Some(Self::BUTTON(Button::new(element))),
            Mark::CANVAS => Some(Self::CANVAS(Canv::new(element))),
            Mark::IFRAME => Some(Self::IFRAME(Iframe::new(element))),
            Mark::IMG => Some(Self::IMG(Img::new(element))),
            Mark::INP => Some(Self::INP(Inp::new(element))),
            Mark::PT => Some(Self::PT(Pt::new(element))),
            Mark::SELECT => Some(Self::SELECT(Select::new(element))),
            Mark::TIME => Some(Self::TIME(Time::new(element))),
            Mark::VIDEO => Some(Self::VIDEO(Video::new(element))),
            _ => None,
        }
    }

    pub(crate) fn resize(&mut self, c: &mut LayoutCoord) {
        match self {
            Self::AREA(o) => o.resize(c),
            Self::AUDIO(o) => o.resize(c),
            Self::BUTTON(o) => o.resize(c),
            Self::CANVAS(o) => o.resize(c),
            Self::IFRAME(o) => o.resize(c),
            Self::IMG(o) => o.resize(c),
            Self::INP(o) => o.resize(c),
            Self::PT(o) => o.resize(c),
            Self::SELECT(o) => o.resize(c),
            Self::TIME(o) => o.resize(c),
            Self::VIDEO(o) => o.resize(c),
        }
    }

    pub(crate) fn right_bottom(&self) -> Option<Coord2D> {
        match self {
            Self::AREA(o) => o.right_bottom(),
            Self::AUDIO(o) => o.right_bottom(),
            Self::BUTTON(o) => o.right_bottom(),
            Self::CANVAS(o) => o.right_bottom(),
            Self::IFRAME(o) => o.right_bottom(),
            Self::IMG(o) => o.right_bottom(),
            Self::INP(o) => o.right_bottom(),
            Self::PT(o) => o.right_bottom(),
            Self::SELECT(o) => o.right_bottom(),
            Self::TIME(o) => o.right_bottom(),
            Self::VIDEO(o) => o.right_bottom(),
        }
    }

    pub(crate) fn draw(&mut self, t: &mut DrawCtx) {
        match self {
            Self::AREA(o) => o.draw(t),
            Self::AUDIO(o) => o.draw(t),
            Self::BUTTON(o) => o.draw(t),
            Self::CANVAS(o) => o.draw(t),
            Self::IFRAME(o) => o.draw(t),
            Self::IMG(o) => o.draw(t),
            Self::INP(o) => o.draw(t),
            Self::PT(o) => o.draw(t),
            Self::SELECT(o) => o.draw(t),
            Self::TIME(o) => o.draw(t),
            Self::VIDEO(o) => o.draw(t),
        }
    }

    pub(crate) fn consume_action(&mut self, t: &mut ActionCtx) {
        match self {
            Self::AREA(o) => o.consume_action(t),
            Self::AUDIO(o) => o.consume_action(t),
            Self::BUTTON(o) => o.consume_action(t),
            Self::CANVAS(o) => o.consume_action(t),
            Self::IFRAME(o) => o.consume_action(t),
            Self::IMG(o) => o.consume_action(t),
            Self::INP(o) => o.consume_action(t),
            Self::PT(o) => o.consume_action(t),
            Self::SELECT(o) => o.consume_action(t),
            Self::TIME(o) => o.consume_action(t),
            Self::VIDEO(o) => o.consume_action(t),
        }
    }
}

#[derive(Default)]
struct DrawUnitHolder(Vec<DrawUnit>);

impl std::fmt::Debug for DrawUnitHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(&self.0).finish()
    }
}

impl DrawUnitHolder {
    pub(crate) fn resize(&mut self, c: &mut LayoutCoord) {
        for i in self.0.iter_mut() {
            i.resize(c);
        }
    }

    pub(crate) fn right_bottom(&self) -> Option<Coord2D> {
        let mut c: Option<Coord2D> = None;
        for i in self.0.iter() {
            if let Some(r) = i.right_bottom() {
                if let Some(c) = c.as_mut() {
                    if c.x() < r.x() {
                        c.set_x(r.x());
                    }
                    if c.y() < r.y() {
                        c.set_y(r.y());
                    }
                } else {
                    c.replace(r);
                }
            }
        }
        c
    }

    pub(crate) fn draw(&mut self, t: &mut DrawCtx) {
        for e in self.0.iter_mut() {
            e.draw(t);
        }
    }

    pub(crate) fn consume_action(&mut self, t: &mut ActionCtx) {
        let mut v = self.0.iter_mut();
        while let Some(i) = v.next_back() {
            i.consume_action(t);
            if t.finish {
                return;
            }
        }
    }
}

deref!(DrawUnitHolder, Vec<DrawUnit>, 0);

pub(crate) struct DrawCtx<'a> {
    surface: Surface,
    context: &'a mut PageContext,
    paint: Paint,
}

impl<'a> DrawCtx<'a> {
    pub(crate) fn new(surface: Surface, context: &'a mut PageContext) -> Self {
        let mut paint = Paint::default();
        paint.set_color(*default_bg_color());
        paint.set_anti_alias(true);
        Self {
            surface,
            context,
            paint,
        }
    }

    pub(crate) fn draw_in_rect(
        &mut self,
        rect: &FixedRect,
        mut f: impl FnMut(&mut Surface, &Paint),
    ) {
        let surface = &mut self.surface;
        let info = surface.image_info().with_dimensions(rect.side());
        if let Some(mut surface2) = surface.new_surface(&info) {
            f(&mut surface2, &self.paint);
            let image2 = surface2.image_snapshot();
            surface.canvas().draw_image(image2, &***rect, None);
        }
    }

    pub(crate) fn draw_in_vision(
        &mut self,
        vision: &FixedRect,
        rect: &FixedRect,
        mut f: impl FnMut(&mut Surface, &Paint),
    ) {
        let surface = &mut self.surface;
        let info = surface.image_info().with_dimensions(vision.right_bottom());
        if let Some(mut surface2) = surface.new_surface(&info) {
            f(&mut surface2, &self.paint);

            if let Some(image2) = surface2.image_snapshot_with_bounds(vision.to_irect()) {
                surface.canvas().draw_image(image2, &***rect, None);
            }
        }
    }
}

pub(crate) struct ActionCtx<'a> {
    kind: ActionKind,
    context: &'a mut PageContext,
    finish: bool,
    callback: &'a mut Vec<Box<dyn FnMut(&ActionKind, &mut PageContext)>>,
}

impl<'a> ActionCtx<'a> {
    pub(crate) fn new(
        kind: ActionKind,
        context: &'a mut PageContext,
        callback: &'a mut Vec<Box<dyn FnMut(&ActionKind, &mut PageContext)>>,
    ) -> Self {
        Self {
            kind,
            context,
            finish: false,
            callback,
        }
    }
}
