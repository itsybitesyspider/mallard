use crate::{LikeHeadlessRenderer, Rgba8ColorRenderPass};

pub trait Drawable {
    fn draw_color<'pass, R: LikeHeadlessRenderer>(&'pass mut self, render_pass: &mut Rgba8ColorRenderPass<'pass,R>) {
        self.prepare_color(render_pass);
        self.render_color(render_pass);
    }
    fn prepare_color<R: LikeHeadlessRenderer>(&mut self, render_pass: &Rgba8ColorRenderPass<R>);
    fn render_color<'pass, R: LikeHeadlessRenderer>(&'pass self, render_pass: &mut Rgba8ColorRenderPass<'pass,R>);
}
