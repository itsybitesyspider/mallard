use futures::executor::block_on;
use mallard::*;

fn main() {
    println!("creating renderer...");
    let renderer = block_on(HeadlessRenderer::new());
    println!("generating canvas...");
    let canvas = block_on(TextureRgba8Color::new_from_white_noise(&renderer, (512,512)));
    println!("writing output...");
    block_on(canvas.save(renderer, "white-noise.png"));
}