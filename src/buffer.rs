use crate::renderer::LikeHeadlessRenderer;

pub struct Rgba8Buffer {
    desc: wgpu::BufferDescriptor<'static>,
    buf: wgpu::Buffer,
}

impl Rgba8Buffer {
    pub(crate) fn new_destination_for_image<R: LikeHeadlessRenderer>(
        renderer: &R,
        size: (u32, u32),
    ) -> Rgba8Buffer {
        let size = ((std::mem::size_of::<u32>() as u32) * size.0 * size.1) as wgpu::BufferAddress;
        let desc = wgpu::BufferDescriptor {
            size: size,
            usage: wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::MAP_READ,
            mapped_at_creation: false,
            label: None,
        };

        let buf = renderer.device().create_buffer(&desc);

        Rgba8Buffer { desc, buf }
    }

    pub(crate) fn new_source_for_image<R: LikeHeadlessRenderer>(
        renderer: &R,
        size: (u32, u32),
    ) -> Rgba8Buffer {
        let size = ((std::mem::size_of::<u32>() as u32) * size.0 * size.1) as wgpu::BufferAddress;
        let desc = wgpu::BufferDescriptor {
            size: size,
            usage: wgpu::BufferUsage::COPY_SRC,
            mapped_at_creation: true,
            label: None,
        };

        let buf = renderer.device().create_buffer(&desc);

        Rgba8Buffer { desc, buf }
    }

    pub fn desc(&self) -> &wgpu::BufferDescriptor<'static> {
        &self.desc
    }

    pub fn buf(&self) -> &wgpu::Buffer {
        &self.buf
    }
}
