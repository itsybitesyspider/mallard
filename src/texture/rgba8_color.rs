use crate::{Rgba8Buffer, renderer::headless::*};

pub struct TextureRgba8Color {
    desc: wgpu::TextureDescriptor<'static>,
    tex: wgpu::Texture,
}

impl TextureRgba8Color {
    pub fn new<R: LikeHeadlessRenderer>(renderer: R, size: (u32,u32)) -> Self {
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

        println!("TextureRgba8Color: create_texture");
        let tex = renderer.device().create_texture(&desc);
        println!("TextureRgba8Color: done");

        TextureRgba8Color {
            desc,
            tex,
        }
    }

    pub fn desc(&self) -> &wgpu::TextureDescriptor<'static> {
        &self.desc
    }

    pub fn tex(&self) -> &wgpu::Texture {
        &self.tex
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
                    texture: &result.tex,
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
            let destination = Rgba8Buffer::new_destination_for_image(&renderer, (self.desc.size.width,self.desc.size.height));
    
            let mut encoder = renderer.device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: None
            });
    
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
                    }
                },
                self.desc.size
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