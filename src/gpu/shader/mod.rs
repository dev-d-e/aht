use super::*;
use getset::Getters;
use pollster::block_on;
use wgpu::{
    include_wgsl, Device, DeviceDescriptor, DownlevelFlags, Features, Instance, InstanceDescriptor,
    Limits, MemoryHints, Queue, RequestAdapterOptions, Trace,
};

#[derive(Getters)]
pub struct ComputeQueueHolder {
    #[getset(get = "pub")]
    device: Device,
    #[getset(get = "pub")]
    queue: Queue,
}

impl ComputeQueueHolder {
    pub fn new() -> Result<Self> {
        let instance = Instance::new(&InstanceDescriptor::default());

        let adapter = block_on(instance.request_adapter(&RequestAdapterOptions::default()))
            .map_err(|e| to_err(ErrorKind::Gpu, e))?;

        let o = adapter.get_downlevel_capabilities();
        if !o.flags.contains(DownlevelFlags::COMPUTE_SHADERS) {
            return Err((ErrorKind::Gpu, "compute_shaders unsupported").into());
        }

        let device_descriptor = DeviceDescriptor {
            label: None,
            required_features: Features::empty(),
            required_limits: Limits::downlevel_defaults(),
            memory_hints: MemoryHints::MemoryUsage,
            trace: Trace::Off,
            ..Default::default()
        };
        block_on(adapter.request_device(&device_descriptor))
            .map_err(|e| to_err(ErrorKind::Gpu, e))
            .map(|(device, queue)| Self { device, queue })
    }
}
