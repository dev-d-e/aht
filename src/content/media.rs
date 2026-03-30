use super::*;
use skia_safe::{Image, Path, Rect};

///"Audio" represents audio stream.
#[derive(Debug)]
pub(crate) struct Audio {
    element: ElementKey,
    rect: FixedRect,
    painter: AppearanceComposite,
    align_pattern: AlignPattern,
    reader: Option<MediaReader>,
    control: PlayPart,
}

impl Audio {
    pub(crate) fn new(element: ElementKey, eh: &ElementHolder) -> Self {
        Self {
            element,
            rect: (1000.0, 100.0).into(),
            painter: Rectangle {
                color: *default_surface_color(),
                ..Default::default()
            }
            .into(),
            align_pattern: AlignPattern::center_middle(),
            reader: None,
            control: Default::default(),
        }
    }

    pub(crate) fn resize(&mut self, c: &mut LayoutCoord, cx: &mut PageContext) {
        if let Some(e) = cx.get(self.element) {
            self.rect.get_attr(&e, c);
            self.control.resize(&self.rect);
        }
    }

    right_bottom!();

    pub(crate) fn draw(&mut self, dcx: &mut DrawCtx, cx: &mut PageContext) {
        draw_check!(self, cx);

        self.painter.draw(&self.rect, dcx);

        if self.reader.is_none() {
            if let Some(s) = cx.text(self.element) {
                self.reader.replace((s, false).into());
            }
        }

        if let Some(o) = &mut self.reader {
            o.no_draw();
            self.control.move_rate(o);
        }

        self.control.draw(dcx);
    }

    pub(crate) fn consume_action(&mut self, acx: &mut ActionCtx, cx: &mut PageContext) {
        match &acx.kind {
            ActionKind::Click(c, _) | ActionKind::Pressed(c, _) => {
                if self.painter.within(&self.rect, c) {
                    acx.finish = true;
                    if self.control.within_pause(c) {
                        if let Some(o) = &mut self.reader {
                            o.pause(self.control.play);
                        }
                        self.control.play_pause();
                    } else if self.control.within_rate(c) {
                    } else if self.control.within_rate_bar(c) {
                        self.control.set_rate(c);
                        if let Some(o) = &mut self.reader {
                            o.seek(self.control.rate());
                        }
                        self.control.sweep = false;
                    }
                    return;
                }
            }
            ActionKind::DoubleClick(c, _)
            | ActionKind::Cursor(c, _)
            | ActionKind::CursorWithoutFocus(c, _) => {
                if self.control.within_rate_bar(c) {
                    acx.finish = true;
                    return;
                }
            }
            ActionKind::Released(_) => {
                if self.control.sweep {
                    if let Some(o) = &mut self.reader {
                        o.seek(self.control.rate());
                    }
                    self.control.sweep = false;
                }
            }
            ActionKind::Sweep(b, a, d) => {
                if self.painter.within(&self.rect, b) {
                    acx.finish = true;
                    if self.control.within_rate(b) {
                        self.control.set_rate(b);
                        self.control.sweep = true;
                        acx.push_callback();
                    }
                    return;
                }
            }
            _ => {}
        }
    }
}

///"Img" represents an image.
#[derive(Debug)]
pub(crate) struct Img {
    element: ElementKey,
    rect: FixedRect,
    painter: AppearanceComposite,
    align_pattern: AlignPattern,
    scroll_bar: ScrollBar,
    buffer: Option<Image>,
}

impl Img {
    pub(crate) fn new(element: ElementKey, eh: &ElementHolder) -> Self {
        Self {
            element,
            rect: (100.0, 100.0).into(),
            painter: RectangleCurve {
                color: *default_border_color(),
                ..Default::default()
            }
            .into(),
            align_pattern: AlignPattern::center_middle(),
            scroll_bar: Default::default(),
            buffer: None,
        }
    }

    resize!();

    right_bottom!();

    pub(crate) fn draw(&mut self, dcx: &mut DrawCtx, cx: &mut PageContext) {
        draw_check!(self, cx);

        self.painter.draw(&self.rect, dcx);

        if self.buffer.is_none() {
            if let Some(i) = cx
                .text(self.element)
                .and_then(|s| get_image(self.rect.side(), s))
            {
                self.buffer.replace(i);
            }
        }
        if let Some(i) = &self.buffer {
            dcx.surface.canvas().draw_image(i, &**self.rect, None);
        }
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

///"Video" represents video.
#[derive(Debug)]
pub(crate) struct Video {
    element: ElementKey,
    rect: FixedRect,
    painter: AppearanceComposite,
    align_pattern: AlignPattern,
    reader: Option<MediaReader>,
    control: PlayPart,
}

impl Video {
    pub(crate) fn new(element: ElementKey, eh: &ElementHolder) -> Self {
        Self {
            element,
            rect: (100.0, 100.0).into(),
            painter: Rectangle {
                color: Color::from_rgb(0, 0, 0),
            }
            .into(),
            align_pattern: AlignPattern::center_middle(),
            reader: None,
            control: Default::default(),
        }
    }

    pub(crate) fn resize(&mut self, c: &mut LayoutCoord, cx: &mut PageContext) {
        if let Some(e) = cx.get(self.element) {
            self.rect.get_attr(&e, c);
            self.control.resize(&self.rect);
        }
    }

    right_bottom!();

    pub(crate) fn draw(&mut self, dcx: &mut DrawCtx, cx: &mut PageContext) {
        draw_check!(self, cx);

        self.painter.draw(&self.rect, dcx);

        if self.reader.is_none() {
            if let Some(s) = cx.text(self.element) {
                self.reader.replace((s, true).into());
            }
        }

        if let Some(o) = &mut self.reader {
            o.draw(&self.rect, dcx);
            self.control.move_rate(o);
        }

        self.control.draw(dcx);
    }

    pub(crate) fn consume_action(&mut self, acx: &mut ActionCtx, cx: &mut PageContext) {
        match &acx.kind {
            ActionKind::Click(c, _) | ActionKind::Pressed(c, _) => {
                if self.painter.within(&self.rect, c) {
                    acx.finish = true;
                    if self.control.within_pause(c) {
                        if let Some(o) = &mut self.reader {
                            o.pause(self.control.play);
                        }
                        self.control.play_pause();
                    } else if self.control.within_rate(c) {
                    } else if self.control.within_rate_bar(c) {
                        self.control.set_rate(c);
                        if let Some(o) = &mut self.reader {
                            o.seek(self.control.rate());
                        }
                        self.control.sweep = false;
                    }
                    return;
                }
            }
            ActionKind::DoubleClick(c, _)
            | ActionKind::Cursor(c, _)
            | ActionKind::CursorWithoutFocus(c, _) => {
                if self.control.within_rate_bar(c) {
                    acx.finish = true;
                    return;
                }
            }
            ActionKind::Released(_) => {
                if self.control.sweep {
                    if let Some(o) = &mut self.reader {
                        o.seek(self.control.rate());
                    }
                    self.control.sweep = false;
                }
            }
            ActionKind::Sweep(b, a, d) => {
                if self.painter.within(&self.rect, b) {
                    acx.finish = true;
                    if self.control.within_rate(b) {
                        self.control.set_rate(b);
                        self.control.sweep = true;
                        acx.push_callback();
                    }
                    return;
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
struct PlayPart {
    color: Color,
    center_x: f32,
    center_y: f32,
    radius: f32,
    position: Coord,
    rate_bar: FixedRect,
    play: bool,
    rate: f32,
    sweep: bool,
}

impl Default for PlayPart {
    fn default() -> Self {
        Self {
            color: Color::from_rgb(255, 100, 100),
            center_x: 100.0,
            center_y: 50.0,
            radius: 20.0,
            position: Default::default(),
            rate_bar: Default::default(),
            play: true,
            rate: 0.0,
            sweep: false,
        }
    }
}

impl PlayPart {
    fn resize(&mut self, rect: &FixedRect) {
        self.position.set_x(rect.x() + self.center_x);
        self.position.set_y(rect.bottom() - self.center_y);

        self.rate_bar.set_x(self.position.x() + (3.0 * self.radius));
        self.rate_bar.set_y(self.position.y() - (self.radius / 2.0));
        let n = self.rate_bar.x() - rect.x();
        let o = self.rate_bar.side_mut();
        o.set_width((rect.side().width() - 2.0 * n).max(0.0));
        o.set_height(self.radius);
    }

    fn rate_width(&self) -> f32 {
        self.rate_bar.side().height()
    }

    fn rate_x(&self) -> f32 {
        self.rate_bar.x() + self.rate - self.rate_width() / 2.0
    }

    fn draw(&mut self, dcx: &mut DrawCtx) {
        let paint = &mut dcx.paint;
        let canvas = dcx.surface.canvas();
        paint.set_color(self.color);
        let p = &*self.position;
        canvas.draw_circle(p, self.radius, paint);

        paint.set_color(Color::from_rgb(0, 200, 200));
        if self.play {
            let a = self.radius / 2.0;
            let rec = Rect::new(p.x() - a, p.y() - a, p.x() + a, p.y() + a);
            let b = a / 2.0;
            let rec2 = Rect::new(p.x() - b, p.y() - a, p.x() + b, p.y() + a);
            canvas.draw_rect(rec, paint);
            paint.set_color(self.color);
            canvas.draw_rect(rec2, paint);
        } else {
            let a = self.radius / 2.1;
            let v = [
                (p.x() - a, p.y() - a).into(),
                (p.x() + (1.2 * a), p.y()).into(),
                (p.x() - a, p.y() + a).into(),
            ];
            let path = Path::polygon(&v, true, None, None);
            canvas.draw_path(&path, paint);
        }

        paint.set_color(self.color);
        canvas.draw_rect(self.rate_bar.to_rect(), paint);
        paint.set_color(Color::from_rgb(200, 0, 200));
        let rec = Rect::from_xywh(
            self.rate_x(),
            self.rate_bar.y(),
            self.rate_width(),
            self.rate_bar.side().height(),
        );
        canvas.draw_rect(rec, paint);
    }

    fn within_pause(&self, c: &Coord2D) -> bool {
        let a = c.x() - self.position.x();
        let b = c.y() - self.position.y();
        let c = (a * a + b * b).sqrt();
        c <= self.radius
    }

    fn within_rate_bar(&self, c: &Coord2D) -> bool {
        self.rate_bar.within(c)
    }

    fn within_rate(&self, c: &Coord2D) -> bool {
        let x = self.rate_x();
        between(c.x(), x, x + self.rate_width())
            && between(c.y(), self.rate_bar.y(), self.rate_bar.bottom())
    }

    fn play_pause(&mut self) {
        self.play = !self.play;
    }

    fn set_rate(&mut self, p: &Coord2D) {
        self.rate = (p.x() - self.rate_bar.x())
            .min(self.rate_bar.side().width())
            .max(0.0);
    }

    fn move_rate(&mut self, r: &mut MediaReader) {
        let a = self.rate_bar.side().width();
        if let Some(b) = r.rate_var(a) {
            self.rate = (self.rate + b).min(a).max(0.0);
            if self.rate == a && self.play {
                self.play = false;
                self.rate = 0.0;
                r.pause(true);
                r.seek(self.rate());
            }
        }
    }

    fn rate(&self) -> (u32, u32) {
        (
            (self.rate * 1000.0) as u32,
            (self.rate_bar.side().width() * 1000.0) as u32,
        )
    }
}
