#[cfg(feature = "window")]
mod window;

#[cfg(feature = "window")]
pub use self::window::*;
use crate::error::*;
use crate::markup::*;
use crate::page::*;
use crate::utils::*;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use std::any::Any;
use std::sync::Arc;

///Represents renderer.
#[derive(Getters, MutGetters)]
pub struct Renderer {
    #[cfg(any(target_os = "linux", target_os = "android"))]
    r: crate::gpu::vk::VkRenderer,
}

#[cfg(any(
    target_os = "linux",
    target_os = "android",
    target_os = "ios",
    target_os = "macos",
    target_os = "windows"
))]
impl Renderer {
    #[cfg(any(target_os = "linux", target_os = "android"))]
    pub fn new<T>(w: Arc<T>) -> Option<Self>
    where
        T: HasWindowHandle + HasDisplayHandle + Any + Send + Sync,
    {
        crate::gpu::vk::SurfaceQueueHolder::new(w)
            .and_then(|o| o.try_into())
            .map_err(|e| error!("Renderer: {e}"))
            .map(|r| Self { r })
            .ok()
    }

    ///Resize physical width and height.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.r.resize(width, height);
    }

    ///Render a page.
    pub fn draw(&mut self, page: &mut Page) {
        self.r.draw(|s| {
            page.draw_body(s);
        });
    }
}

#[cfg(not(any(
    target_os = "linux",
    target_os = "android",
    target_os = "ios",
    target_os = "macos",
    target_os = "windows"
)))]
impl Renderer {
    pub fn new<T>(w: Arc<T>) -> Option<Self>
    where
        T: HasWindowHandle + HasDisplayHandle + Any + Send + Sync,
    {
        error!("There is no renderer");
        None
    }

    pub fn resize(&mut self, width: u32, height: u32) {}

    pub fn draw(&mut self, page: &mut Page) {}
}
