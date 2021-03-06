use crate::{Drawable, LikeHeadlessRenderer, Position2d, Rgba8ColorRenderPass, TextureRgba8Color};

pub struct UnlitTriangles<P> {
    position: Vec<P>,
    //color: Vec<ColorRgba>,
    //tex: Vec<TexCoord>,
    //color_texture: TextureRgba8Color,

    // TODO: instead of an Option here, we need a memoized, lazy-initialized container for the RenderPipeline
    render_pipeline: Option<wgpu::RenderPipeline>,
}

impl Drawable for UnlitTriangles<Position2d> {
    fn prepare_color<R: LikeHeadlessRenderer>(&mut self, render_pass: &Rgba8ColorRenderPass<R>) {
        self.render_pipeline = Some(self.render_pipeline(render_pass.renderer, render_pass.render_target));
    }

    fn render_color<'pass, R: LikeHeadlessRenderer>(&'pass self, render_pass: &mut Rgba8ColorRenderPass<'pass,R>) {
        if let Some(render_pipeline) = self.render_pipeline.as_ref() {
            render_pass.render_pass.set_pipeline(render_pipeline);
            render_pass.render_pass.draw(0..self.position.len() as u32, 0..1);
        }
    }
}

impl UnlitTriangles<Position2d> {
    fn render_pipeline<R: LikeHeadlessRenderer>(
        &self,
        renderer: &R,
        render_target: &TextureRgba8Color,
    ) -> wgpu::RenderPipeline {
        let vs_src = include_str!("unlit.vert");
        let fs_src = include_str!("unlit.frag");
        let mut compiler = shaderc::Compiler::new().unwrap();
        let vs_spirv = compiler
            .compile_into_spirv(
                vs_src,
                shaderc::ShaderKind::Vertex,
                "unlit.vert",
                "main",
                None,
            )
            .unwrap();
        let fs_spirv = compiler
            .compile_into_spirv(
                fs_src,
                shaderc::ShaderKind::Fragment,
                "unlit.frag",
                "main",
                None,
            )
            .unwrap();
        let vs_module = renderer
            .device()
            .create_shader_module(wgpu::util::make_spirv(&vs_spirv.as_binary_u8()));
        let fs_module = renderer
            .device()
            .create_shader_module(wgpu::util::make_spirv(&fs_spirv.as_binary_u8()));

        let render_pipeline_layout =
            renderer
                .device()
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });

        renderer
            .device()
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&render_pipeline_layout),
                vertex_stage: wgpu::ProgrammableStageDescriptor {
                    module: &vs_module,
                    entry_point: "main",
                },
                fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                    module: &fs_module,
                    entry_point: "main",
                }),
                rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: wgpu::CullMode::Back,
                    depth_bias: 0,
                    depth_bias_slope_scale: 0.0,
                    depth_bias_clamp: 0.0,
                    clamp_depth: false,
                }),
                color_states: &[wgpu::ColorStateDescriptor {
                    format: render_target.desc().format,
                    color_blend: wgpu::BlendDescriptor::REPLACE,
                    alpha_blend: wgpu::BlendDescriptor::REPLACE,
                    write_mask: wgpu::ColorWrite::ALL,
                }],
                primitive_topology: wgpu::PrimitiveTopology::TriangleList,
                depth_stencil_state: None,
                vertex_state: wgpu::VertexStateDescriptor {
                    index_format: wgpu::IndexFormat::Uint16,
                    vertex_buffers: &[],
                },
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            })
    }
}
