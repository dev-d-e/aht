/*!
A module for use Vulkan.

If you intend to show graphics on a window or a screen, create a [`SurfaceQueueHolder`], and then create a [`VkRenderer`].
*/

mod context;

use self::context::*;
use super::*;
use getset::Getters;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use skia_safe::gpu::DirectContext;
use std::any::Any;
use std::cmp::max;
use std::collections::BTreeMap;
use std::sync::{Arc, LazyLock};
use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType};
use vulkano::device::{
    Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo, QueueFlags,
};
use vulkano::image::view::ImageView;
use vulkano::image::ImageUsage;
use vulkano::instance::{Instance, InstanceCreateInfo, InstanceExtensions};
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass};
use vulkano::swapchain::{
    acquire_next_image, PresentMode, Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo,
};
use vulkano::sync::{now, GpuFuture};
use vulkano::{single_pass_renderpass, VulkanLibrary};

static INSTANCE: LazyLock<InstanceHolder> = LazyLock::new(|| InstanceHolder::new().unwrap());

///Represents an instance and physical devices.
#[derive(Getters)]
pub struct InstanceHolder {
    #[getset(get = "pub")]
    instance: Arc<Instance>,
    #[getset(get = "pub")]
    physical_devices: Vec<Arc<PhysicalDevice>>,
}

impl InstanceHolder {
    ///Creates a new instance, get available physical devices.
    pub fn new() -> Result<Self> {
        let library = VulkanLibrary::new().map_err(|e| to_err(ErrorKind::Gpu, e))?;
        let c = InstanceCreateInfo {
            enabled_extensions: InstanceExtensions {
                khr_surface: true,
                #[cfg(target_os = "android")]
                khr_android_surface: true,
                #[cfg(any(target_os = "ios", target_os = "macos"))]
                ext_metal_surface: true,
                #[cfg(target_os = "linux")]
                khr_wayland_surface: true,
                ..InstanceExtensions::empty()
            },
            ..Default::default()
        };
        let instance = Instance::new(library, c).map_err(|e| to_err(ErrorKind::Gpu, e))?;
        instance
            .enumerate_physical_devices()
            .map_err(|e| to_err(ErrorKind::Gpu, e))
            .map(|d| Self {
                instance,
                physical_devices: d.collect(),
            })
    }

    ///Returns a physical device by a surface.
    pub fn physical_device(&self, surface: Arc<Surface>) -> Result<Arc<PhysicalDevice>> {
        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..Default::default()
        };
        self.physical_devices
            .iter()
            .filter(|p| {
                p.supported_extensions().contains(&device_extensions)
                    && p.queue_family_properties()
                        .iter()
                        .enumerate()
                        .find(|(i, q)| {
                            q.queue_flags.intersects(QueueFlags::GRAPHICS)
                                && p.surface_support(*i as u32, &surface).unwrap_or(false)
                        })
                        .is_some()
            })
            .min_by_key(device_type)
            .ok_or_else(|| (ErrorKind::Gpu, "graphics queue was not found").into())
            .cloned()
    }

    ///Returns a physical device at random.
    pub fn random_physical_device(&self) -> Result<Arc<PhysicalDevice>> {
        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..Default::default()
        };
        self.physical_devices
            .iter()
            .filter(|p| {
                p.supported_extensions().contains(&device_extensions)
                    && p.queue_family_properties()
                        .iter()
                        .find(|q| q.queue_flags.intersects(QueueFlags::GRAPHICS))
                        .is_some()
            })
            .min_by_key(device_type)
            .ok_or_else(|| (ErrorKind::Gpu, "graphics queue was not found").into())
            .cloned()
    }
}

#[inline]
fn device_type(pd: &&Arc<PhysicalDevice>) -> usize {
    match pd.properties().device_type {
        PhysicalDeviceType::DiscreteGpu => 0,
        PhysicalDeviceType::IntegratedGpu => 1,
        PhysicalDeviceType::VirtualGpu => 2,
        PhysicalDeviceType::Cpu => 3,
        PhysicalDeviceType::Other => 4,
        _ => 5,
    }
}

#[inline]
fn queue_family_flags(pd: &Arc<PhysicalDevice>) -> BTreeMap<u32, QueueFlags> {
    let qf = pd.queue_family_properties();
    let mut o = BTreeMap::new();
    for i in 0..qf.len() {
        let f = qf[i].queue_flags;
        if f.intersects(QueueFlags::GRAPHICS) || f.intersects(QueueFlags::COMPUTE) {
            o.insert(i as u32, f);
        }
    }
    o
}

///Represents a device and accompanying queues.
#[derive(Getters)]
pub struct DeviceQueueHolder {
    #[getset(get = "pub")]
    device: Arc<Device>,
    #[getset(get = "pub")]
    queue_graphics: Vec<Arc<Queue>>,
    #[getset(get = "pub")]
    queue_compute: Vec<Arc<Queue>>,
}

impl DeviceQueueHolder {
    ///Creates a device and accompanying queues from a physical device.
    pub fn new(physical_device: Arc<PhysicalDevice>) -> Result<Self> {
        let qf = queue_family_flags(&physical_device);
        if qf.is_empty() {
            return Err((ErrorKind::Gpu, "queue family was not found").into());
        }

        let queue_create_infos = qf
            .keys()
            .map(|i| QueueCreateInfo {
                queue_family_index: *i,
                ..Default::default()
            })
            .collect();
        let d = DeviceCreateInfo {
            queue_create_infos,
            enabled_extensions: DeviceExtensions {
                khr_swapchain: true,
                ..Default::default()
            },
            ..Default::default()
        };
        let (device, queue) =
            Device::new(physical_device, d).map_err(|e| to_err(ErrorKind::Gpu, e))?;

        let (queue_graphics, mut queue_compute): (Vec<Arc<Queue>>, Vec<Arc<Queue>>) = queue
            .partition(|q| {
                qf.get(&q.queue_family_index())
                    .map(|f| f.intersects(QueueFlags::GRAPHICS))
                    .unwrap_or(false)
            });
        if queue_graphics.is_empty() {
            return Err((ErrorKind::Gpu, "graphics queue was not found").into());
        } else if queue_compute.is_empty() {
            queue_compute.push(queue_graphics.last().unwrap().clone());
        }
        Ok(Self {
            device,
            queue_graphics,
            queue_compute,
        })
    }

    ///Creates a device and accompanying queues from a random physical device.
    pub fn with_random_physical_device() -> Result<Self> {
        let o = INSTANCE.random_physical_device()?;
        Self::new(o)
    }
}

///Represents a device and accompanying queues and a surface.
#[derive(Getters)]
pub struct SurfaceQueueHolder {
    queue: DeviceQueueHolder,
    #[getset(get = "pub")]
    surface: Arc<Surface>,
}

deref!(SurfaceQueueHolder, DeviceQueueHolder, queue);

impl SurfaceQueueHolder {
    ///Creates a device and queues and a surface from a closure.
    pub fn new(
        w: Arc<impl HasWindowHandle + HasDisplayHandle + Any + Send + Sync>,
    ) -> Result<Self> {
        let i = &INSTANCE;
        let surface =
            Surface::from_window(i.instance().clone(), w).map_err(|e| to_err(ErrorKind::Gpu, e))?;
        let p = i.physical_device(surface.clone())?;
        DeviceQueueHolder::new(p).map(|queue| Self { queue, surface })
    }
}

#[inline]
fn swapchain(device: Arc<Device>, surface: Arc<Surface>) -> Result<Arc<Swapchain>> {
    let physical_device = device.physical_device();
    let surface_capabilities = physical_device
        .surface_capabilities(&surface, Default::default())
        .map_err(|e| to_err(ErrorKind::Gpu, e))?;

    let (image_format, image_color_space) = physical_device
        .surface_formats(&surface, Default::default())
        .map_err(|e| to_err(ErrorKind::Gpu, e))?[0];
    Swapchain::new(
        device,
        surface,
        SwapchainCreateInfo {
            min_image_count: max(surface_capabilities.min_image_count, 2),
            image_format,
            image_color_space,
            image_extent: surface_capabilities.current_extent.unwrap_or([100, 100]),
            image_usage: ImageUsage::COLOR_ATTACHMENT,
            pre_transform: surface_capabilities.current_transform,
            present_mode: PresentMode::Fifo,
            ..Default::default()
        },
    )
    .map_err(|e| to_err(ErrorKind::Gpu, e))
    .map(|(swapchain, _images)| swapchain)
}

///Represents renderer.
#[derive(Getters)]
pub struct VkRenderer {
    #[getset(get = "pub")]
    holder: DeviceQueueHolder,
    #[getset(get = "pub")]
    swapchain: Arc<Swapchain>,
    render_pass: Arc<RenderPass>,
    acquire_future: Option<Box<dyn GpuFuture>>,
    framebuffers: Vec<Arc<Framebuffer>>,
    skia_context: DirectContext,
    width: u32,
    height: u32,
    reflesh: bool,
}

impl TryFrom<SurfaceQueueHolder> for VkRenderer {
    type Error = Error;

    fn try_from(queue: SurfaceQueueHolder) -> Result<Self> {
        Self::new(queue.queue, queue.surface)
    }
}

impl Drop for VkRenderer {
    fn drop(&mut self) {
        self.skia_context.free_gpu_resources();
    }
}

impl VkRenderer {
    ///Creates a renderer from a device and queues and a surface.
    pub fn new(holder: DeviceQueueHolder, surface: Arc<Surface>) -> Result<Self> {
        let device = holder.device().clone();
        let swapchain = swapchain(device.clone(), surface)?;
        let render_pass = single_pass_renderpass!(
            device.clone(),
            attachments: {
                a: {
                    format: swapchain.image_format(),
                    samples: 1,
                    load_op: DontCare,
                    store_op: Store,
                },
            },
            pass: {
                color: [a],
                depth_stencil: {},
            },
        )
        .map_err(|e| to_err(ErrorKind::Gpu, e))?;

        build_direct_context(device.clone(), holder.queue_graphics[0].clone())
            .ok_or_else(|| (ErrorKind::Gpu, "directcontext was not found").into())
            .map(|skia_context| Self {
                holder,
                swapchain,
                render_pass,
                acquire_future: Some(now(device).boxed()),
                framebuffers: Vec::new(),
                skia_context,
                width: 100,
                height: 100,
                reflesh: true,
            })
    }

    ///Resizes width and height.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.reflesh = true;
    }

    fn reflesh(&mut self) -> bool {
        if let Some(acquire_future) = self.acquire_future.as_mut() {
            acquire_future.cleanup_finished();
        }

        if !self.reflesh {
            return true;
        }
        if self.width == 0 || self.height == 0 {
            return false;
        }

        self.swapchain
            .recreate(SwapchainCreateInfo {
                image_extent: [self.width, self.height],
                ..self.swapchain.create_info()
            })
            .map(|(new_swapchain, new_images)| {
                self.swapchain = new_swapchain;
                self.framebuffers = new_images
                    .into_iter()
                    .filter_map(|image| {
                        ImageView::new_default(image)
                            .and_then(|view| {
                                Framebuffer::new(
                                    self.render_pass.clone(),
                                    FramebufferCreateInfo {
                                        attachments: vec![view],
                                        ..Default::default()
                                    },
                                )
                            })
                            .ok()
                    })
                    .collect();
                self.reflesh = false;
                ()
            })
            .is_ok()
    }

    ///Draws contents to canvas.
    pub fn draw<F>(&mut self, f: F)
    where
        F: FnOnce(skia_safe::Surface),
    {
        if self.reflesh {
            if !self.reflesh() {
                return;
            }
        }
        let (image_index, suboptimal, acquire_future) =
            match acquire_next_image(self.swapchain.clone(), None) {
                Ok(o) => o,
                Err(e) => {
                    warn!("{e}");
                    return;
                }
            };

        if suboptimal {
            self.reflesh = true;
        }

        let mut surface = match build_surface(
            &mut self.skia_context,
            &self.framebuffers[image_index as usize],
        ) {
            Some(o) => o,
            None => return,
        };

        let canvas = surface.canvas();
        canvas.reset_matrix();
        f(surface);

        self.skia_context.flush_and_submit();

        self.acquire_future = self
            .acquire_future
            .take()
            .unwrap_or_else(|| now(self.holder.device().clone()).boxed())
            .join(acquire_future)
            .then_swapchain_present(
                self.holder.queue_graphics[0].clone(),
                SwapchainPresentInfo::swapchain_image_index(self.swapchain.clone(), image_index),
            )
            .then_signal_fence_and_flush()
            .map(|f| f.boxed())
            .ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vk() {
        let context = DeviceQueueHolder::with_random_physical_device().unwrap();
        println!("{:?}\n", context.device().enabled_extensions());
        println!("{:?}\n", context.device().enabled_features());
        println!(
            "{:?}\n",
            context.device().physical_device().properties().device_type
        );
        println!("{:?}\n", context.queue_graphics());
        println!("{:?}\n", context.queue_compute());
    }
}
