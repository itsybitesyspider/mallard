use crate::{LikeHeadlessRenderer, TextureRgba8Color};

pub trait Drawable {
    fn draw_color<R: LikeHeadlessRenderer>(&self, renderer: &R, render_target: &TextureRgba8Color);
}
