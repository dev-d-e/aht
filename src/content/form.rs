use super::*;

///"Button" represents a button.
#[derive(Debug)]
pub(crate) struct Button {
    element: ElementKey,
    rect: FixedRect,
    painter: AppearanceComposite,
    draw_text: DrawText,
    time_meter: Chronograph,
    f: bool,
}

impl Button {
    pub(crate) fn new(element: ElementKey, eh: &ElementHolder) -> Self {
        Self {
            element,
            rect: (100.0, 60.0).into(),
            painter: RoundRectangle {
                color: *default_button_color(),
                ..Default::default()
            }
            .into(),
            draw_text: AlignPattern::center_middle().into(),
            time_meter: Chronograph::new(1000),
            f: false,
        }
    }

    resize!();

    right_bottom!();

    pub(crate) fn draw(&mut self, dcx: &mut DrawCtx, cx: &mut PageContext) {
        draw_check!(self, cx);

        if self.f && self.time_meter.elapsed() {
            self.painter = RoundRectangle {
                color: *default_button_color(),
                ..Default::default()
            }
            .into();
            self.f = false;
        }

        self.painter.draw(&self.rect, dcx);

        if let Some(s) = cx.text(self.element) {
            self.draw_text.draw(&self.rect, s, dcx);
        }
    }

    pub(crate) fn consume_action(&mut self, acx: &mut ActionCtx, cx: &mut PageContext) {
        match &acx.kind {
            ActionKind::Click(c, _) | ActionKind::Pressed(c, _) => {
                if self.painter.within(&self.rect, c) {
                    self.f = true;
                    self.painter = RoundRectangle {
                        color: *default_button2_color(),
                        ..Default::default()
                    }
                    .into();
                    self.time_meter.refresh();
                    acx.finish = true;
                    return;
                }
            }
            ActionKind::Released(_) => {
                if self.f {
                    self.painter = RoundRectangle {
                        color: *default_button_color(),
                        ..Default::default()
                    }
                    .into();
                    self.f = false;
                }
            }
            _ => {}
        }
    }
}

///"Inp" represents input.
#[derive(Debug)]
pub(crate) struct Inp {
    element: ElementKey,
    rect: FixedRect,
    painter: AppearanceComposite,
    draw_text: DrawText,
    ops: Vec<Opt>,
    f: bool,
}

impl Inp {
    pub(crate) fn new(element: ElementKey, eh: &ElementHolder) -> Self {
        let v = eh.subset_with_mark(element, Mark::OPTION);
        let ops = v.into_iter().map(|o| Opt::new(o)).collect();
        Self {
            element,
            rect: (100.0, 30.0).into(),
            painter: RectangleCurve {
                color: *default_border_color(),
                ..Default::default()
            }
            .into(),
            draw_text: Default::default(),
            ops,
            f: false,
        }
    }

    resize!();

    right_bottom!();

    pub(crate) fn draw(&mut self, dcx: &mut DrawCtx, cx: &mut PageContext) {
        draw_check!(self, cx);

        self.painter.draw(&self.rect, dcx);

        if let Some(e) = cx.get(self.element) {
            self.draw_text.draw(&self.rect, e.get_value_or_text(), dcx);
        }
    }

    pub(crate) fn consume_action(&mut self, acx: &mut ActionCtx, cx: &mut PageContext) {
        match &acx.kind {
            ActionKind::Click(c, _) | ActionKind::DoubleClick(c, _) | ActionKind::Pressed(c, _) => {
                if self.painter.within(&self.rect, c) {
                    self.f = true;
                    self.draw_text.set_cursor(true);
                    acx.finish = true;
                    return;
                } else {
                    self.f = false;
                    self.draw_text.set_cursor(false);
                }
            }
            ActionKind::InputStr(s) => {
                if self.f {
                    if s.len() > 0 {
                        if let Some(a) = cx.get_mut(self.element).and_then(|e| e.value_or_insert())
                        {
                            a.push_str(&s);
                        }
                    }
                    acx.finish = true;
                    return;
                }
            }
            ActionKind::DeleteFront(n) => {
                if self.f {
                    acx.finish = true;
                    return;
                }
            }
            ActionKind::DeleteBack(n) => {
                if self.f {
                    acx.finish = true;
                    return;
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
}

///"Opt" represents an option.
#[derive(Debug)]
pub(crate) struct Opt {
    element: ElementKey,
    rect: FixedRect,
    painter: AppearanceComposite,
}

impl Opt {
    pub(crate) fn new(element: ElementKey) -> Self {
        Self {
            element,
            rect: (100.0, 100.0).into(),
            painter: Default::default(),
        }
    }

    pub(crate) fn draw(&mut self, dcx: &mut DrawCtx) {
        self.painter.draw(&self.rect, dcx);
    }
}

///"Pt" represents plain text.
#[derive(Debug)]
pub(crate) struct Pt {
    element: ElementKey,
    rect: FixedRect,
    painter: AppearanceComposite,
    draw_text: DrawText,
    scroll_bar: ScrollBar,
}

impl Pt {
    pub(crate) fn new(element: ElementKey, eh: &ElementHolder) -> Self {
        Self {
            element,
            rect: (100.0, 100.0).into(),
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

    pub(crate) fn draw(&mut self, dcx: &mut DrawCtx, cx: &mut PageContext) {
        draw_check!(self, cx);

        self.painter.draw(&self.rect, dcx);

        if let Some(s) = cx.text(self.element) {
            dcx.paint.set_color(*default_font_color());
            let dt = &self.draw_text;
            let size = dt.apply_font().text_size(s, &dcx.paint);
            let h = self.rect.side().height().max(size.height());
            let max = (size.width(), h).into();
            let vision = self.scroll_bar.resize(&self.rect, &max);
            let (c, a) = dt.align_pattern().font_xy(&self.rect, size.height());
            let font = dt.apply_font().font();
            dcx.draw_in_vision(&vision, &self.rect, |surface2, paint| {
                surface2.canvas().draw_str_align(s, &c, font, paint, a);
            });
            self.scroll_bar.draw(dcx);
        }
    }

    pub(crate) fn consume_action(&mut self, acx: &mut ActionCtx, cx: &mut PageContext) {
        match &acx.kind {
            ActionKind::Click(c, _) | ActionKind::DoubleClick(c, _) | ActionKind::Pressed(c, _) => {
                if self.painter.within(&self.rect, c) {
                    self.draw_text.set_cursor(true);
                    acx.finish = true;
                    return;
                } else {
                    self.draw_text.set_cursor(false);
                }
            }
            _ => {}
        }
    }
}

///"Select" represents a select.
#[derive(Debug)]
pub(crate) struct Select {
    element: ElementKey,
    rect: FixedRect,
    painter: AppearanceComposite,
    draw_text: DrawText,
    scroll_bar: ScrollBar,
    ops: Vec<Opt>,
}

impl Select {
    pub(crate) fn new(element: ElementKey, eh: &ElementHolder) -> Self {
        let v = eh.subset_with_mark(element, Mark::OPTION);
        let ops = v.into_iter().map(|o| Opt::new(o)).collect();
        Self {
            element,
            rect: (100.0, 100.0).into(),
            painter: RectangleCurve {
                color: *default_border_color(),
                ..Default::default()
            }
            .into(),
            draw_text: Default::default(),
            scroll_bar: Default::default(),
            ops,
        }
    }

    resize!();

    right_bottom!();

    pub(crate) fn draw(&mut self, dcx: &mut DrawCtx, cx: &mut PageContext) {
        draw_check!(self, cx);

        self.painter.draw(&self.rect, dcx);

        if let Some(s) = cx.text(self.element) {
            self.draw_text.draw(&self.rect, s, dcx);
        }
    }

    pub(crate) fn consume_action(&mut self, acx: &mut ActionCtx, cx: &mut PageContext) {
        match &acx.kind {
            ActionKind::Click(c, _) | ActionKind::DoubleClick(c, _) | ActionKind::Pressed(c, _) => {
                if self.painter.within(&self.rect, c) {
                    self.draw_text.set_cursor(true);
                    acx.finish = true;
                    return;
                } else {
                    self.draw_text.set_cursor(false);
                }
            }
            _ => {}
        }
    }
}

///"Time" represents date time.
#[derive(Debug)]
pub(crate) struct Time {
    element: ElementKey,
    rect: FixedRect,
    painter: AppearanceComposite,
    draw_text: DrawText,
}

impl Time {
    pub(crate) fn new(element: ElementKey, eh: &ElementHolder) -> Self {
        Self {
            element,
            rect: (100.0, 100.0).into(),
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

    pub(crate) fn draw(&mut self, dcx: &mut DrawCtx, cx: &mut PageContext) {
        draw_check!(self, cx);

        self.painter.draw(&self.rect, dcx);

        if let Some(s) = cx.text(self.element) {
            self.draw_text.draw(&self.rect, s, dcx);
        }
    }

    pub(crate) fn consume_action(&mut self, acx: &mut ActionCtx, cx: &mut PageContext) {
        match &acx.kind {
            ActionKind::Click(c, _) | ActionKind::DoubleClick(c, _) | ActionKind::Pressed(c, _) => {
                if self.painter.within(&self.rect, c) {
                    self.draw_text.set_cursor(true);
                    acx.finish = true;
                    return;
                } else {
                    self.draw_text.set_cursor(false);
                }
            }
            _ => {}
        }
    }
}
