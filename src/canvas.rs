use crate::{buffer::Rgba8Buffer, renderer::*, texture::rgba8_color::TextureRgba8Color};

pub struct Canvas {
    size: (u32,u32),
    final_tex: TextureRgba8Color,
}

impl Canvas {
    pub fn new<R: LikeHeadlessRenderer>(renderer: R, size: (u32,u32)) -> Self {
        println!("canvas: new");
        let final_tex = TextureRgba8Color::new(renderer, size);

        Canvas {
            size,
            final_tex,
        }
    }

    // TODO: this should be moved to a Texture type
    pub async fn new_from_white_noise<R: LikeHeadlessRenderer>(renderer: R, size: (u32,u32)) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut data = Vec::new();

        for _i in 0..(size.0*size.1) {
            data.push(rng.gen());
            data.push(rng.gen());
            data.push(rng.gen());
            data.push(rng.gen());
        }

        Self::new_from_rgba(renderer, size, &data).await
    }

    // TODO: this should be moved to a Texture type
    pub async fn new_from_rgba<R: LikeHeadlessRenderer>(renderer: R, size: (u32,u32), data: &[u8]) -> Self {
        println!("new_from_rgba");
        let result = Self::new(&renderer, size);
        let source = Rgba8Buffer::new_source_for_image(&renderer, size);

        {
            let slice = source.buf().slice(..);
            println!("new_from_rgba: get_mapped_range_mut");
            let mut view = slice.get_mapped_range_mut();
            println!("new_from_rgba: copy_from_slice");
            view.copy_from_slice(data);
        }
        source.buf().unmap();

        println!("new_from_rgba: encoder");
        let mut encoder = renderer.device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: None
        });

        println!("new_from_rgba: copy buffer");
        encoder.copy_buffer_to_texture(
            wgpu::BufferCopyView {
                buffer: &source.buf(),
                layout: wgpu::TextureDataLayout {
                    offset: 0,
                    bytes_per_row: (std::mem::size_of::<u32>() as u32) * size.0,
                    rows_per_image: size.1,
                }
            },
            wgpu::TextureCopyView {
                texture: &result.final_tex.tex(),
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth: 1,
            }
        );

        println!("new_from_rgba: submit");
        renderer.queue().submit(std::iter::once(encoder.finish()));

        result
    }

    pub async fn to_rgba8<R: LikeHeadlessRenderer>(&self, renderer: R) -> Vec<u8> {
        let destination = Rgba8Buffer::new_destination_for_image(&renderer, self.size);

        let mut encoder = renderer.device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: None
        });

        encoder.copy_texture_to_buffer(
            wgpu::TextureCopyView {
                texture: self.final_tex.tex(),
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::BufferCopyView {
                buffer: &destination.buf(),
                layout: wgpu::TextureDataLayout {
                    offset: 0,
                    bytes_per_row: (std::mem::size_of::<u32>() as u32) * self.size.0,
                    rows_per_image: self.size.1,
                }
            },
            wgpu::Extent3d {
                width: self.size.0,
                height: self.size.1,
                depth: 1,
            }
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

    pub async fn save<R: LikeHeadlessRenderer>(&self, renderer: R, path: &str) {
        let rgba_data = self.to_rgba8(&renderer).await;
        image::save_buffer(path, &rgba_data, 512, 512, image::ColorType::Rgba8).unwrap();
    }
}