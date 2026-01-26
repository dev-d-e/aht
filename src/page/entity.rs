use super::*;
use getset::{CopyGetters, Getters, MutGetters, Setters};
use std::sync::{Arc, RwLock};

///Represents page.
pub struct Page {
    context: PageContext,
    head: Head,
    body: Body,
    style: Style,
    script: Script,
    callback: Vec<Box<dyn FnMut(&ActionKind, &mut PageContext)>>,
}

deref!(Page, PageContext, context);

impl std::fmt::Debug for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Page")
            .field("head", &self.head)
            .field("body", &self.body)
            .field("style", &self.style)
            .field("script", &self.script)
            .finish_non_exhaustive()
    }
}

impl Page {
    pub(crate) fn new(
        root: Element,
        head_element: Arc<RwLock<Element>>,
        body_element: Arc<RwLock<Element>>,
        style_element: Arc<RwLock<Element>>,
        script_element: Arc<RwLock<Element>>,
    ) -> Self {
        let head = Head::new(head_element.clone());
        let body = Body::new(body_element.clone());
        let style = Style::new(style_element.clone());
        let script = Script::new(script_element.clone());
        let mut page = Self {
            context: PageContext::new(
                root,
                head_element,
                body_element,
                style_element,
                script_element,
            ),
            head,
            body,
            style,
            script,
            callback: Default::default(),
        };
        page.style.build(&mut page.context);
        page.script.run(&mut page.context);
        page
    }

    pub(crate) fn renew(&mut self) {
        self.head = Head::new(self.head_element.clone());
        self.body = Body::new(self.body_element.clone());
        self.style = Style::new(self.style_element.clone());
        self.script = Script::new(self.script_element.clone());
        self.style.build(&mut self.context);
        self.script.run(&mut self.context);
    }

    ///Parse a string slice.
    fn parse0(buf: &str, o: MarkNumber) -> (Option<Self>, ErrorHolder) {
        let (e, mut err) = Element::parse_one(buf, o);
        let p = e
            .map(|e| {
                e.to_page()
                    .map_err(|_| err.push((ErrorKind::Markup, "no page").into()))
                    .ok()
            })
            .flatten();
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
        self.body.resize(width, height);
    }

    ///Reset zero point on rectangular coordinates and size. `resize`
    pub fn reset(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.body.reset(x, y, width, height);
    }

    pub(crate) fn draw_body(&mut self, surface: skia_safe::Surface) {
        self.body.draw(DrawCtx::new(surface, &mut self.context));
    }

    ///Receive a action.
    pub fn receive_action(&mut self, a: ActionKind) {
        for f in &mut self.callback {
            f(&a, &mut self.context);
        }

        if let Some(a) = self.context.consume_action(a) {
            let o = ActionCtx::new(a, &mut self.context, &mut self.callback);
            self.body.consume_action(o);
        }
    }
}

///Represents the context of page.
#[derive(CopyGetters, Getters, MutGetters, Setters)]
pub struct PageContext {
    root: Arc<RwLock<Element>>,
    #[getset(get = "pub")]
    head_element: Arc<RwLock<Element>>,
    #[getset(get = "pub")]
    body_element: Arc<RwLock<Element>>,
    #[getset(get = "pub")]
    style_element: Arc<RwLock<Element>>,
    #[getset(get = "pub")]
    script_element: Arc<RwLock<Element>>,
    #[getset(get_copy = "pub", set = "pub")]
    scale_factor: f32,
    input_to: Option<Arc<RwLock<Element>>>,
}

impl PageContext {
    fn new(
        root: Element,
        head_element: Arc<RwLock<Element>>,
        body_element: Arc<RwLock<Element>>,
        style_element: Arc<RwLock<Element>>,
        script_element: Arc<RwLock<Element>>,
    ) -> Self {
        Self {
            root: Arc::new(RwLock::new(root)),
            head_element,
            body_element,
            style_element,
            script_element,
            scale_factor: 1.0,
            input_to: None,
        }
    }

    pub(crate) fn set_input_to(&mut self, a: Arc<RwLock<Element>>) {
        self.input_to.replace(a);
    }

    pub(crate) fn take_input_to(&mut self) {
        self.input_to.take();
    }

    fn consume_action(&mut self, a: ActionKind) -> Option<ActionKind> {
        match &a {
            ActionKind::Click(_, _) => {
                self.input_to.take();
            }
            ActionKind::DoubleClick(_, _) => {}
            ActionKind::Focused(o) => {
                if !o {
                    self.input_to.take();
                }
            }
            ActionKind::Cursor(_) => {}
            ActionKind::CursorWithoutFocus(_) => {}
            ActionKind::CursorEntered => {}
            ActionKind::CursorLeft => {}
            ActionKind::InputStr(s) => {
                if s.len() > 0 {
                    if let Some(o) = &mut self.input_to {
                        if let Ok(mut e) = o.try_write() {
                            if let Some(a) = e.attribute_mut().value_or_insert() {
                                a.push_str(&s);
                            }
                        }
                    }
                }
                return None;
            }
            ActionKind::DeleteFront(_, n) => {
                if let Some(o) = &mut self.input_to {
                    if let Ok(mut e) = o.try_write() {
                        if let Some(a) = e.attribute_mut().value_or_insert() {
                            let mut n = *n;
                            while n > 0 {
                                a.pop();
                                n -= 1;
                            }
                        }
                    }
                }
                return None;
            }
            ActionKind::DeleteBack(_, n) => {
                if let Some(o) = &mut self.input_to {
                    if let Ok(mut e) = o.try_write() {
                        if let Some(a) = e.attribute_mut().value_or_insert() {
                            let mut n = *n;
                            while n > 0 {
                                a.pop();
                                n -= 1;
                            }
                        }
                    }
                }
                return None;
            }
            ActionKind::Sweep(_, _) => {}
        }
        Some(a)
    }
}

///Represents the kind of action.
#[derive(Clone, Debug)]
pub enum ActionKind {
    Click(Coord2D, u8),
    DoubleClick(Coord2D, u8),
    Focused(bool),
    Cursor(Coord2D),
    CursorWithoutFocus(Coord2D),
    CursorEntered,
    CursorLeft,
    InputStr(String),
    DeleteFront(Coord2D, u32),
    DeleteBack(Coord2D, u32),
    Sweep(Coord2D, Coord2D),
}

impl ActionKind {
    pub(crate) fn set_var_cursor(&mut self, x: f32, y: f32) {
        match self {
            Self::Click(c, _)
            | Self::DoubleClick(c, _)
            | Self::Cursor(c)
            | Self::CursorWithoutFocus(c)
            | Self::DeleteFront(c, _)
            | Self::DeleteBack(c, _) => {
                c.set_x(c.x() + x);
                c.set_y(c.y() + y);
            }
            Self::Sweep(a, b) => {
                a.set_x(a.x() + x);
                a.set_y(a.y() + y);
                b.set_x(b.x() + x);
                b.set_y(b.y() + y);
            }
            _ => {}
        }
    }
}
