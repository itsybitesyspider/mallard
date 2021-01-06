use futures::executor::block_on;
use mallard::*;

fn main() {
    block_on(orange());
}

async fn orange() {
    let renderer = HeadlessRenderer::new().await;
    let texture = TextureRgba8Color::new(&renderer, (512, 512));
    texture.draw(&renderer, Clear { color: [1.0,0.6,0.2,1.0] });
    texture.save(renderer, "orange.png").await;
}
