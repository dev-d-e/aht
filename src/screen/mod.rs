#[cfg(feature = "window")]
mod window;

#[cfg(feature = "window")]
pub use self::window::*;
use crate::error::*;
use crate::markup::*;
use crate::page::*;
use crate::utils::*;
use getset::{Getters, MutGetters};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use std::any::Any;
use std::sync::Arc;

///Represents renderer.
#[derive(Getters, MutGetters)]
pub struct Renderer {
    #[cfg(feature = "vulkan")]
    r: crate::gpu::vk::VkRenderer,
}

impl Renderer {
    pub fn new(
        w: Arc<impl HasWindowHandle + HasDisplayHandle + Any + Send + Sync>,
    ) -> Result<Self> {
        if cfg!(feature = "vulkan") {
            crate::gpu::vk::SurfaceQueueHolder::new(w)?
                .try_into()
                .map(|r| Self { r })
        } else {
            warn!("There is no renderer");
            Err((ErrorKind::Gpu, "no renderer").into())
        }
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
