use super::*;
use getset::{Getters, MutGetters, Setters};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalPosition, LogicalSize};
use winit::event::{ElementState, MouseButton, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::{Window, WindowAttributes, WindowId};

const WINDOW_FPS: f32 = 60.0;

///A builder for an application.
pub struct ScreenBuilder {}

impl Default for ScreenBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl ScreenBuilder {
    ///Run the application.
    pub fn run(&mut self, mut o: WindowContext) -> Result<()> {
        let event_loop: Result<_> = EventLoop::new().map_err(|e| to_err(ErrorKind::Window, e));
        event_loop?
            .run_app(&mut o)
            .map_err(|e| to_err(ErrorKind::Window, e))
    }

    ///Run the application.
    pub fn run_page(&mut self, page: Page) -> Result<()> {
        self.run(page.into())
    }
}

///Represents the context of window application.
#[derive(Getters, MutGetters, Setters)]
pub struct WindowContext {
    #[getset(get = "pub", set = "pub")]
    page: Page,
    r: Option<(Renderer, Arc<Window>)>,
    #[getset(get = "pub", get_mut = "pub")]
    attributes: WindowAttributes,
    fps_ctrl: FpsCtrl,
    fps_counter: FpsCounter,
    event_wrapper: WindowEventWrapper,
}

impl WindowContext {
    pub fn new(page: Page, attributes: WindowAttributes) -> Self {
        Self {
            page,
            r: None,
            attributes,
            fps_ctrl: FpsCtrl::new(WINDOW_FPS),
            fps_counter: Default::default(),
            event_wrapper: Default::default(),
        }
    }
}

impl ApplicationHandler for WindowContext {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.r.is_none() {
            let window = Arc::new(event_loop.create_window(self.attributes.clone()).unwrap());
            let s = Renderer::new(window.clone()).unwrap();
            self.r.replace((s.try_into().unwrap(), window));
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let (renderer, _window) = match self.r.as_mut() {
            Some(o) if o.1.id() == id => o,
            _ => return,
        };
        match event {
            WindowEvent::CloseRequested => {
                self.r.take();
                event_loop.exit();
            }
            WindowEvent::CursorEntered { .. } => {
                self.page.receive_action(ActionKind::CursorEntered);
            }
            WindowEvent::CursorLeft { .. } => {
                self.page.receive_action(ActionKind::CursorLeft);
                self.event_wrapper.clear();
            }
            WindowEvent::CursorMoved { position, .. } => {
                let c: Coord2D = position.to_logical(self.page.scale_factor() as f64).into();
                if let Some(a) = self.event_wrapper.analyse(c) {
                    self.page.receive_action(a);
                }
            }
            WindowEvent::Focused(o) => {
                self.page.receive_action(ActionKind::Focused(o));
                self.event_wrapper.focused = o;
            }
            WindowEvent::KeyboardInput { event, .. } => {
                match event.state {
                    ElementState::Pressed => match event.logical_key {
                        Key::Named(n) => match n {
                            NamedKey::Backspace => {
                                if let Some(c) = &self.event_wrapper.cursor {
                                    self.page
                                        .receive_action(ActionKind::DeleteFront(c.clone(), 1));
                                }
                            }
                            NamedKey::Clear => {}
                            NamedKey::Copy => {}
                            NamedKey::Cut => {}
                            NamedKey::Delete => {
                                if let Some(c) = &self.event_wrapper.cursor {
                                    self.page
                                        .receive_action(ActionKind::DeleteBack(c.clone(), 1));
                                }
                            }
                            NamedKey::Insert => {}
                            _ => {}
                        },
                        Key::Character(s) => {
                            if s.len() > 0 {
                                self.page
                                    .receive_action(ActionKind::InputStr(s.to_string()));
                            }
                        }
                        _ => {}
                    },
                    ElementState::Released => {}
                };
            }
            WindowEvent::MouseInput { state, button, .. } => match button {
                MouseButton::Left => {
                    match state {
                        ElementState::Pressed => {
                            if let Some(c) = &self.event_wrapper.cursor {
                                self.page.receive_action(ActionKind::Click(c.clone(), 0));
                            }
                            self.event_wrapper.pressed.replace(0);
                        }
                        ElementState::Released => {
                            self.event_wrapper.pressed.take();
                        }
                    };
                }
                _ => {}
            },
            WindowEvent::RedrawRequested => {
                renderer.draw(&mut self.page);
                self.fps_counter.count();
            }
            WindowEvent::Resized(n) => {
                renderer.resize(n.width, n.height);
                let o: LogicalSize<f32> = n.to_logical(self.page.scale_factor() as f64);
                self.page.resize(o.width, o.height);
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.page.set_scale_factor(scale_factor as f32);
            }
            _ => {}
        }
    }

    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        let _ = (event_loop, cause);
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: ()) {
        let _ = (event_loop, event);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some((_, window)) = self.r.as_mut() {
            if let Some(o) = self.fps_ctrl.need_to_wait() {
                event_loop.set_control_flow(ControlFlow::WaitUntil(o));
            } else {
                window.request_redraw();
            }
        }
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        self.r.take();
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        let _ = event_loop;
    }

    fn memory_warning(&mut self, event_loop: &ActiveEventLoop) {
        let _ = event_loop;
    }
}

impl From<Page> for WindowContext {
    fn from(o: Page) -> Self {
        Self::new(o, WindowAttributes::default().with_title("AHT Window"))
    }
}

impl From<LogicalPosition<f32>> for Coord2D {
    fn from(o: LogicalPosition<f32>) -> Self {
        Self::new(o.x, o.y)
    }
}

struct WindowEventWrapper {
    focused: bool,
    cursor: Option<Coord2D>,
    pressed: Option<u8>,
}

impl Default for WindowEventWrapper {
    fn default() -> Self {
        Self {
            focused: false,
            cursor: None,
            pressed: None,
        }
    }
}

impl WindowEventWrapper {
    fn clear(&mut self) {
        self.cursor.take();
        self.pressed.take();
    }

    fn analyse(&mut self, o: Coord2D) -> Option<ActionKind> {
        let mut r = None;
        if let Some(_) = self.pressed {
            if let Some(a) = &self.cursor {
                r.replace(ActionKind::Sweep(a.clone(), o.clone()));
            }
        }
        self.cursor.replace(o);
        r
    }
}
