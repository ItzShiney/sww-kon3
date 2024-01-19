use {
    crate::{
        shaders,
        BindBuffer,
        Instances,
        Mesh,
        INDEX_FORMAT,
    },
    std::borrow::Cow,
};

type BufferType = shaders::mesh::Transform;

pub struct MeshDrawer {
    pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl MeshDrawer {
    pub fn new(device: &wgpu::Device, swapchain_format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("mesh.wgsl"))),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: Some(BindBuffer::<BufferType>::SIZE_NONZERO),
                },
                count: None,
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    shaders::mesh::InVertex::vertex_buffer_layout(wgpu::VertexStepMode::Vertex),
                    shaders::mesh::InTransform::vertex_buffer_layout(
                        wgpu::VertexStepMode::Instance,
                    ),
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
