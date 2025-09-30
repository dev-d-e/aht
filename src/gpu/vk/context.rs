use skia_safe::gpu::backend_render_targets::make_vk;
use skia_safe::gpu::direct_contexts::make_vulkan;
use skia_safe::gpu::surfaces::wrap_backend_render_target;
use skia_safe::gpu::vk::{
    Alloc, BackendContext, Format, GetProcOf, ImageInfo, ImageLayout, ImageTiling,
};
use skia_safe::gpu::{DirectContext, SurfaceOrigin};
use skia_safe::{ColorType, Surface};
use std::sync::Arc;
use vulkano::device::{Device, Queue};
use vulkano::format::Format as VulkanoFormat;
use vulkano::render_pass::Framebuffer;
use vulkano::{Handle, VulkanObject};

pub(super) fn build_direct_context(
    device: Arc<Device>,
    queue: Arc<Queue>,
) -> Option<DirectContext> {
    let instance = device.instance();
    let library = instance.library();
    unsafe {
        let get_proc = |gpo| {
            match gpo {
                GetProcOf::Instance(i, name) => {
                    let vk_instance = ash::vk::Instance::from_raw(i as _);
                    library.get_instance_proc_addr(vk_instance, name)
                }
                GetProcOf::Device(d, name) => {
                    let get_device_proc_addr = instance.fns().v1_0.get_device_proc_addr;
                    let vk_device = ash::vk::Device::from_raw(d as _);
                    get_device_proc_addr(vk_device, name)
                }
            }
            .map(|o| o as _)
            .unwrap_or_else(|| std::ptr::null())
        };

        let b = BackendContext::new(
            instance.handle().as_raw() as _,
            device.physical_device().handle().as_raw() as _,
            device.handle().as_raw() as _,
            (
                queue.handle().as_raw() as _,
                queue.queue_family_index() as usize,
            ),
            &get_proc,
        );
        make_vulkan(&b, None)
    }
}

pub(super) fn build_surface(
    context: &mut DirectContext,
    framebuffer: &Arc<Framebuffer>,
) -> Option<Surface> {
    let image_view = &framebuffer.attachments()[0];
    let image = image_view.image().handle().as_raw();
    let (format, color_type) = to_format(image_view.format());
    let alloc = Alloc::default();
    let image_info = unsafe {
        ImageInfo::new(
            image as _,
            alloc,
            ImageTiling::OPTIMAL,
            ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            format,
            1,
            None,
            None,
            None,
            None,
        )
    };
    let [width, height] = framebuffer.extent();
    let render_target = make_vk((width as i32, height as i32), &image_info);
    wrap_backend_render_target(
        context,
        &render_target,
        SurfaceOrigin::TopLeft,
        color_type,
        None,
        None,
    )
}

#[inline]
fn to_format(f: VulkanoFormat) -> (Format, ColorType) {
    match f {
        VulkanoFormat::UNDEFINED => (Format::UNDEFINED, ColorType::Unknown),
        VulkanoFormat::R4G4_UNORM_PACK8 => (Format::R4G4_UNORM_PACK8, ColorType::ARGB4444),
        VulkanoFormat::R4G4B4A4_UNORM_PACK16 => {
            (Format::R4G4B4A4_UNORM_PACK16, ColorType::ARGB4444)
        }
        VulkanoFormat::B4G4R4A4_UNORM_PACK16 => {
            (Format::B4G4R4A4_UNORM_PACK16, ColorType::ARGB4444)
        }
        VulkanoFormat::R5G6B5_UNORM_PACK16 => (Format::R5G6B5_UNORM_PACK16, ColorType::RGB565),
        VulkanoFormat::B5G6R5_UNORM_PACK16 => (Format::B5G6R5_UNORM_PACK16, ColorType::RGB565),
        VulkanoFormat::R5G5B5A1_UNORM_PACK16 => (Format::R5G5B5A1_UNORM_PACK16, ColorType::RGB565),
        VulkanoFormat::B5G5R5A1_UNORM_PACK16 => (Format::B5G5R5A1_UNORM_PACK16, ColorType::RGB565),
        VulkanoFormat::A1R5G5B5_UNORM_PACK16 => (Format::A1R5G5B5_UNORM_PACK16, ColorType::RGB565),
        VulkanoFormat::A1B5G5R5_UNORM_PACK16 => {
            (Format::A1B5G5R5_UNORM_PACK16_KHR, ColorType::RGB565)
        }
        VulkanoFormat::A8_UNORM => (Format::A8_UNORM_KHR, ColorType::Alpha8),
        VulkanoFormat::R8_UNORM => (Format::R8_UNORM, ColorType::R8UNorm),
        VulkanoFormat::R8_SNORM => (Format::R8_SNORM, ColorType::R8UNorm),
        VulkanoFormat::R8_USCALED => (Format::R8_USCALED, ColorType::R8UNorm),
        VulkanoFormat::R8_SSCALED => (Format::R8_SSCALED, ColorType::R8UNorm),
        VulkanoFormat::R8_UINT => (Format::R8_UINT, ColorType::R8UNorm),
        VulkanoFormat::R8_SINT => (Format::R8_SINT, ColorType::R8UNorm),
        VulkanoFormat::R8_SRGB => (Format::R8_SRGB, ColorType::R8UNorm),
        VulkanoFormat::R8G8_UNORM => (Format::R8G8_UNORM, ColorType::R8G8UNorm),
        VulkanoFormat::R8G8_SNORM => (Format::R8G8_SNORM, ColorType::R8G8UNorm),
        VulkanoFormat::R8G8_USCALED => (Format::R8G8_USCALED, ColorType::R8G8UNorm),
        VulkanoFormat::R8G8_SSCALED => (Format::R8G8_SSCALED, ColorType::R8G8UNorm),
        VulkanoFormat::R8G8_UINT => (Format::R8G8_UINT, ColorType::R8G8UNorm),
        VulkanoFormat::R8G8_SINT => (Format::R8G8_SINT, ColorType::R8G8UNorm),
        VulkanoFormat::R8G8_SRGB => (Format::R8G8_SRGB, ColorType::R8G8UNorm),
        VulkanoFormat::R8G8B8_UNORM => (Format::R8G8B8_UNORM, ColorType::RGB888x),
        VulkanoFormat::R8G8B8_SNORM => (Format::R8G8B8_SNORM, ColorType::RGB888x),
        VulkanoFormat::R8G8B8_USCALED => (Format::R8G8B8_USCALED, ColorType::RGB888x),
        VulkanoFormat::R8G8B8_SSCALED => (Format::R8G8B8_SSCALED, ColorType::RGB888x),
        VulkanoFormat::R8G8B8_UINT => (Format::R8G8B8_UINT, ColorType::RGB888x),
        VulkanoFormat::R8G8B8_SINT => (Format::R8G8B8_SINT, ColorType::RGB888x),
        VulkanoFormat::R8G8B8_SRGB => (Format::R8G8B8_SRGB, ColorType::RGB888x),
        VulkanoFormat::B8G8R8_UNORM => (Format::B8G8R8_UNORM, ColorType::Unknown),
        VulkanoFormat::B8G8R8_SNORM => (Format::B8G8R8_SNORM, ColorType::Unknown),
        VulkanoFormat::B8G8R8_USCALED => (Format::B8G8R8_USCALED, ColorType::Unknown),
        VulkanoFormat::B8G8R8_SSCALED => (Format::B8G8R8_SSCALED, ColorType::Unknown),
        VulkanoFormat::B8G8R8_UINT => (Format::B8G8R8_UINT, ColorType::Unknown),
        VulkanoFormat::B8G8R8_SINT => (Format::B8G8R8_SINT, ColorType::Unknown),
        VulkanoFormat::B8G8R8_SRGB => (Format::B8G8R8_SRGB, ColorType::Unknown),
        VulkanoFormat::R8G8B8A8_UNORM => (Format::R8G8B8A8_UNORM, ColorType::RGBA8888),
        VulkanoFormat::R8G8B8A8_SNORM => (Format::R8G8B8A8_SNORM, ColorType::RGBA8888),
        VulkanoFormat::R8G8B8A8_USCALED => (Format::R8G8B8A8_USCALED, ColorType::RGBA8888),
        VulkanoFormat::R8G8B8A8_SSCALED => (Format::R8G8B8A8_SSCALED, ColorType::RGBA8888),
        VulkanoFormat::R8G8B8A8_UINT => (Format::R8G8B8A8_UINT, ColorType::RGBA8888),
        VulkanoFormat::R8G8B8A8_SINT => (Format::R8G8B8A8_SINT, ColorType::RGBA8888),
        VulkanoFormat::R8G8B8A8_SRGB => (Format::R8G8B8A8_SRGB, ColorType::SRGBA8888),
        VulkanoFormat::B8G8R8A8_UNORM => (Format::B8G8R8A8_UNORM, ColorType::BGRA8888),
        VulkanoFormat::B8G8R8A8_SNORM => (Format::B8G8R8A8_SNORM, ColorType::BGRA8888),
        VulkanoFormat::B8G8R8A8_USCALED => (Format::B8G8R8A8_USCALED, ColorType::BGRA8888),
        VulkanoFormat::B8G8R8A8_SSCALED => (Format::B8G8R8A8_SSCALED, ColorType::BGRA8888),
        VulkanoFormat::B8G8R8A8_UINT => (Format::B8G8R8A8_UINT, ColorType::BGRA8888),
        VulkanoFormat::B8G8R8A8_SINT => (Format::B8G8R8A8_SINT, ColorType::BGRA8888),
        VulkanoFormat::B8G8R8A8_SRGB => (Format::B8G8R8A8_SRGB, ColorType::BGRA8888),
        VulkanoFormat::A8B8G8R8_UNORM_PACK32 => {
            (Format::A8B8G8R8_UNORM_PACK32, ColorType::BGRA8888)
        }
        VulkanoFormat::A8B8G8R8_SNORM_PACK32 => {
            (Format::A8B8G8R8_SNORM_PACK32, ColorType::BGRA8888)
        }
        VulkanoFormat::A8B8G8R8_USCALED_PACK32 => {
            (Format::A8B8G8R8_USCALED_PACK32, ColorType::BGRA8888)
        }
        VulkanoFormat::A8B8G8R8_SSCALED_PACK32 => {
            (Format::A8B8G8R8_SSCALED_PACK32, ColorType::BGRA8888)
        }
        VulkanoFormat::A8B8G8R8_UINT_PACK32 => (Format::A8B8G8R8_UINT_PACK32, ColorType::BGRA8888),
        VulkanoFormat::A8B8G8R8_SINT_PACK32 => (Format::A8B8G8R8_SINT_PACK32, ColorType::BGRA8888),
        VulkanoFormat::A8B8G8R8_SRGB_PACK32 => (Format::A8B8G8R8_SRGB_PACK32, ColorType::BGRA8888),
        VulkanoFormat::A2R10G10B10_UNORM_PACK32 => {
            (Format::A2R10G10B10_UNORM_PACK32, ColorType::RGBA1010102)
        }
        VulkanoFormat::A2R10G10B10_SNORM_PACK32 => {
            (Format::A2R10G10B10_SNORM_PACK32, ColorType::RGBA1010102)
        }
        VulkanoFormat::A2R10G10B10_USCALED_PACK32 => {
            (Format::A2R10G10B10_USCALED_PACK32, ColorType::RGBA1010102)
        }
        VulkanoFormat::A2R10G10B10_SSCALED_PACK32 => {
            (Format::A2R10G10B10_SSCALED_PACK32, ColorType::RGBA1010102)
        }
        VulkanoFormat::A2R10G10B10_UINT_PACK32 => {
            (Format::A2R10G10B10_UINT_PACK32, ColorType::RGBA1010102)
        }
        VulkanoFormat::A2R10G10B10_SINT_PACK32 => {
            (Format::A2R10G10B10_SINT_PACK32, ColorType::RGBA1010102)
        }
        VulkanoFormat::A2B10G10R10_UNORM_PACK32 => {
            (Format::A2B10G10R10_UNORM_PACK32, ColorType::BGRA1010102)
        }
        VulkanoFormat::A2B10G10R10_SNORM_PACK32 => {
            (Format::A2B10G10R10_SNORM_PACK32, ColorType::BGRA1010102)
        }
        VulkanoFormat::A2B10G10R10_USCALED_PACK32 => {
            (Format::A2B10G10R10_USCALED_PACK32, ColorType::BGRA1010102)
        }
        VulkanoFormat::A2B10G10R10_SSCALED_PACK32 => {
            (Format::A2B10G10R10_SSCALED_PACK32, ColorType::BGRA1010102)
        }
        VulkanoFormat::A2B10G10R10_UINT_PACK32 => {
            (Format::A2B10G10R10_UINT_PACK32, ColorType::BGRA1010102)
        }
        VulkanoFormat::A2B10G10R10_SINT_PACK32 => {
            (Format::A2B10G10R10_SINT_PACK32, ColorType::BGRA1010102)
        }
        VulkanoFormat::R16_UNORM => (Format::R16_UNORM, ColorType::Unknown),
        VulkanoFormat::R16_SNORM => (Format::R16_SNORM, ColorType::Unknown),
        VulkanoFormat::R16_USCALED => (Format::R16_USCALED, ColorType::Unknown),
        VulkanoFormat::R16_SSCALED => (Format::R16_SSCALED, ColorType::Unknown),
        VulkanoFormat::R16_UINT => (Format::R16_UINT, ColorType::Unknown),
        VulkanoFormat::R16_SINT => (Format::R16_SINT, ColorType::Unknown),
        VulkanoFormat::R16_SFLOAT => (Format::R16_SFLOAT, ColorType::Unknown),
        VulkanoFormat::R16G16_UNORM => (Format::R16G16_UNORM, ColorType::R16G16UNorm),
        VulkanoFormat::R16G16_SNORM => (Format::R16G16_SNORM, ColorType::R16G16UNorm),
        VulkanoFormat::R16G16_USCALED => (Format::R16G16_USCALED, ColorType::R16G16UNorm),
        VulkanoFormat::R16G16_SSCALED => (Format::R16G16_SSCALED, ColorType::R16G16UNorm),
        VulkanoFormat::R16G16_UINT => (Format::R16G16_UINT, ColorType::R16G16UNorm),
        VulkanoFormat::R16G16_SINT => (Format::R16G16_SINT, ColorType::R16G16UNorm),
        VulkanoFormat::R16G16_SFLOAT => (Format::R16G16_SFLOAT, ColorType::R16G16Float),
        VulkanoFormat::R16G16B16_UNORM => (Format::R16G16B16_UNORM, ColorType::Unknown),
        VulkanoFormat::R16G16B16_SNORM => (Format::R16G16B16_SNORM, ColorType::Unknown),
        VulkanoFormat::R16G16B16_USCALED => (Format::R16G16B16_USCALED, ColorType::Unknown),
        VulkanoFormat::R16G16B16_SSCALED => (Format::R16G16B16_SSCALED, ColorType::Unknown),
        VulkanoFormat::R16G16B16_UINT => (Format::R16G16B16_UINT, ColorType::Unknown),
        VulkanoFormat::R16G16B16_SINT => (Format::R16G16B16_SINT, ColorType::Unknown),
        VulkanoFormat::R16G16B16_SFLOAT => (Format::R16G16B16_SFLOAT, ColorType::RGBF16F16F16x),
        VulkanoFormat::R16G16B16A16_UNORM => {
            (Format::R16G16B16A16_UNORM, ColorType::R16G16B16A16UNorm)
        }
        VulkanoFormat::R16G16B16A16_SNORM => {
            (Format::R16G16B16A16_SNORM, ColorType::R16G16B16A16UNorm)
        }
        VulkanoFormat::R16G16B16A16_USCALED => (Format::R16G16B16A16_USCALED, ColorType::Unknown),
        VulkanoFormat::R16G16B16A16_SSCALED => (Format::R16G16B16A16_SSCALED, ColorType::Unknown),
        VulkanoFormat::R16G16B16A16_UINT => {
            (Format::R16G16B16A16_UINT, ColorType::R16G16B16A16UNorm)
        }
        VulkanoFormat::R16G16B16A16_SINT => {
            (Format::R16G16B16A16_SINT, ColorType::R16G16B16A16UNorm)
        }
        VulkanoFormat::R16G16B16A16_SFLOAT => (Format::R16G16B16A16_SFLOAT, ColorType::RGBAF16),
        VulkanoFormat::R32_UINT => (Format::R32_UINT, ColorType::Unknown),
        VulkanoFormat::R32_SINT => (Format::R32_SINT, ColorType::Unknown),
        VulkanoFormat::R32_SFLOAT => (Format::R32_SFLOAT, ColorType::Unknown),
        VulkanoFormat::R32G32_UINT => (Format::R32G32_UINT, ColorType::Unknown),
        VulkanoFormat::R32G32_SINT => (Format::R32G32_SINT, ColorType::Unknown),
        VulkanoFormat::R32G32_SFLOAT => (Format::R32G32_SFLOAT, ColorType::Unknown),
        VulkanoFormat::R32G32B32_UINT => (Format::R32G32B32_UINT, ColorType::Unknown),
        VulkanoFormat::R32G32B32_SINT => (Format::R32G32B32_SINT, ColorType::Unknown),
        VulkanoFormat::R32G32B32_SFLOAT => (Format::R32G32B32_SFLOAT, ColorType::Unknown),
        VulkanoFormat::R32G32B32A32_UINT => (Format::R32G32B32A32_UINT, ColorType::Unknown),
        VulkanoFormat::R32G32B32A32_SINT => (Format::R32G32B32A32_SINT, ColorType::Unknown),
        VulkanoFormat::R32G32B32A32_SFLOAT => (Format::R32G32B32A32_SFLOAT, ColorType::RGBAF32),
        VulkanoFormat::R64_UINT => (Format::R64_UINT, ColorType::Unknown),
        VulkanoFormat::R64_SINT => (Format::R64_SINT, ColorType::Unknown),
        VulkanoFormat::R64_SFLOAT => (Format::R64_SFLOAT, ColorType::Unknown),
        VulkanoFormat::R64G64_UINT => (Format::R64G64_UINT, ColorType::Unknown),
        VulkanoFormat::R64G64_SINT => (Format::R64G64_SINT, ColorType::Unknown),
        VulkanoFormat::R64G64_SFLOAT => (Format::R64G64_SFLOAT, ColorType::Unknown),
        VulkanoFormat::R64G64B64_UINT => (Format::R64G64B64_UINT, ColorType::Unknown),
        VulkanoFormat::R64G64B64_SINT => (Format::R64G64B64_SINT, ColorType::Unknown),
        VulkanoFormat::R64G64B64_SFLOAT => (Format::R64G64B64_SFLOAT, ColorType::Unknown),
        VulkanoFormat::R64G64B64A64_UINT => (Format::R64G64B64A64_UINT, ColorType::Unknown),
        VulkanoFormat::R64G64B64A64_SINT => (Format::R64G64B64A64_SINT, ColorType::Unknown),
        VulkanoFormat::R64G64B64A64_SFLOAT => (Format::R64G64B64A64_SFLOAT, ColorType::Unknown),
        VulkanoFormat::B10G11R11_UFLOAT_PACK32 => {
            (Format::B10G11R11_UFLOAT_PACK32, ColorType::Unknown)
        }
        VulkanoFormat::E5B9G9R9_UFLOAT_PACK32 => {
            (Format::E5B9G9R9_UFLOAT_PACK32, ColorType::Unknown)
        }
        VulkanoFormat::D16_UNORM => (Format::D16_UNORM, ColorType::Unknown),
        VulkanoFormat::X8_D24_UNORM_PACK32 => (Format::X8_D24_UNORM_PACK32, ColorType::Unknown),
        VulkanoFormat::D32_SFLOAT => (Format::D32_SFLOAT, ColorType::Unknown),
        VulkanoFormat::S8_UINT => (Format::S8_UINT, ColorType::Unknown),
        VulkanoFormat::D16_UNORM_S8_UINT => (Format::D16_UNORM_S8_UINT, ColorType::Unknown),
        VulkanoFormat::D24_UNORM_S8_UINT => (Format::D24_UNORM_S8_UINT, ColorType::Unknown),
        VulkanoFormat::D32_SFLOAT_S8_UINT => (Format::D32_SFLOAT_S8_UINT, ColorType::Unknown),
        VulkanoFormat::BC1_RGB_UNORM_BLOCK => (Format::BC1_RGB_UNORM_BLOCK, ColorType::Unknown),
        VulkanoFormat::BC1_RGB_SRGB_BLOCK => (Format::BC1_RGB_SRGB_BLOCK, ColorType::Unknown),
        VulkanoFormat::BC1_RGBA_UNORM_BLOCK => (Format::BC1_RGBA_UNORM_BLOCK, ColorType::Unknown),
        VulkanoFormat::BC1_RGBA_SRGB_BLOCK => (Format::BC1_RGBA_SRGB_BLOCK, ColorType::Unknown),
        VulkanoFormat::BC2_UNORM_BLOCK => (Format::BC2_UNORM_BLOCK, ColorType::Unknown),
        VulkanoFormat::BC2_SRGB_BLOCK => (Format::BC2_SRGB_BLOCK, ColorType::Unknown),
        VulkanoFormat::BC3_UNORM_BLOCK => (Format::BC3_UNORM_BLOCK, ColorType::Unknown),
        VulkanoFormat::BC3_SRGB_BLOCK => (Format::BC3_SRGB_BLOCK, ColorType::Unknown),
        VulkanoFormat::BC4_UNORM_BLOCK => (Format::BC4_UNORM_BLOCK, ColorType::Unknown),
        VulkanoFormat::BC4_SNORM_BLOCK => (Format::BC4_SNORM_BLOCK, ColorType::Unknown),
        VulkanoFormat::BC5_UNORM_BLOCK => (Format::BC5_UNORM_BLOCK, ColorType::Unknown),
        VulkanoFormat::BC5_SNORM_BLOCK => (Format::BC5_SNORM_BLOCK, ColorType::Unknown),
        VulkanoFormat::BC6H_UFLOAT_BLOCK => (Format::BC6H_UFLOAT_BLOCK, ColorType::Unknown),
        VulkanoFormat::BC6H_SFLOAT_BLOCK => (Format::BC6H_SFLOAT_BLOCK, ColorType::Unknown),
        VulkanoFormat::BC7_UNORM_BLOCK => (Format::BC7_UNORM_BLOCK, ColorType::Unknown),
        VulkanoFormat::BC7_SRGB_BLOCK => (Format::BC7_SRGB_BLOCK, ColorType::Unknown),
        VulkanoFormat::ETC2_R8G8B8_UNORM_BLOCK => {
            (Format::ETC2_R8G8B8_UNORM_BLOCK, ColorType::RGB888x)
        }
        VulkanoFormat::ETC2_R8G8B8_SRGB_BLOCK => {
            (Format::ETC2_R8G8B8_SRGB_BLOCK, ColorType::RGB888x)
        }
        VulkanoFormat::ETC2_R8G8B8A1_UNORM_BLOCK => {
            (Format::ETC2_R8G8B8A1_UNORM_BLOCK, ColorType::RGB888x)
        }
        VulkanoFormat::ETC2_R8G8B8A1_SRGB_BLOCK => {
            (Format::ETC2_R8G8B8A1_SRGB_BLOCK, ColorType::RGB888x)
        }
        VulkanoFormat::ETC2_R8G8B8A8_UNORM_BLOCK => {
            (Format::ETC2_R8G8B8A8_UNORM_BLOCK, ColorType::RGBA8888)
        }
        VulkanoFormat::ETC2_R8G8B8A8_SRGB_BLOCK => {
            (Format::ETC2_R8G8B8A8_SRGB_BLOCK, ColorType::SRGBA8888)
        }
        VulkanoFormat::EAC_R11_UNORM_BLOCK => (Format::EAC_R11_UNORM_BLOCK, ColorType::Unknown),
        VulkanoFormat::EAC_R11_SNORM_BLOCK => (Format::EAC_R11_SNORM_BLOCK, ColorType::Unknown),
        VulkanoFormat::EAC_R11G11_UNORM_BLOCK => {
            (Format::EAC_R11G11_UNORM_BLOCK, ColorType::Unknown)
        }
        VulkanoFormat::EAC_R11G11_SNORM_BLOCK => {
            (Format::EAC_R11G11_SNORM_BLOCK, ColorType::Unknown)
        }
        VulkanoFormat::ASTC_4x4_UNORM_BLOCK => (Format::ASTC_4x4_UNORM_BLOCK, ColorType::Unknown),
        VulkanoFormat::ASTC_4x4_SRGB_BLOCK => (Format::ASTC_4x4_SRGB_BLOCK, ColorType::Unknown),
        VulkanoFormat::ASTC_5x4_UNORM_BLOCK => (Format::ASTC_5x4_UNORM_BLOCK, ColorType::Unknown),
        VulkanoFormat::ASTC_5x4_SRGB_BLOCK => (Format::ASTC_5x4_SRGB_BLOCK, ColorType::Unknown),
        VulkanoFormat::ASTC_5x5_UNORM_BLOCK => (Format::ASTC_5x5_UNORM_BLOCK, ColorType::Unknown),
        VulkanoFormat::ASTC_5x5_SRGB_BLOCK => (Format::ASTC_5x5_SRGB_BLOCK, ColorType::Unknown),
        VulkanoFormat::ASTC_6x5_UNORM_BLOCK => (Format::ASTC_6x5_UNORM_BLOCK, ColorType::Unknown),
        VulkanoFormat::ASTC_6x5_SRGB_BLOCK => (Format::ASTC_6x5_SRGB_BLOCK, ColorType::Unknown),
        VulkanoFormat::ASTC_6x6_UNORM_BLOCK => (Format::ASTC_6x6_UNORM_BLOCK, ColorType::Unknown),
        VulkanoFormat::ASTC_6x6_SRGB_BLOCK => (Format::ASTC_6x6_SRGB_BLOCK, ColorType::Unknown),
        VulkanoFormat::ASTC_8x5_UNORM_BLOCK => (Format::ASTC_8x5_UNORM_BLOCK, ColorType::Unknown),
        VulkanoFormat::ASTC_8x5_SRGB_BLOCK => (Format::ASTC_8x5_SRGB_BLOCK, ColorType::Unknown),
        VulkanoFormat::ASTC_8x6_UNORM_BLOCK => (Format::ASTC_8x6_UNORM_BLOCK, ColorType::Unknown),
        VulkanoFormat::ASTC_8x6_SRGB_BLOCK => (Format::ASTC_8x6_SRGB_BLOCK, ColorType::Unknown),
        VulkanoFormat::ASTC_8x8_UNORM_BLOCK => (Format::ASTC_8x8_UNORM_BLOCK, ColorType::Unknown),
        VulkanoFormat::ASTC_8x8_SRGB_BLOCK => (Format::ASTC_8x8_SRGB_BLOCK, ColorType::Unknown),
        VulkanoFormat::ASTC_10x5_UNORM_BLOCK => (Format::ASTC_10x5_UNORM_BLOCK, ColorType::Unknown),
        VulkanoFormat::ASTC_10x5_SRGB_BLOCK => (Format::ASTC_10x5_SRGB_BLOCK, ColorType::Unknown),
        VulkanoFormat::ASTC_10x6_UNORM_BLOCK => (Format::ASTC_10x6_UNORM_BLOCK, ColorType::Unknown),
        VulkanoFormat::ASTC_10x6_SRGB_BLOCK => (Format::ASTC_10x6_SRGB_BLOCK, ColorType::Unknown),
        VulkanoFormat::ASTC_10x8_UNORM_BLOCK => (Format::ASTC_10x8_UNORM_BLOCK, ColorType::Unknown),
        VulkanoFormat::ASTC_10x8_SRGB_BLOCK => (Format::ASTC_10x8_SRGB_BLOCK, ColorType::Unknown),
        VulkanoFormat::ASTC_10x10_UNORM_BLOCK => {
            (Format::ASTC_10x10_UNORM_BLOCK, ColorType::Unknown)
        }
        VulkanoFormat::ASTC_10x10_SRGB_BLOCK => (Format::ASTC_10x10_SRGB_BLOCK, ColorType::Unknown),
        VulkanoFormat::ASTC_12x10_UNORM_BLOCK => {
            (Format::ASTC_12x10_UNORM_BLOCK, ColorType::Unknown)
        }
        VulkanoFormat::ASTC_12x10_SRGB_BLOCK => (Format::ASTC_12x10_SRGB_BLOCK, ColorType::Unknown),
        VulkanoFormat::ASTC_12x12_UNORM_BLOCK => {
            (Format::ASTC_12x12_UNORM_BLOCK, ColorType::Unknown)
        }
        VulkanoFormat::ASTC_12x12_SRGB_BLOCK => (Format::ASTC_12x12_SRGB_BLOCK, ColorType::Unknown),
        VulkanoFormat::G8B8G8R8_422_UNORM => (Format::G8B8G8R8_422_UNORM, ColorType::Unknown),
        VulkanoFormat::B8G8R8G8_422_UNORM => (Format::B8G8R8G8_422_UNORM, ColorType::Unknown),
        VulkanoFormat::G8_B8_R8_3PLANE_420_UNORM => {
            (Format::G8_B8_R8_3PLANE_420_UNORM, ColorType::Unknown)
        }
        VulkanoFormat::G8_B8R8_2PLANE_420_UNORM => {
            (Format::G8_B8R8_2PLANE_420_UNORM, ColorType::Unknown)
        }
        VulkanoFormat::G8_B8_R8_3PLANE_422_UNORM => {
            (Format::G8_B8_R8_3PLANE_422_UNORM, ColorType::Unknown)
        }
        VulkanoFormat::G8_B8R8_2PLANE_422_UNORM => {
            (Format::G8_B8R8_2PLANE_422_UNORM, ColorType::Unknown)
        }
        VulkanoFormat::G8_B8_R8_3PLANE_444_UNORM => {
            (Format::G8_B8_R8_3PLANE_444_UNORM, ColorType::Unknown)
        }
        VulkanoFormat::R10X6_UNORM_PACK16 => (Format::R10X6_UNORM_PACK16, ColorType::Unknown),
        VulkanoFormat::R10X6G10X6_UNORM_2PACK16 => {
            (Format::R10X6G10X6_UNORM_2PACK16, ColorType::Unknown)
        }
        VulkanoFormat::R10X6G10X6B10X6A10X6_UNORM_4PACK16 => (
            Format::R10X6G10X6B10X6A10X6_UNORM_4PACK16,
            ColorType::Unknown,
        ),
        VulkanoFormat::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16 => (
            Format::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16,
            ColorType::Unknown,
        ),
        VulkanoFormat::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16 => (
            Format::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16,
            ColorType::Unknown,
        ),
        VulkanoFormat::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16 => (
            Format::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16,
            ColorType::Unknown,
        ),
        VulkanoFormat::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16 => (
            Format::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16,
            ColorType::Unknown,
        ),
        VulkanoFormat::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16 => (
            Format::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16,
            ColorType::Unknown,
        ),
        VulkanoFormat::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16 => (
            Format::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16,
            ColorType::Unknown,
        ),
        VulkanoFormat::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16 => (
            Format::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16,
            ColorType::Unknown,
        ),
        VulkanoFormat::R12X4_UNORM_PACK16 => (Format::R12X4_UNORM_PACK16, ColorType::Unknown),
        VulkanoFormat::R12X4G12X4_UNORM_2PACK16 => {
            (Format::R12X4G12X4_UNORM_2PACK16, ColorType::Unknown)
        }
        VulkanoFormat::R12X4G12X4B12X4A12X4_UNORM_4PACK16 => (
            Format::R12X4G12X4B12X4A12X4_UNORM_4PACK16,
            ColorType::Unknown,
        ),
        VulkanoFormat::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16 => (
            Format::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16,
            ColorType::Unknown,
        ),
        VulkanoFormat::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16 => (
            Format::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16,
            ColorType::Unknown,
        ),
        VulkanoFormat::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16 => (
            Format::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16,
            ColorType::Unknown,
        ),
        VulkanoFormat::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16 => (
            Format::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16,
            ColorType::Unknown,
        ),
        VulkanoFormat::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16 => (
            Format::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16,
            ColorType::Unknown,
        ),
        VulkanoFormat::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16 => (
            Format::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16,
            ColorType::Unknown,
        ),
        VulkanoFormat::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16 => (
            Format::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16,
            ColorType::Unknown,
        ),
        VulkanoFormat::G16B16G16R16_422_UNORM => {
            (Format::G16B16G16R16_422_UNORM, ColorType::Unknown)
        }
        VulkanoFormat::B16G16R16G16_422_UNORM => {
            (Format::B16G16R16G16_422_UNORM, ColorType::Unknown)
        }
        VulkanoFormat::G16_B16_R16_3PLANE_420_UNORM => {
            (Format::G16_B16_R16_3PLANE_420_UNORM, ColorType::Unknown)
        }
        VulkanoFormat::G16_B16R16_2PLANE_420_UNORM => {
            (Format::G16_B16R16_2PLANE_420_UNORM, ColorType::Unknown)
        }
        VulkanoFormat::G16_B16_R16_3PLANE_422_UNORM => {
            (Format::G16_B16_R16_3PLANE_422_UNORM, ColorType::Unknown)
        }
        VulkanoFormat::G16_B16R16_2PLANE_422_UNORM => {
            (Format::G16_B16R16_2PLANE_422_UNORM, ColorType::Unknown)
        }
        VulkanoFormat::G16_B16_R16_3PLANE_444_UNORM => {
            (Format::G16_B16_R16_3PLANE_444_UNORM, ColorType::Unknown)
        }
        VulkanoFormat::PVRTC1_2BPP_UNORM_BLOCK => {
            (Format::PVRTC1_2BPP_UNORM_BLOCK_IMG, ColorType::Unknown)
        }
        VulkanoFormat::PVRTC1_4BPP_UNORM_BLOCK => {
            (Format::PVRTC1_4BPP_UNORM_BLOCK_IMG, ColorType::Unknown)
        }
        VulkanoFormat::PVRTC2_2BPP_UNORM_BLOCK => {
            (Format::PVRTC2_2BPP_UNORM_BLOCK_IMG, ColorType::Unknown)
        }
        VulkanoFormat::PVRTC2_4BPP_UNORM_BLOCK => {
            (Format::PVRTC2_4BPP_UNORM_BLOCK_IMG, ColorType::Unknown)
        }
        VulkanoFormat::PVRTC1_2BPP_SRGB_BLOCK => {
            (Format::PVRTC1_2BPP_SRGB_BLOCK_IMG, ColorType::Unknown)
        }
        VulkanoFormat::PVRTC1_4BPP_SRGB_BLOCK => {
            (Format::PVRTC1_4BPP_SRGB_BLOCK_IMG, ColorType::Unknown)
        }
        VulkanoFormat::PVRTC2_2BPP_SRGB_BLOCK => {
            (Format::PVRTC2_2BPP_SRGB_BLOCK_IMG, ColorType::Unknown)
        }
        VulkanoFormat::PVRTC2_4BPP_SRGB_BLOCK => {
            (Format::PVRTC2_4BPP_SRGB_BLOCK_IMG, ColorType::Unknown)
        }
        VulkanoFormat::ASTC_4x4_SFLOAT_BLOCK => (Format::ASTC_4x4_SFLOAT_BLOCK, ColorType::Unknown),
        VulkanoFormat::ASTC_5x4_SFLOAT_BLOCK => (Format::ASTC_5x4_SFLOAT_BLOCK, ColorType::Unknown),
        VulkanoFormat::ASTC_5x5_SFLOAT_BLOCK => (Format::ASTC_5x5_SFLOAT_BLOCK, ColorType::Unknown),
        VulkanoFormat::ASTC_6x5_SFLOAT_BLOCK => (Format::ASTC_6x5_SFLOAT_BLOCK, ColorType::Unknown),
        VulkanoFormat::ASTC_6x6_SFLOAT_BLOCK => (Format::ASTC_6x6_SFLOAT_BLOCK, ColorType::Unknown),
        VulkanoFormat::ASTC_8x5_SFLOAT_BLOCK => (Format::ASTC_8x5_SFLOAT_BLOCK, ColorType::Unknown),
        VulkanoFormat::ASTC_8x6_SFLOAT_BLOCK => (Format::ASTC_8x6_SFLOAT_BLOCK, ColorType::Unknown),
        VulkanoFormat::ASTC_8x8_SFLOAT_BLOCK => (Format::ASTC_8x8_SFLOAT_BLOCK, ColorType::Unknown),
        VulkanoFormat::ASTC_10x5_SFLOAT_BLOCK => {
            (Format::ASTC_10x5_SFLOAT_BLOCK, ColorType::Unknown)
        }
        VulkanoFormat::ASTC_10x6_SFLOAT_BLOCK => {
            (Format::ASTC_10x6_SFLOAT_BLOCK, ColorType::Unknown)
        }
        VulkanoFormat::ASTC_10x8_SFLOAT_BLOCK => {
            (Format::ASTC_10x8_SFLOAT_BLOCK, ColorType::Unknown)
        }
        VulkanoFormat::ASTC_10x10_SFLOAT_BLOCK => {
            (Format::ASTC_10x10_SFLOAT_BLOCK, ColorType::Unknown)
        }
        VulkanoFormat::ASTC_12x10_SFLOAT_BLOCK => {
            (Format::ASTC_12x10_SFLOAT_BLOCK, ColorType::Unknown)
        }
        VulkanoFormat::ASTC_12x12_SFLOAT_BLOCK => {
            (Format::ASTC_12x12_SFLOAT_BLOCK, ColorType::Unknown)
        }
        VulkanoFormat::G8_B8R8_2PLANE_444_UNORM => {
            (Format::G8_B8R8_2PLANE_444_UNORM, ColorType::Unknown)
        }
        VulkanoFormat::G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16 => (
            Format::G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16,
            ColorType::Unknown,
        ),
        VulkanoFormat::G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16 => (
            Format::G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16,
            ColorType::Unknown,
        ),
        VulkanoFormat::G16_B16R16_2PLANE_444_UNORM => {
            (Format::G16_B16R16_2PLANE_444_UNORM, ColorType::Unknown)
        }
        VulkanoFormat::A4R4G4B4_UNORM_PACK16 => {
            (Format::A4R4G4B4_UNORM_PACK16, ColorType::ARGB4444)
        }
        VulkanoFormat::A4B4G4R4_UNORM_PACK16 => {
            (Format::A4B4G4R4_UNORM_PACK16, ColorType::ARGB4444)
        }
        VulkanoFormat::R16G16_S10_5_NV => (Format::R16G16_S10_5_NV, ColorType::R16G16Float),
        _ => (Format::UNDEFINED, ColorType::Unknown),
    }
}
