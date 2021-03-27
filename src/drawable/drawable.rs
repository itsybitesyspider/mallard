use crate::{LikeHeadlessRenderer, Rgba8ColorRenderPass};

pub trait Drawable {
    fn prepare_color<R: LikeHeadlessRenderer>(&mut self, render_pass: &Rgba8ColorRenderPass<R>);
    fn draw_color<'a, R: LikeHeadlessRenderer>(&'a self, render_pass: &mut Rgba8ColorRenderPass<'a,R>);
}
