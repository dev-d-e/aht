use super::*;
use std::sync::{Arc, RwLock};

///"Button" represents a button.
#[derive(Getters, MutGetters)]
pub(crate) struct Button {
    #[getset(get = "pub(crate)")]
    element: Arc<RwLock<Element>>,
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    rect: FixedRect,
    painter: AppearanceComposite,
    draw_text: DrawText,
    time_meter: Chronograph,
    f: bool,
}

impl std::fmt::Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Button");
        if let Ok(o) = self.element.try_read() {
            f.field("element", &o.to_string());
        }
        f.field("rect", &self.rect)
            .field("painter", &self.painter)
            .finish()
    }
}

impl Button {
    pub(crate) fn new(element: Arc<RwLock<Element>>) -> Self {
        Self {
            element,
            rect: FixedRect::with_side(100.0, 60.0),
            painter: RoundRectangle {
                color: *default_button_color(),
                ..Default::default()
            }
            .into(),
            draw_text: DrawText::new(AlignPattern::center_middle()),
            time_meter: Chronograph::new(1000),
            f: false,
        }
    }

    resize!();

    right_bottom!();

    pub(crate) fn draw(&mut self, t: &mut DrawCtx) {
        draw_check!(self);

        if self.f && self.time_meter.elapsed() {
            self.painter = RoundRectangle {
                color: *default_button_color(),
                ..Default::default()
            }
            .into();
        }

        self.painter.draw(&self.rect, t);

        if let Ok(e) = self.element.read() {
            self.draw_text.draw(&self.rect, e.text(), t);
        }
    }

    pub(crate) fn consume_action(&mut self, t: &mut ActionCtx) {
        match &t.kind {
            ActionKind::Click(c, _) => {
                if self.painter.within(&self.rect, c) {
                    self.f = true;
                    self.painter = RoundRectangle {
                        color: *default_button2_color(),
                        ..Default::default()
                    }
                    .into();
                    self.time_meter.refresh();
                    t.finish = true;
                    return;
                }
            }
            _ => {}
        }
    }

    fn callback_action(&mut self) -> impl FnMut(&ActionKind, &mut PageContext) {
        let o = self as *mut Self;
        move |a, context| match &a {
            _ => {}
        }
    }
}

///"Inp" represents input.
#[derive(Getters, MutGetters)]
pub(crate) struct Inp {
    #[getset(get = "pub(crate)")]
    element: Arc<RwLock<Element>>,
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    rect: FixedRect,
    painter: AppearanceComposite,
    draw_text: DrawText,
}

impl std::fmt::Debug for Inp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Inp");
        if let Ok(o) = self.element.try_read() {
            f.field("element", &o.to_string());
        }
        f.field("rect", &self.rect)
            .field("painter", &self.painter)
            .finish()
    }
}

impl Inp {
    pub(crate) fn new(element: Arc<RwLock<Element>>) -> Self {
        Self {
            element,
            rect: FixedRect::with_side(100.0, 30.0),
            painter: RectangleCurve {
                color: *default_border_color(),
                ..Default::default()
            }
            .into(),
            draw_text: Default::default(),
        }
    }

    resize!();

    right_bottom!();

    pub(crate) fn draw(&mut self, t: &mut DrawCtx) {
        draw_check!(self);

        self.painter.draw(&self.rect, t);

        if let Ok(e) = self.element.read() {
            self.draw_text.draw(&self.rect, e.get_value_or_text(), t);
        }
    }

    pub(crate) fn consume_action(&mut self, t: &mut ActionCtx) {
        match &t.kind {
            ActionKind::Click(c, _) | ActionKind::DoubleClick(c, _) => {
                if self.painter.within(&self.rect, c) {
                    t.context.set_input_to(self.element.clone());
                    self.draw_text.set_cursor(true);
                    t.finish = true;
                    t.callback.push(Box::new(self.callback_action()));
                    return;
                } else {
                    t.context.take_input_to();
                    self.draw_text.set_cursor(false);
                }
            }
            ActionKind::Focused(o) => {
                if *o {
                } else {
                    self.draw_text.set_cursor(false);
                }
            }
            _ => {}
        }
    }

    fn callback_action(&mut self) -> impl FnMut(&ActionKind, &mut PageContext) {
        let o = self as *mut Self;
        move |a, context| match &a {
            _ => {}
        }
    }
}

///"Opt" represents an option.
#[derive(Getters, MutGetters)]
pub(crate) struct Opt {
    #[getset(get = "pub(crate)")]
    element: Arc<RwLock<Element>>,
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    rect: FixedRect,
    painter: AppearanceComposite,
}

impl std::fmt::Debug for Opt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Opt");
        if let Ok(o) = self.element.try_read() {
            f.field("element", &o.to_string());
        }
        f.field("rect", &self.rect)
            .field("painter", &self.painter)
            .finish()
    }
}

impl Opt {
    pub(crate) fn new(element: Arc<RwLock<Element>>) -> Self {
        Self {
            element,
            rect: FixedRect::with_side(100.0, 100.0),
            painter: Default::default(),
        }
    }

    pub(crate) fn draw(&mut self, t: &mut DrawCtx) {
        self.painter.draw(&self.rect, t);
    }
}

///"Pt" represents plain text.
#[derive(Getters, MutGetters)]
pub(crate) struct Pt {
    #[getset(get = "pub(crate)")]
    element: Arc<RwLock<Element>>,
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    rect: FixedRect,
    painter: AppearanceComposite,
    draw_text: DrawText,
    scroll_bar: ScrollBar,
}

impl std::fmt::Debug for Pt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Pt");
        if let Ok(o) = self.element.try_read() {
            f.field("element", &o.to_string());
        }
        f.field("rect", &self.rect)
            .field("painter", &self.painter)
            .finish()
    }
}

impl Pt {
    pub(crate) fn new(element: Arc<RwLock<Element>>) -> Self {
        Self {
            element,
            rect: FixedRect::with_side(100.0, 100.0),
            painter: RectangleCurve {
                color: *default_border_color(),
                ..Default::default()
            }
            .into(),
            draw_text: Default::default(),
            scroll_bar: Default::default(),
        }
    }

    resize!();

    right_bottom!();

    pub(crate) fn draw(&mut self, t: &mut DrawCtx) {
        draw_check!(self);

        self.painter.draw(&self.rect, t);

        if let Ok(e) = self.element.read() {
            self.draw_text.draw(&self.rect, e.text(), t);
        }
    }

    pub(crate) fn consume_action(&mut self, t: &mut ActionCtx) {
        match &t.kind {
            _ => {}
        }
    }

    fn callback_action(&mut self) -> impl FnMut(&ActionKind, &mut PageContext) {
        let o = self as *mut Self;
        move |a, context| match &a {
            _ => {}
        }
    }
}

///"Select" represents a select.
#[derive(Getters, MutGetters)]
pub(crate) struct Select {
    #[getset(get = "pub(crate)")]
    element: Arc<RwLock<Element>>,
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    rect: FixedRect,
    painter: AppearanceComposite,
    draw_text: DrawText,
    scroll_bar: ScrollBar,
    ops: Vec<Opt>,
}

impl std::fmt::Debug for Select {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Select");
        if let Ok(o) = self.element.try_read() {
            f.field("element", &o.to_string());
        }
        f.field("rect", &self.rect)
            .field("painter", &self.painter)
            .finish()
    }
}

impl Select {
    pub(crate) fn new(element: Arc<RwLock<Element>>) -> Self {
        Self {
            element,
            rect: FixedRect::with_side(100.0, 100.0),
            painter: RectangleCurve {
                color: *default_border_color(),
                ..Default::default()
            }
            .into(),
            draw_text: Default::default(),
            scroll_bar: Default::default(),
            ops: Default::default(),
        }
    }

    resize!();

    right_bottom!();

    pub(crate) fn draw(&mut self, t: &mut DrawCtx) {
        draw_check!(self);

        self.painter.draw(&self.rect, t);

        if let Ok(e) = self.element.read() {
            self.draw_text.draw(&self.rect, e.text(), t);
        }
    }

    pub(crate) fn consume_action(&mut self, t: &mut ActionCtx) {
        match t.kind {
            _ => {}
        }
    }

    fn callback_action(&mut self) -> impl FnMut(&ActionKind, &mut PageContext) {
        let o = self as *mut Self;
        move |a, context| match &a {
            _ => {}
        }
    }
}

///"Time" represents date time.
#[derive(Getters, MutGetters)]
pub(crate) struct Time {
    #[getset(get = "pub(crate)")]
    element: Arc<RwLock<Element>>,
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    rect: FixedRect,
    painter: AppearanceComposite,
    draw_text: DrawText,
}

impl std::fmt::Debug for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Time");
        if let Ok(o) = self.element.try_read() {
            f.field("element", &o.to_string());
        }
        f.field("rect", &self.rect)
            .field("painter", &self.painter)
            .finish()
    }
}

impl Time {
    pub(crate) fn new(element: Arc<RwLock<Element>>) -> Self {
        Self {
            element,
            rect: FixedRect::with_side(100.0, 100.0),
            painter: Rectangle {
                color: *default_surface_color(),
                ..Default::default()
            }
            .into(),
            draw_text: Default::default(),
        }
    }

    resize!();

    right_bottom!();

    pub(crate) fn draw(&mut self, t: &mut DrawCtx) {
        draw_check!(self);

        self.painter.draw(&self.rect, t);

        if let Ok(e) = self.element.read() {
            self.draw_text.draw(&self.rect, e.text(), t);
        }
    }

    pub(crate) fn consume_action(&mut self, t: &mut ActionCtx) {
        match t.kind {
            _ => {}
        }
    }

    fn callback_action(&mut self) -> impl FnMut(&ActionKind, &mut PageContext) {
        let o = self as *mut Self;
        move |a, context| match &a {
            _ => {}
        }
    }
}
