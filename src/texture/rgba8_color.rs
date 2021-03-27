use crate::{renderer::*, Drawable, Rgba8Buffer};

/// An RGBA8 color texture.
pub struct TextureRgba8Color {
    desc: wgpu::TextureDescriptor<'static>,
    tex: wgpu::Texture,
}

#[derive(Clone,Copy,Debug)]
pub enum Rgba8ColorRenderPassOptions {
    Clear([f64; 4]),
    DontClear,
}

pub struct Rgba8ColorRenderPass<'a, R: LikeHeadlessRenderer> {
    pub renderer: &'a R,
    pub render_pass: wgpu::RenderPass<'a>,
    pub render_target: &'a TextureRgba8Color,
}

impl TextureRgba8Color {
    /// Construct a new RBGA8 color texture with the given size.
    pub fn new<R: LikeHeadlessRenderer>(renderer: &R, size: (u32, u32)) -> Self {
        println!("TextureRgba8Color: new");
        let desc = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::all(),
            label: None,
        };

        let tex = renderer.device().create_texture(&desc);

        TextureRgba8Color { desc, tex }
    }

    /// Get the TextureDescriptor for this texture.
    pub fn desc(&self) -> &wgpu::TextureDescriptor<'static> {
        &self.desc
    }

    /// Get the WebGPU representation of this texture.
    pub fn tex(&self) -> &wgpu::Texture {
        &self.tex
    }

    pub fn render<F, R: LikeHeadlessRenderer>(&self, renderer: &R, options: Rgba8ColorRenderPassOptions, rendering: F)
    where
        F: FnOnce(&mut Rgba8ColorRenderPass<R>),
    {
        let mut encoder =
            renderer
                .device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("mallard_color_pass"),
                });

        let texture_view = self
            .tex
            .create_view(&wgpu::TextureViewDescriptor::default());

        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: match options {
                        Rgba8ColorRenderPassOptions::DontClear => wgpu::LoadOp::Load,
                        Rgba8ColorRenderPassOptions::Clear(color) => wgpu::LoadOp::Clear(wgpu::Color {
                            r: color[0],
                            g: color[1],
                            b: color[2],
                            a: color[3],
                        }),
                    },
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        rendering(&mut Rgba8ColorRenderPass {
            renderer,
            render_pass,
            render_target: &self,
        });

        let command_buffer = encoder.finish();
        renderer.queue().submit(std::iter::once(command_buffer));
    }

    /// Make a new RGBA8 color texture containing white noise.
    pub async fn new_from_white_noise<R: LikeHeadlessRenderer>(
        renderer: &R,
        size: (u32, u32),
    ) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut data = Vec::new();

        for _i in 0..(size.0 * size.1) {
            data.push(rng.gen());
            data.push(rng.gen());
            data.push(rng.gen());
            data.push(rng.gen());
        }

        Self::new_from_rgba(renderer, size, &data).await
    }

    /// Make a new RGBA8 color texture containing the specified color data.
    pub async fn new_from_rgba<R: LikeHeadlessRenderer>(
        renderer: &R,
        size: (u32, u32),
        data: &[u8],
    ) -> Self {
        let result = Self::new(renderer, size);
        let source = Rgba8Buffer::new_source_for_image(renderer, size);

        {
            let slice = source.buf().slice(..);
            let mut view = slice.get_mapped_range_mut();
            view.copy_from_slice(data);
        }
        source.buf().unmap();

        let mut encoder = renderer
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        encoder.copy_buffer_to_texture(
            wgpu::BufferCopyView {
                buffer: &source.buf(),
                layout: wgpu::TextureDataLayout {
                    offset: 0,
                    bytes_per_row: (std::mem::size_of::<u32>() as u32) * size.0,
                    rows_per_image: size.1,
                },
            },
            wgpu::TextureCopyView {
                texture: &result.tex,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth: 1,
            },
        );

        renderer.queue().submit(std::iter::once(encoder.finish()));

        result
    }

    /// Get the RGBA8 color data out of this texture.
    pub async fn to_rgba8<R: LikeHeadlessRenderer>(&self, renderer: &R) -> Vec<u8> {
        let destination = Rgba8Buffer::new_destination_for_image(
            renderer,
            (self.desc.size.width, self.desc.size.height),
        );

        let mut encoder = renderer
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        encoder.copy_texture_to_buffer(
            wgpu::TextureCopyView {
                texture: &self.tex,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::BufferCopyView {
                buffer: &destination.buf(),
                layout: wgpu::TextureDataLayout {
                    offset: 0,
                    bytes_per_row: (std::mem::size_of::<u32>() as u32) * self.desc.size.width,
                    rows_per_image: self.desc.size.height,
                },
            },
            self.desc.size,
        );

        renderer.queue().submit(std::iter::once(encoder.finish()));

        let slice = destination.buf().slice(..);
        let mapping = slice.map_async(wgpu::MapMode::Read);
        renderer.device().poll(wgpu::Maintain::Wait);
        mapping.await.unwrap();
        let view = slice.get_mapped_range();

        let mut result = Vec::new();
        result.extend_from_slice(&view);
        result
    }

    /// Save this texture to disk (as a .png, etc, based on file name).
    pub async fn save<R: LikeHeadlessRenderer>(&self, renderer: &R, path: &str) {
        let rgba_data = self.to_rgba8(renderer).await;
        image::save_buffer(path, &rgba_data, 512, 512, image::ColorType::Rgba8).unwrap();
    }
}
