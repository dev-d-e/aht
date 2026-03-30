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
use skia_safe::{Paint, Surface};
use slotmap::{SlotMap, new_key_type};

pub(crate) struct Body {
    subset: Vec<DrawUnitKey>,
    rect: FixedRect,
    painter: AppearanceComposite,
    align_pattern: AlignPattern,
    layout: LayoutCoord,
    scroll_bar: ScrollBar,
    dh: DrawUnitHolder,
}

impl std::fmt::Debug for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Body {{ ")?;
        write!(f, "subset: {:?}, ", self.subset)?;
        writeln!(f, "dh: [")?;
        for o in &self.dh.data {
            writeln!(f, "{:?}", o)?;
        }
        write!(f, "] ")?;
        write!(f, "rect: {:?} }}", self.rect)
    }
}

impl Body {
    pub(crate) fn new(cx: &mut PageContext) -> Self {
        let mut o = Self {
            subset: Default::default(),
            rect: (100.0, 100.0).into(),
            painter: Rectangle::default().into(),
            align_pattern: Default::default(),
            layout: Default::default(),
            scroll_bar: Default::default(),
            dh: Default::default(),
        };
        o.build_subset(&cx, cx.body_key());
        o
    }

    pub(crate) fn build_subset(&mut self, eh: &ElementHolder, key: ElementKey) {
        if let Some(e) = eh.get(key) {
            for &k in e.subset() {
                if let Some(e) = eh.get(k) {
                    if let Some(o) = DrawUnit::new(k, e.mark_type(), eh) {
                        let dk = self.dh.insert(o);
                        self.subset.push(dk);
                        match self.dh[dk] {
                            DrawUnit::AREA(_) => {
                                self.dh.build(eh, k, dk);
                            }
                            _ => {}
                        }
                    } else {
                        self.build_subset(eh, k);
                    }
                }
            }
        }
    }

    pub(crate) fn resize(&mut self, w: f32, h: f32, cx: &mut PageContext) {
        let k = RectSide::new(w, h);
        self.rect.set_side(k.clone());
        if let Some(e) = cx.body_element() {
            self.rect.side_mut().get_attr(e, &k);
            self.layout.get_attr(e, self.rect.clone());
        }
        self.dh.resize(&mut self.layout, &self.subset, cx);
    }

    pub(crate) fn reset(&mut self, x: f32, y: f32, w: f32, h: f32, cx: &mut PageContext) {
        self.rect.set_x(x);
        self.rect.set_y(y);
        self.resize(w, h, cx);
    }

    pub(crate) fn draw(&mut self, mut dcx: DrawCtx, cx: &mut PageContext) {
        if self.rect.is_empty() {
            return;
        }

        dcx.surface.canvas().clear(*default_bg_color());

        self.painter.draw(&self.rect, &mut dcx);
        dcx.surface.canvas().save();

        self.dh
            .draw_rect(&self.rect, &mut self.scroll_bar, &self.subset, &mut dcx, cx);
        dcx.surface.canvas().save();
    }

    pub(crate) fn consume_action(&mut self, mut acx: ActionCtx, cx: &mut PageContext) {
        match &acx.kind {
            ActionKind::Click(c, _)
            | ActionKind::DoubleClick(c, _)
            | ActionKind::Cursor(c, _)
            | ActionKind::CursorWithoutFocus(c, _) => {
                if self.scroll_bar.within(c) {
                    return;
                }
            }
            ActionKind::Pressed(c, _) => {
                if self.scroll_bar.within(c) {
                    self.scroll_bar.set_mp(c.clone());
                    return;
                }
            }
            ActionKind::Released(_) => {
                self.scroll_bar.clear_mp();
            }
            ActionKind::Sweep(b, a, d) => {
                if self.scroll_bar.within(b) {
                    self.scroll_bar.move_to(b, d.0, d.1);
                    return;
                }
                if let Some(c) = self.scroll_bar.get_mp() {
                    self.scroll_bar.move_to(&c, d.0, d.1);
                }
            }
            _ => {}
        }
        let (x, y) = self.scroll_bar.vision_var();
        acx.kind.set_var_cursor(x, y);
        self.dh.consume_action(&self.subset, &mut acx, cx);
    }
}

#[derive(Default)]
struct DrawUnitHolder {
    data: SlotMap<DrawUnitKey, DrawUnit>,
}

deref!(DrawUnitHolder, SlotMap<DrawUnitKey, DrawUnit>, data);

impl DrawUnitHolder {
    fn build(&mut self, eh: &ElementHolder, key: ElementKey, dkey: DrawUnitKey) {
        if let Some(e) = eh.get(key) {
            for &k in e.subset() {
                if let Some(h) = eh.get(k) {
                    if let Some(o) = DrawUnit::new(k, h.mark_type(), eh) {
                        let dk = self.data.insert(o);
                        match &mut self.data[dkey] {
                            DrawUnit::AREA(o) => {
                                o.subset.push(dk);
                                self.build(eh, k, dk);
                            }
                            _ => {}
                        }
                    } else {
                        self.build(eh, k, dkey);
                    }
                }
            }
        }
    }

    fn resize(&mut self, c: &mut LayoutCoord, ks: &[DrawUnitKey], cx: &mut PageContext) {
        let mut r = None;
        for &k in ks {
            if let Some(o) = self.data.get_mut(k) {
                o.resize(c, cx);
                match o {
                    DrawUnit::AREA(o) => {
                        r.replace(&mut *o as *mut Area);
                    }
                    _ => {}
                }
            }
            if let Some(o) = r.take() {
                let o = unsafe { &mut *o };
                self.resize(&mut o.layout, &o.subset, cx);
            }
        }
    }

    fn draw_rect(
        &mut self,
        rect: &FixedRect,
        scroll_bar: &mut ScrollBar,
        ks: &[DrawUnitKey],
        dcx: &mut DrawCtx,
        cx: &mut PageContext,
    ) {
        if let Some(right_bottom) = self.right_bottom(ks, cx) {
            let max = RectSide::away_from(&right_bottom, rect);
            let vision = scroll_bar.resize(rect, &max);
            let surface = &mut dcx.surface;
            let info = surface.image_info().with_dimensions(vision.right_bottom());
            if let Some(s) = surface.new_surface(&info) {
                let mut d = DrawCtx::new(s);
                d.surface.canvas().clip_rect(vision.to_rect(), None, None);
                self.draw_subset(ks, &mut d, cx);

                if let Some(i) = d.surface.image_snapshot_with_bounds(vision.to_irect()) {
                    surface.canvas().draw_image(i, &***rect, None);
                    scroll_bar.draw(dcx);
                }
            }
        }
    }

    fn right_bottom(&mut self, ks: &[DrawUnitKey], cx: &mut PageContext) -> Option<Coord2D> {
        let mut c: Option<Coord2D> = None;
        for &k in ks {
            if let Some(r) = self.data.get(k).and_then(|o| o.right_bottom(cx)) {
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

    fn draw_subset(&mut self, ks: &[DrawUnitKey], dcx: &mut DrawCtx, cx: &mut PageContext) {
        let mut r = None;
        for &k in ks {
            if let Some(o) = self.get_mut(k) {
                o.draw(dcx, cx);
                match o {
                    DrawUnit::AREA(o) => {
                        r.replace(&mut *o as *mut Area);
                    }
                    _ => {}
                }
            }
            if let Some(o) = r.take() {
                let o = unsafe { &mut *o };
                self.draw_rect(&o.rect, &mut o.scroll_bar, &o.subset, dcx, cx);
            }
        }
    }

    fn consume_action(&mut self, ks: &[DrawUnitKey], acx: &mut ActionCtx, cx: &mut PageContext) {
        let mut v = ks.iter();
        while let Some(&k) = v.next_back() {
            let o = if let Some(o) = self.data.get_mut(k) {
                o as *mut DrawUnit
            } else {
                continue;
            };
            acx.remove(k);
            unsafe { &mut *o }.consume_action(self, acx, cx);
            if acx.is_finished() {
                return;
            }
        }
    }
}

new_key_type! { pub struct DrawUnitKey; }

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

impl std::fmt::Debug for DrawUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DrawUnit {{ mark_type: ")?;
        match self {
            Self::AREA(o) => write!(f, "{:?}, subset: {:?}", Mark::AREA, o.subset)?,
            Self::AUDIO(_) => write!(f, "{:?}", Mark::AUDIO)?,
            Self::BUTTON(_) => write!(f, "{:?}", Mark::BUTTON)?,
            Self::CANVAS(_) => write!(f, "{:?}", Mark::CANVAS)?,
            Self::IFRAME(_) => write!(f, "{:?}", Mark::IFRAME)?,
            Self::IMG(_) => write!(f, "{:?}", Mark::IMG)?,
            Self::INP(_) => write!(f, "{:?}", Mark::INP)?,
            Self::PT(_) => write!(f, "{:?}", Mark::PT)?,
            Self::SELECT(_) => write!(f, "{:?}", Mark::SELECT)?,
            Self::TIME(_) => write!(f, "{:?}", Mark::TIME)?,
            Self::VIDEO(_) => write!(f, "{:?}", Mark::VIDEO)?,
        }
        write!(f, " }}")
    }
}

impl DrawUnit {
    pub(crate) fn new(key: ElementKey, mark_type: &Mark, eh: &ElementHolder) -> Option<Self> {
        match mark_type {
            Mark::AREA => Some(Self::AREA(Area::new(key, eh))),
            Mark::AUDIO => Some(Self::AUDIO(Audio::new(key, eh))),
            Mark::BUTTON => Some(Self::BUTTON(Button::new(key, eh))),
            Mark::CANVAS => Some(Self::CANVAS(Canv::new(key, eh))),
            Mark::IFRAME => Some(Self::IFRAME(Iframe::new(key, eh))),
            Mark::IMG => Some(Self::IMG(Img::new(key, eh))),
            Mark::INP => Some(Self::INP(Inp::new(key, eh))),
            Mark::PT => Some(Self::PT(Pt::new(key, eh))),
            Mark::SELECT => Some(Self::SELECT(Select::new(key, eh))),
            Mark::TIME => Some(Self::TIME(Time::new(key, eh))),
            Mark::VIDEO => Some(Self::VIDEO(Video::new(key, eh))),
            _ => None,
        }
    }

    pub(crate) fn resize(&mut self, c: &mut LayoutCoord, cx: &mut PageContext) {
        match self {
            Self::AREA(o) => o.resize(c, cx),
            Self::AUDIO(o) => o.resize(c, cx),
            Self::BUTTON(o) => o.resize(c, cx),
            Self::CANVAS(o) => o.resize(c, cx),
            Self::IFRAME(o) => o.resize(c, cx),
            Self::IMG(o) => o.resize(c, cx),
            Self::INP(o) => o.resize(c, cx),
            Self::PT(o) => o.resize(c, cx),
            Self::SELECT(o) => o.resize(c, cx),
            Self::TIME(o) => o.resize(c, cx),
            Self::VIDEO(o) => o.resize(c, cx),
        }
    }

    pub(crate) fn right_bottom(&self, cx: &mut PageContext) -> Option<Coord2D> {
        match self {
            Self::AREA(o) => o.right_bottom(cx),
            Self::AUDIO(o) => o.right_bottom(cx),
            Self::BUTTON(o) => o.right_bottom(cx),
            Self::CANVAS(o) => o.right_bottom(cx),
            Self::IFRAME(o) => o.right_bottom(cx),
            Self::IMG(o) => o.right_bottom(cx),
            Self::INP(o) => o.right_bottom(cx),
            Self::PT(o) => o.right_bottom(cx),
            Self::SELECT(o) => o.right_bottom(cx),
            Self::TIME(o) => o.right_bottom(cx),
            Self::VIDEO(o) => o.right_bottom(cx),
        }
    }

    pub(crate) fn draw(&mut self, dcx: &mut DrawCtx, cx: &mut PageContext) {
        match self {
            Self::AREA(o) => o.draw(dcx, cx),
            Self::AUDIO(o) => o.draw(dcx, cx),
            Self::BUTTON(o) => o.draw(dcx, cx),
            Self::CANVAS(o) => o.draw(dcx, cx),
            Self::IFRAME(o) => o.draw(dcx, cx),
            Self::IMG(o) => o.draw(dcx, cx),
            Self::INP(o) => o.draw(dcx, cx),
            Self::PT(o) => o.draw(dcx, cx),
            Self::SELECT(o) => o.draw(dcx, cx),
            Self::TIME(o) => o.draw(dcx, cx),
            Self::VIDEO(o) => o.draw(dcx, cx),
        }
    }

    pub(crate) fn consume_action(
        &mut self,
        dh: &mut DrawUnitHolder,
        acx: &mut ActionCtx,
        cx: &mut PageContext,
    ) {
        match self {
            Self::AREA(o) => o.consume_action(dh, acx, cx),
            Self::AUDIO(o) => o.consume_action(acx, cx),
            Self::BUTTON(o) => o.consume_action(acx, cx),
            Self::CANVAS(o) => o.consume_action(acx, cx),
            Self::IFRAME(o) => o.consume_action(acx, cx),
            Self::IMG(o) => o.consume_action(acx, cx),
            Self::INP(o) => o.consume_action(acx, cx),
            Self::PT(o) => o.consume_action(acx, cx),
            Self::SELECT(o) => o.consume_action(acx, cx),
            Self::TIME(o) => o.consume_action(acx, cx),
            Self::VIDEO(o) => o.consume_action(acx, cx),
        }
    }
}

#[derive(Debug)]
struct Area {
    element: ElementKey,
    subset: Vec<DrawUnitKey>,
    rect: FixedRect,
    painter: AppearanceComposite,
    align_pattern: AlignPattern,
    layout: LayoutCoord,
    scroll_bar: ScrollBar,
}

impl Area {
    pub(crate) fn new(element: ElementKey, eh: &ElementHolder) -> Self {
        Self {
            element,
            subset: Default::default(),
            rect: (100.0, 100.0).into(),
            painter: Default::default(),
            align_pattern: Default::default(),
            layout: Default::default(),
            scroll_bar: Default::default(),
        }
    }

    pub(crate) fn resize(&mut self, c: &mut LayoutCoord, cx: &mut PageContext) {
        if let Some(e) = cx.get(self.element) {
            self.rect.get_attr(&e, c);
            self.layout.get_attr(&e, self.rect.clone());
        }
    }

    right_bottom!();

    pub(crate) fn draw(&mut self, dcx: &mut DrawCtx, cx: &mut PageContext) {
        draw_check!(self, cx);

        self.painter.draw(&self.rect, dcx);
        dcx.surface.canvas().save();
    }

    fn consume_action(
        &mut self,
        dh: &mut DrawUnitHolder,
        acx: &mut ActionCtx,
        cx: &mut PageContext,
    ) {
        match &acx.kind {
            ActionKind::Click(c, _)
            | ActionKind::DoubleClick(c, _)
            | ActionKind::Cursor(c, _)
            | ActionKind::CursorWithoutFocus(c, _) => {
                if self.rect.within(c) {
                    acx.finish = true;
                    if self.scroll_bar.within(c) {
                    } else {
                        let (x, y) = self.scroll_bar.vision_var();
                        acx.kind.set_var_cursor(x, y);
                        dh.consume_action(&self.subset, acx, cx);
                        acx.kind.set_var_cursor(-x, -y);
                    }
                    return;
                }
            }
            ActionKind::Pressed(c, _) => {
                if self.rect.within(c) {
                    acx.finish = true;
                    if self.scroll_bar.within(c) {
                        self.scroll_bar.set_mp(c.clone());
                    } else {
                        let (x, y) = self.scroll_bar.vision_var();
                        acx.kind.set_var_cursor(x, y);
                        dh.consume_action(&self.subset, acx, cx);
                        acx.kind.set_var_cursor(-x, -y);
                    }
                    return;
                }
            }
            ActionKind::Released(_) => {
                self.scroll_bar.clear_mp();
            }
            ActionKind::Sweep(b, a, d) => {
                if self.rect.within(b) {
                    if self.scroll_bar.within(b) {
                        self.scroll_bar.move_to(b, d.0, d.1);
                        acx.finish = true;
                        return;
                    }
                    if let Some(c) = self.scroll_bar.get_mp() {
                        self.scroll_bar.move_to(&c, d.0, d.1);
                    }
                    let (x, y) = self.scroll_bar.vision_var();
                    acx.kind.set_var_cursor(x, y);
                    dh.consume_action(&self.subset, acx, cx);
                    acx.kind.set_var_cursor(-x, -y);
                    return;
                }
            }
            _ => {
                dh.consume_action(&self.subset, acx, cx);
            }
        }
    }
}

//------------------------------------------------------------------------------------------

pub(crate) struct DrawCtx {
    surface: Surface,
    paint: Paint,
}

impl DrawCtx {
    pub(crate) fn new(surface: Surface) -> Self {
        let mut paint = Paint::default();
        paint.set_color(*default_bg_color());
        paint.set_anti_alias(true);
        Self { surface, paint }
    }

    fn draw_in_rect(&mut self, rect: &FixedRect, mut f: impl FnMut(&mut Surface, &Paint)) {
        let surface = &mut self.surface;
        let info = surface.image_info().with_dimensions(rect.side());
        if let Some(mut surface2) = surface.new_surface(&info) {
            f(&mut surface2, &self.paint);
            let image2 = surface2.image_snapshot();
            surface.canvas().draw_image(image2, &***rect, None);
        }
    }

    fn draw_in_vision(
        &mut self,
        vision: &FixedRect,
        rect: &FixedRect,
        mut f: impl FnMut(&mut Surface, &Paint),
    ) {
        let surface = &mut self.surface;
        let info = surface.image_info().with_dimensions(vision.right_bottom());
        if let Some(mut surface2) = surface.new_surface(&info) {
            surface2.canvas().clip_rect(vision.to_rect(), None, None);
            f(&mut surface2, &self.paint);

            if let Some(image2) = surface2.image_snapshot_with_bounds(vision.to_irect()) {
                surface.canvas().draw_image(image2, &***rect, None);
            }
        }
    }
}

pub(crate) struct ActionCtx<'a> {
    kind: ActionKind,
    finish: bool,
    callback: &'a mut Vec<DrawUnitKey>,
    new_callback: Vec<DrawUnitKey>,
    current: DrawUnitKey,
}

impl<'a> Drop for ActionCtx<'a> {
    fn drop(&mut self) {
        self.callback.clear();
        self.callback.append(&mut self.new_callback);
    }
}

impl<'a> ActionCtx<'a> {
    pub(crate) fn new(kind: ActionKind, callback: &'a mut Vec<DrawUnitKey>) -> Self {
        Self {
            kind,
            finish: false,
            callback,
            new_callback: Default::default(),
            current: Default::default(),
        }
    }

    fn is_finished(&self) -> bool {
        self.finish && self.callback.is_empty()
    }

    fn remove(&mut self, k: DrawUnitKey) {
        self.current = k;
        if let Some(i) = self.callback.iter().position(|o| o == &self.current) {
            self.callback.remove(i);
        }
    }

    fn push_callback(&mut self) {
        self.new_callback.push(self.current);
    }
}
