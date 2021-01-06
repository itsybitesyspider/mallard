use crate::renderer::headless::*;

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
}