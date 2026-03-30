use super::*;

///"Canv" represents canvas.
#[derive(Debug)]
pub(crate) struct Canv {
    element: ElementKey,
    rect: FixedRect,
    painter: AppearanceComposite,
    scroll_bar: ScrollBar,
}

impl Canv {
    pub(crate) fn new(element: ElementKey, eh: &ElementHolder) -> Self {
        Self {
            element,
            rect: (100.0, 100.0).into(),
            painter: Default::default(),
            scroll_bar: Default::default(),
        }
    }

    resize!();

    right_bottom!();

    pub(crate) fn draw(&mut self, dcx: &mut DrawCtx, cx: &mut PageContext) {
        draw_check!(self, cx);

        self.painter.draw(&self.rect, dcx);
    }

    pub(crate) fn consume_action(&mut self, acx: &mut ActionCtx, cx: &mut PageContext) {
        match &acx.kind {
            ActionKind::Click(c, _)
            | ActionKind::DoubleClick(c, _)
            | ActionKind::Pressed(c, _)
            | ActionKind::Cursor(c, _)
            | ActionKind::CursorWithoutFocus(c, _)
            | ActionKind::Sweep(c, _, _) => {
                if self.painter.within(&self.rect, c) {
                    acx.finish = true;
                    return;
                }
            }
            _ => {}
        }
    }
}

///"Iframe" represents iframe.
#[derive(Debug)]
pub(crate) struct Iframe {
    element: ElementKey,
    rect: FixedRect,
    painter: AppearanceComposite,
}

impl Iframe {
    pub(crate) fn new(element: ElementKey, eh: &ElementHolder) -> Self {
        Self {
            element,
            rect: (100.0, 100.0).into(),
            painter: Default::default(),
        }
    }

    resize!();

    right_bottom!();

    pub(crate) fn draw(&mut self, dcx: &mut DrawCtx, cx: &mut PageContext) {
        draw_check!(self, cx);

        self.painter.draw(&self.rect, dcx);
    }

    pub(crate) fn consume_action(&mut self, acx: &mut ActionCtx, cx: &mut PageContext) {
        match &acx.kind {
            ActionKind::Click(c, _)
            | ActionKind::DoubleClick(c, _)
            | ActionKind::Pressed(c, _)
            | ActionKind::Cursor(c, _)
            | ActionKind::CursorWithoutFocus(c, _)
            | ActionKind::Sweep(c, _, _) => {
                if self.painter.within(&self.rect, c) {
                    acx.finish = true;
                    return;
                }
            }
            _ => {}
        }
    }
}
