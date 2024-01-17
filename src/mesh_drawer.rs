use {
    crate::{
        Attributes,
        Instances,
        Mesh,
        Transform,
        Vertex,
        INDEX_FORMAT,
    },
    std::borrow::Cow,
};

pub struct MeshDrawer {
    pipeline: wgpu::RenderPipeline,
}

impl MeshDrawer {
    pub fn new(device: &wgpu::Device, swapchain_format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("mesh.wgsl"))),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    Attributes::<Vertex>::new_vertex(0).layout(),
                    Attributes::<Transform>::new_instance(2).layout(),
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(swapchain_format.into())],
            }),
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            multiview: None,
        });

        Self { pipeline }
    }

    pub fn draw<'s>(
        &'s self,
        render_pass: &mut wgpu::RenderPass<'s>,
        mesh: &'s Mesh,
        instances: &'s Instances<Transform>,
    ) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, mesh.vertex_buffer().slice(..));
        render_pass.set_vertex_buffer(1, instances.buffer().slice(..));

        let instances = 0..instances.len() as _;
        if let Some((index_buffer, indices_count)) = mesh.index_buffer() {
            render_pass.set_index_buffer(index_buffer.slice(..), INDEX_FORMAT);
            render_pass.draw_indexed(0..indices_count as _, 0, instances);
        } else {
            render_pass.draw(0..mesh.vertices_count() as _, instances);
        }
    }
}
