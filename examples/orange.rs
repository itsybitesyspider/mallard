use futures::executor::block_on;
use mallard::*;

fn main() {
    block_on(orange());
}

async fn orange() {
    let renderer = HeadlessRenderer::new().await;
    let texture = TextureRgba8Color::new(&renderer, (512, 512));
    texture.render(&renderer, Rgba8ColorRenderPassOptions::Clear([1.0,0.4,0.1,1.0]), |_pass| {});
    texture.save(&renderer, "orange.png").await;
}
