use super::*;
use std::sync::{Arc, RwLock};

///Represents page.
#[derive(CopyGetters, Getters, MutGetters, Setters)]
pub struct Page {
    context: Arc<RwLock<PageContext>>,
    head: Head,
    body: Body,
    style: Style,
    script: Script,
    callback: Vec<DrawUnitKey>,
    #[getset(get_copy = "pub", set = "pub")]
    scale_factor: f32,
}

deref!(Page, Arc<RwLock<PageContext>>, context);

impl std::fmt::Debug for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Page");
        if let Ok(o) = self.context.try_read() {
            f.field("context", &o);
        }
        f.finish_non_exhaustive()
    }
}

impl Page {
    pub(crate) fn new(
        eh: ElementHolder,
        head_key: ElementKey,
        body_key: ElementKey,
        style_key: ElementKey,
        script_key: ElementKey,
    ) -> Self {
        let mut context = PageContext::new(eh, head_key, body_key, style_key, script_key);
        let head = Head::new(&mut context);
        let body = Body::new(&mut context);
        let style = Style::new(&mut context);
        let script = Script::new(&mut context);
        let context = Arc::new(RwLock::new(context));
        let mut page = Self {
            context,
            head,
            body,
            style,
            script,
            callback: Default::default(),
            scale_factor: 1.0,
        };
        page.script.build(page.context.clone());
        page
    }

    pub(crate) fn renew(&mut self) {
        if let Ok(mut context) = self.context.write() {
            self.head = Head::new(&mut context);
            self.body = Body::new(&mut context);
        }
    }

    ///Parse a string slice.
    fn parse0(buf: &str, o: MarkNumber) -> (Option<Self>, ErrorHolder) {
        let (e, mut err) = ElementHolder::parse(buf, o);
        let p = e.and_then(|e| {
            e.try_into()
                .map_err(|_| err.push((ErrorKind::Markup, "no page").into()))
                .ok()
        });
        (p, err)
    }

    ///Parse a string slice.
    pub fn parse(buf: &str) -> (Option<Self>, ErrorHolder) {
        Self::parse0(buf, MarkNumber::Double)
    }

    ///Parse a string slice.
    pub fn parse_s(buf: &str) -> (Option<Self>, ErrorHolder) {
        Self::parse0(buf, MarkNumber::Single)
    }

    ///Reset width and height, each number is not equal to the size of window if the coordinate is not 0.0
    pub fn resize(&mut self, width: f32, height: f32) {
        if let Ok(mut context) = self.context.write() {
            self.body.resize(width, height, &mut context);
        }
    }

    ///Reset zero point on rectangular coordinates and size. `resize`
    pub fn reset(&mut self, x: f32, y: f32, width: f32, height: f32) {
        if let Ok(mut context) = self.context.write() {
            self.body.reset(x, y, width, height, &mut context);
        }
    }

    pub(crate) fn draw_body(&mut self, surface: skia_safe::Surface) {
        if let Ok(mut context) = self.context.write() {
            self.body.draw(DrawCtx::new(surface), &mut context);
        }
    }

    ///Receive a action.
    pub fn receive_action(&mut self, a: ActionKind) {
        if let Ok(mut context) = self.context.try_write() {
            let o = ActionCtx::new(a, &mut self.callback);
            self.body.consume_action(o, &mut context);
        }
    }
}

///Represents the context of page.
#[derive(CopyGetters, Debug, Getters, MutGetters, Setters)]
pub struct PageContext {
    eh: ElementHolder,
    #[getset(get_copy = "pub")]
    head_key: ElementKey,
    #[getset(get_copy = "pub")]
    body_key: ElementKey,
    #[getset(get_copy = "pub")]
    style_key: ElementKey,
    #[getset(get_copy = "pub")]
    script_key: ElementKey,
}

deref!(PageContext, ElementHolder, eh);

impl std::fmt::Display for PageContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PageContext {{ eh: {}, ", self.eh)?;
        write!(f, "head_key: {:?}, ", self.head_key)?;
        write!(f, "body_key: {:?}, ", self.body_key)?;
        write!(f, "style_key: {:?}, ", self.style_key)?;
        write!(f, "script_key: {:?} }}", self.script_key)
    }
}

impl PageContext {
    fn new(
        eh: ElementHolder,
        head_key: ElementKey,
        body_key: ElementKey,
        style_key: ElementKey,
        script_key: ElementKey,
    ) -> Self {
        Self {
            eh,
            head_key,
            body_key,
            style_key,
            script_key,
        }
    }

    pub fn head_element(&self) -> Option<&Element> {
        self.eh.get(self.head_key)
    }

    pub fn body_element(&self) -> Option<&Element> {
        self.eh.get(self.body_key)
    }

    pub fn style_element(&self) -> Option<&Element> {
        self.eh.get(self.style_key)
    }

    pub fn script_element(&self) -> Option<&Element> {
        self.eh.get(self.script_key)
    }
}

///Represents the kind of action.
#[derive(Clone, Debug)]
pub enum ActionKind {
    Click(Coord2D, u8),
    DoubleClick(Coord2D, u8),
    Pressed(Coord2D, u8),
    Released(u8),
    Focused(bool),
    Cursor(Coord2D, (f32, f32)),
    CursorWithoutFocus(Coord2D, (f32, f32)),
    CursorEntered,
    CursorLeft,
    InputStr(String),
    DeleteFront(usize),
    DeleteBack(usize),
    Sweep(Coord2D, Coord2D, (f32, f32)),
}

impl ActionKind {
    pub(crate) fn set_var_cursor(&mut self, x: f32, y: f32) {
        match self {
            Self::Click(c, _)
            | Self::DoubleClick(c, _)
            | Self::Pressed(c, _)
            | Self::Cursor(c, _)
            | Self::CursorWithoutFocus(c, _) => {
                c.set_x(c.x() + x);
                c.set_y(c.y() + y);
            }
            Self::Sweep(a, b, _) => {
                a.set_x(a.x() + x);
                a.set_y(a.y() + y);
                b.set_x(b.x() + x);
                b.set_y(b.y() + y);
            }
            _ => {}
        }
    }
}
