use crate::{buffer::Rgba8Buffer, renderer::*, texture::rgba8_color::TextureRgba8Color};

pub struct Canvas {
    size: (u32, u32),
    final_tex: TextureRgba8Color,
}

impl Canvas {
    pub fn new<R: LikeHeadlessRenderer>(renderer: R, size: (u32, u32)) -> Self {
        println!("canvas: new");
        let final_tex = TextureRgba8Color::new(renderer, size);

        Canvas { size, final_tex }
    }
}
