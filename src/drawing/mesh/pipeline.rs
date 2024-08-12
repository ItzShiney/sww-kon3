use crate::shaders;
use crate::window::RenderWindow;
use wgpu::BlendComponent;
use wgpu::BlendFactor;
use wgpu::BlendState;
use wgpu::ColorTargetState;
use wgpu::ColorWrites;
use wgpu::VertexStepMode;

pub struct MeshPipeline(wgpu::RenderPipeline);

impl MeshPipeline {
    pub fn new(rw: &RenderWindow) -> MeshPipeline {
        use shaders::mesh::*;

        let device = rw.device();
        let layout = create_pipeline_layout(device);
        let shader = create_shader_module(device);

        let targets = [Some(ColorTargetState {
            format: rw.swapchain_format(),
            blend: Some(BlendState {
                color: BlendComponent {
                    src_factor: BlendFactor::SrcAlpha,
                    dst_factor: BlendFactor::OneMinusSrcAlpha,
                    operation: wgpu::BlendOperation::Add,
                },
                alpha: BlendComponent::OVER,
            }),
            write_mask: ColorWrites::ALL,
        })];

        let vertex_entry = vs_main_entry(VertexStepMode::Vertex, VertexStepMode::Instance);
        let vertex = vertex_state(&shader, &vertex_entry);

        let fragment_entry = fs_main_entry(targets);
        let fragment = Some(fragment_state(&shader, &fragment_entry));

        MeshPipeline(
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Mesh"),
                layout: Some(&layout),
                vertex,
                fragment,
                primitive: Default::default(),
                depth_stencil: None,
                multisample: Default::default(),
                multiview: None,
                cache: None,
            }),
        )
    }

    pub fn set<'w>(&'w self, render_pass: &mut wgpu::RenderPass<'w>) {
        render_pass.set_pipeline(&self.0);
    }
}
