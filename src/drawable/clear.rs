use crate::{Drawable, LikeHeadlessRenderer, TextureRgba8Color};

pub struct Clear {
    pub color: [f64; 4],
}

impl Drawable for Clear {
    fn draw_color<R: LikeHeadlessRenderer>(&self, renderer: R, render_target: &TextureRgba8Color) {
        let mut encoder =
            renderer
                .device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("clear"),
                });

        let texture_view = render_target
            .tex()
            .create_view(&wgpu::TextureViewDescriptor::default());

        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: self.color[0],
                        g: self.color[1],
                        b: self.color[2],
                        a: self.color[3],
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        renderer.queue().submit(std::iter::once(encoder.finish()));
    }
}
