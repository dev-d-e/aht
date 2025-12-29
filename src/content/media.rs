use super::*;
use skia_safe::{EncodedImageFormat, Image};

///"Audio" represents audio stream.
#[derive(Getters, MutGetters)]
pub(crate) struct Audio {
    #[getset(get = "pub(crate)")]
    element: Arc<RwLock<Element>>,
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    rect: FixedRect,
    painter: AppearanceComposite,
    align_pattern: AlignPattern,
    reader: Option<AudioReader>,
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
            rect: (100.0, 100.0).into(),
            painter: Rectangle {
                color: *default_surface_color(),
                ..Default::default()
            }
            .into(),
            align_pattern: AlignPattern::center_middle(),
            reader: None,
        }
    }

    resize!();

    right_bottom!();

    pub(crate) fn draw(&mut self, t: &mut DrawCtx) {
        draw_check!(self);

        self.painter.draw(&self.rect, t);

        if self.reader.is_none() {
            if let Ok(e) = self.element.read() {
                self.reader.replace(AudioReader::new(e.text()));
            }
        }
        if let Some(o) = &mut self.reader {
            o.draw(&self.rect, t);
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
    reader: Option<VideoReader>,
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
                color: *default_surface_color(),
            }
            .into(),
            align_pattern: AlignPattern::center_middle(),
            reader: None,
        }
    }

    resize!();

    right_bottom!();

    pub(crate) fn draw(&mut self, t: &mut DrawCtx) {
        draw_check!(self);

        if self.reader.is_none() {
            if let Ok(e) = self.element.read() {
                self.reader.replace(VideoReader::new(e.text()));
            }
        }
        if let Some(o) = &mut self.reader {
            o.draw(&self.rect, t);
        } else {
            self.painter.draw(&self.rect, t);
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
