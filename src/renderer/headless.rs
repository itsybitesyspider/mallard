use std::sync::Arc;
pub struct HeadlessRenderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
}

pub trait LikeHeadlessRenderer {
    fn device(&self) -> &wgpu::Device;
    fn queue(&self) -> &wgpu::Queue;
}

impl LikeHeadlessRenderer for HeadlessRenderer {
    fn device(&self) -> &wgpu::Device {
        &self.device
    }

    fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }
}

impl<R> LikeHeadlessRenderer for &R
where
    R: LikeHeadlessRenderer,
{
    fn device(&self) -> &wgpu::Device {
        (*self).device()
    }

    fn queue(&self) -> &wgpu::Queue {
        (*self).queue()
    }
}

impl<R> LikeHeadlessRenderer for Arc<R>
where
    R: LikeHeadlessRenderer,
{
    fn device(&self) -> &wgpu::Device {
        self.as_ref().device()
    }

    fn queue(&self) -> &wgpu::Queue {
        self.as_ref().queue()
    }
}

impl HeadlessRenderer {
    pub async fn new() -> Self {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: None,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    shader_validation: true,
                },
                Some(&std::path::Path::new("~/wgpu_trace")),
            )
            .await
            .unwrap();

        HeadlessRenderer { device, queue }
    }
}
