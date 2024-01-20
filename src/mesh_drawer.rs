use crate::{
    shaders,
    BindBuffer,
    Instances,
    Mesh,
    INDEX_FORMAT,
};

type BufferType = shaders::mesh::Transform;

pub struct MeshDrawer {
    pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl MeshDrawer {
    pub fn new(device: &wgpu::Device, swapchain_format: wgpu::TextureFormat) -> Self {
        let pipeline_layout = shaders::mesh::create_pipeline_layout(device);

        let shader = shaders::mesh::create_shader_module(device);

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
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
                targets: &[Some(swapchain_format.into())],
            }),
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            multiview: None,
        });

        let bind_group_layout =
            shaders::mesh::bind_groups::BindGroup0::get_bind_group_layout(device);

        Self {
            pipeline,
            bind_group_layout,
        }
    }

    pub fn make_bind_buffer(
        &self,
        device: &wgpu::Device,
        value: BufferType,
    ) -> BindBuffer<BufferType> {
        BindBuffer::new(device, &self.bind_group_layout, value)
    }

    pub fn draw<'s>(
        &'s self,
        render_pass: &mut wgpu::RenderPass<'s>,
        mesh: &'s Mesh,
        instances: &'s Instances<BufferType>,
    ) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, mesh.vertex_buffer().slice(..));
        render_pass.set_vertex_buffer(1, instances.buffer().slice(..));

        instances.transform().bind(0, render_pass);

        let instances = 0..instances.len() as _;
        if let Some((index_buffer, indices_count)) = mesh.index_buffer() {
            render_pass.set_index_buffer(index_buffer.slice(..), INDEX_FORMAT);
            render_pass.draw_indexed(0..indices_count as _, 0, instances);
        } else {
            render_pass.draw(0..mesh.vertices_count() as _, instances);
        }
    }
}
