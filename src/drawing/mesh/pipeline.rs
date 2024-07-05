use crate::shaders;
use crate::window::RenderWindow;

pub struct MeshPipeline(wgpu::RenderPipeline);

impl MeshPipeline {
    pub fn set<'w>(&'w self, render_pass: &mut wgpu::RenderPass<'w>) {
        render_pass.set_pipeline(&self.0);
    }
}

impl RenderWindow<'_> {
    pub fn create_mesh_pipeline(&self) -> MeshPipeline {
        let layout = shaders::mesh::create_pipeline_layout(self.device());
        let shader = shaders::mesh::create_shader_module(self.device());

        MeshPipeline(
            self.device()
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: None,
                    layout: Some(&layout),
                    vertex: shaders::mesh::vertex_state(
                        &shader,
                        &shaders::mesh::vs_main_entry(
                            wgpu::VertexStepMode::Vertex,
                            wgpu::VertexStepMode::Instance,
                        ),
                    ),
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: shaders::mesh::ENTRY_FS_MAIN,
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                        targets: &[Some(wgpu::ColorTargetState {
                            format: self.swapchain_format(),
                            blend: Some(wgpu::BlendState {
                                color: wgpu::BlendComponent {
                                    src_factor: wgpu::BlendFactor::SrcAlpha,
                                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                                    operation: wgpu::BlendOperation::Add,
                                },
                                alpha: wgpu::BlendComponent::OVER,
                            }),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: Default::default(),
                    depth_stencil: None,
                    multisample: Default::default(),
                    multiview: None,
                }),
        )
    }
}
