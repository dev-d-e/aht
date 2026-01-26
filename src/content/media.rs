use super::*;
use skia_safe::{EncodedImageFormat, Image, Path, Rect};

///"Audio" represents audio stream.
#[derive(Getters, MutGetters)]
pub(crate) struct Audio {
    #[getset(get = "pub(crate)")]
    element: Arc<RwLock<Element>>,
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    rect: FixedRect,
    painter: AppearanceComposite,
    align_pattern: AlignPattern,
    reader: Option<MediaReader>,
    control: PlayPart,
}

impl std::fmt::Debug for Audio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Audio");
        if let Ok(o) = self.element.try_read() {
            f.field("element", &o.to_string());
        }
        f.field("rect", &self.rect)
            .field("painter", &self.painter)
            .field("reader", &self.reader.is_some())
            .finish()
    }
}

impl Audio {
    pub(crate) fn new(element: Arc<RwLock<Element>>) -> Self {
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

    pub(crate) fn resize(&mut self, c: &mut LayoutCoord) {
        if let Ok(e) = self.element.read() {
            self.rect.get_attr(&e, c);
            self.control.resize(&self.rect);
        }
    }

    right_bottom!();

    pub(crate) fn draw(&mut self, t: &mut DrawCtx) {
        draw_check!(self);

        self.painter.draw(&self.rect, t);

        if self.reader.is_none() {
            if let Ok(e) = self.element.read() {
                self.reader.replace((e.text(), false).into());
            }
        }
        if let Some(o) = &mut self.reader {
            o.no_draw();
        }

        self.control.draw(t);
    }

    pub(crate) fn consume_action(&mut self, t: &mut ActionCtx) {
        match &t.kind {
            ActionKind::Click(c, _) => {
                if self.painter.within(&self.rect, c) {
                    t.finish = true;
                    if self.control.within_pause(c) {
                        if let Some(o) = &mut self.reader {
                            o.pause(self.control.play);
                        }
                        self.control.play_pause();
                    }
                    return;
                }
            }
            ActionKind::Sweep(a, b) => {
                if self.painter.within(&self.rect, b) {
                    t.finish = true;
                    if self.control.within_time(b) || self.control.within_time(a) {
                        self.control.set_rate(b.x() - a.x());
                    }
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

///"Img" represents an image.
#[derive(Getters, MutGetters)]
pub(crate) struct Img {
    #[getset(get = "pub(crate)")]
    element: Arc<RwLock<Element>>,
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    rect: FixedRect,
    painter: AppearanceComposite,
    align_pattern: AlignPattern,
    scroll_bar: ScrollBar,
    buffer: Option<Image>,
}

impl std::fmt::Debug for Img {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Img");
        if let Ok(o) = self.element.try_read() {
            f.field("element", &o.to_string());
        }
        f.field("rect", &self.rect)
            .field("painter", &self.painter)
            .field("buffer", &self.buffer.is_some())
            .finish()
    }
}

impl Img {
    pub(crate) fn new(element: Arc<RwLock<Element>>) -> Self {
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

    pub(crate) fn draw(&mut self, t: &mut DrawCtx) {
        draw_check!(self);

        self.painter.draw(&self.rect, t);

        if self.buffer.is_none() {
            if let Ok(_) = self.element.read() {
                let o = Vec::new();
                if let Some(i) = get_image(
                    self.rect.side(),
                    t,
                    EncodedImageFormat::JPEG,
                    &mut o.as_slice(),
                ) {
                    self.buffer.replace(i);
                }
            }
        }
        if let Some(i) = &self.buffer {
            t.surface.canvas().draw_image(i, &**self.rect, None);
        }
    }

    pub(crate) fn consume_action(&mut self, t: &mut ActionCtx) {
        match &t.kind {
            ActionKind::Click(c, _) => {
                if self.painter.within(&self.rect, c) {
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

///"Video" represents video.
#[derive(Getters, MutGetters)]
pub(crate) struct Video {
    #[getset(get = "pub(crate)")]
    element: Arc<RwLock<Element>>,
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    rect: FixedRect,
    painter: AppearanceComposite,
    align_pattern: AlignPattern,
    reader: Option<MediaReader>,
    control: PlayPart,
}

impl std::fmt::Debug for Video {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Video");
        if let Ok(o) = self.element.try_read() {
            f.field("element", &o.to_string());
        }
        f.field("rect", &self.rect)
            .field("painter", &self.painter)
            .field("reader", &self.reader.is_some())
            .finish()
    }
}

impl Video {
    pub(crate) fn new(element: Arc<RwLock<Element>>) -> Self {
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

    pub(crate) fn resize(&mut self, c: &mut LayoutCoord) {
        if let Ok(e) = self.element.read() {
            self.rect.get_attr(&e, c);
            self.control.resize(&self.rect);
        }
    }

    right_bottom!();

    pub(crate) fn draw(&mut self, t: &mut DrawCtx) {
        draw_check!(self);

        self.painter.draw(&self.rect, t);

        if self.reader.is_none() {
            if let Ok(e) = self.element.read() {
                self.reader.replace((e.text(), true).into());
            }
        }

        if let Some(o) = &mut self.reader {
            o.draw(&self.rect, t);
        }

        self.control.draw(t);
    }

    pub(crate) fn consume_action(&mut self, t: &mut ActionCtx) {
        match &t.kind {
            ActionKind::Click(c, _) => {
                if self.painter.within(&self.rect, c) {
                    t.finish = true;
                    if self.control.within_pause(c) {
                        if let Some(o) = &mut self.reader {
                            o.pause(self.control.play);
                        }
                        self.control.play_pause();
                    }
                    return;
                }
            }
            ActionKind::Sweep(a, b) => {
                if self.painter.within(&self.rect, b) {
                    t.finish = true;
                    if self.control.within_time(b) || self.control.within_time(a) {
                        self.control.set_rate(b.x() - a.x());
                    }
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

#[derive(Debug)]
struct PlayPart {
    color: Color,
    center_x: f32,
    center_y: f32,
    radius: f32,
    rate_0: FixedRect,
    rate_1: FixedRect,
    play: bool,
    time: f32,
    position: Coord,
}

impl Default for PlayPart {
    fn default() -> Self {
        Self {
            color: Color::from_rgb(255, 100, 100),
            center_x: 100.0,
            center_y: 50.0,
            radius: 20.0,
            rate_0: Default::default(),
            rate_1: Default::default(),
            play: true,
            time: 0.0,
            position: Default::default(),
        }
    }
}

impl PlayPart {
    fn resize(&mut self, rect: &FixedRect) {
        self.position.set_x(rect.x() + self.center_x);
        self.position.set_y(rect.bottom() - self.center_y);

        self.rate_0.set_x(self.position.x() + (3.0 * self.radius));
        self.rate_0.set_y(self.position.y() - (self.radius / 2.0));
        let n = self.rate_0.x() - rect.x();
        let o = self.rate_0.side_mut();
        o.set_width((rect.side().width() - 2.0 * n).max(0.0));
        o.set_height(self.radius);

        self.rate_1.set_x(self.rate_0.x() + self.time);
        self.rate_1.set_y(self.rate_0.y());
        let o = self.rate_1.side_mut();
        o.set_width(10.0);
        o.set_height(self.rate_0.side().height());
    }

    fn set_rate(&mut self, n: f32) {
        self.time = (self.time + n).min(self.rate_0.side().width()).max(0.0);
    }

    fn rate(&self) -> (u32, u32) {
        (
            (self.time * 1000.0) as u32,
            (self.rate_0.side().width() * 1000.0) as u32,
        )
    }

    fn draw(&mut self, t: &mut DrawCtx) {
        let paint = &mut t.paint;
        let canvas = t.surface.canvas();
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
        canvas.draw_rect(self.rate_0.to_rect(), paint);
        paint.set_color(Color::from_rgb(200, 0, 200));
        let x = self.rate_0.x() + self.time - self.rate_1.side().width();
        self.rate_1.set_x(x);
        let mut r = self.rate_1.to_rect();
        r.right += self.rate_1.side().width();
        canvas.draw_rect(r, paint);
    }

    fn within_pause(&self, c: &Coord2D) -> bool {
        let a = c.x() - self.position.x();
        let b = c.y() - self.position.y();
        let c = (a * a + b * b).sqrt();
        c <= self.radius
    }

    fn within_time(&self, c: &Coord2D) -> bool {
        self.rate_1.within(c)
    }

    fn play_pause(&mut self) {
        self.play = !self.play;
    }
}
