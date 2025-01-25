use super::*;
use crate::grid::AlignPattern;
use crate::markup::Page;
use skia_safe::{Canvas, EncodedImageFormat, Image};

///"Audio" represents audio stream.
#[derive(Debug)]
pub(crate) struct Audio {
    background: Box<dyn Painter>,
    align_pattern: AlignPattern,
    reader: Option<AudioReader>,
}

impl Audio {
    pub(crate) fn new() -> Self {
        Self {
            background: Box::new(Range::new()),
            align_pattern: AlignPattern::center_middle(),
            reader: None,
        }
    }

    pub(crate) fn draw(&mut self, canvas: &Canvas, page: &mut Page, wrapper: &mut DrawUnitWrapper) {
        let r = wrapper.rect();

        self.background.as_mut().act(&r, canvas);

        if self.reader.is_none() {
            if let Ok(e) = wrapper.element.read() {
                self.reader.replace(AudioReader::new(&e.text));
            }
        }
        if let Some(o) = &mut self.reader {
            o.act(&r, canvas);
        }
    }
}

///"Img" represents an image.
#[derive(Debug)]
pub(crate) struct Img {
    background: Box<dyn Painter>,
    align_pattern: AlignPattern,
    outside: Box<dyn OutPainter>,
    scroll_bar: ScrollBar,
    buffer: Option<Image>,
}

impl Img {
    pub(crate) fn new() -> Self {
        Self {
            background: Box::new(Range::new()),
            align_pattern: AlignPattern::center_middle(),
            outside: Box::new(Border::new()),
            scroll_bar: ScrollBar::new(),
            buffer: None,
        }
    }

    pub(crate) fn draw(&mut self, canvas: &Canvas, page: &mut Page, wrapper: &mut DrawUnitWrapper) {
        let r = wrapper.rect();

        self.outside.as_mut().act(&r, canvas);
        self.background.as_mut().act(&r, canvas);

        if self.buffer.is_none() {
            if let Ok(_) = wrapper.element.read() {
                let o = Vec::new();
                if let Some(i) =
                    get_image(&r.side, canvas, EncodedImageFormat::JPEG, &mut o.as_slice())
                {
                    self.buffer.replace(i);
                }
            }
        }
        if let Some(i) = &self.buffer {
            canvas.draw_image(i, r.pos.clone(), None);
        }
    }
}

///"Video" represents video.
#[derive(Debug)]
pub(crate) struct Video {
    background: Box<dyn Painter>,
    align_pattern: AlignPattern,
    reader: Option<VideoReader>,
}

impl Video {
    pub(crate) fn new() -> Self {
        Self {
            background: Box::new(Range::new()),
            align_pattern: AlignPattern::center_middle(),
            reader: None,
        }
    }

    pub(crate) fn draw(&mut self, canvas: &Canvas, page: &mut Page, wrapper: &mut DrawUnitWrapper) {
        let r = wrapper.rect();

        self.background.as_mut().act(&r, canvas);

        if self.reader.is_none() {
            if let Ok(e) = wrapper.element.read() {
                self.reader.replace(VideoReader::new(&e.text));
            }
        }
        if let Some(o) = &mut self.reader {
            o.act(&r, canvas);
        }
    }
}
