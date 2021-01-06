use futures::executor::block_on;
use mallard::*;

fn main() {
    block_on(white_noise());
}

async fn white_noise() {
    let renderer = HeadlessRenderer::new().await;
    let texture = TextureRgba8Color::new_from_white_noise(
        &renderer,
        (512, 512),
    ).await;
    texture.save(renderer, "white-noise.png").await;
}
