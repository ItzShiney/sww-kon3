use {
    crate::{
        Attributes,
        Instances,
        Mesh,
        Transform,
        Vertex,
        WgslBytesWriter,
        INDEX_FORMAT,
    },
    std::{
        borrow::Cow,
        cell::RefCell,
        io::Write,
        mem,
        num::NonZeroU64,
    },
};

type BufferType = Transform;

pub struct MeshDrawer {
    pipeline: wgpu::RenderPipeline,

    bind_group: wgpu::BindGroup,
    bind_buffer: wgpu::Buffer,
    bytes_writer: RefCell<WgslBytesWriter<BufferType>>,
}

impl MeshDrawer {
    pub fn new(device: &wgpu::Device, swapchain_format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("mesh.wgsl"))),
        });

        let bind_buffer_size = mem::size_of::<BufferType>() as wgpu::BufferAddress;
        let bind_buffer_size_nonzero = NonZeroU64::new(bind_buffer_size).unwrap();

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: Some(bind_buffer_size_nonzero),
                },
                count: None,
            }],
        });

        let bind_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: bind_buffer_size,
            usage: wgpu::BufferUsages::UNIFORM
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::MAP_WRITE,
            mapped_at_creation: true,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &bind_buffer,
                    offset: 0,
                    size: Some(bind_buffer_size_nonzero),
                }),
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

        Self {
            pipeline,
            bind_buffer,
            bind_group,
            bytes_writer: WgslBytesWriter::default().into(),
        }
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

        self.bind_buffer(render_pass, instances.transform);

        let instances = 0..instances.len() as _;
        if let Some((index_buffer, indices_count)) = mesh.index_buffer() {
            render_pass.set_index_buffer(index_buffer.slice(..), INDEX_FORMAT);
            render_pass.draw_indexed(0..indices_count as _, 0, instances);
        } else {
            render_pass.draw(0..mesh.vertices_count() as _, instances);
        }
    }

    fn bind_buffer<'s>(&'s self, render_pass: &mut wgpu::RenderPass<'s>, value: BufferType) {
        self.bind_buffer
            .slice(..)
            .get_mapped_range_mut()
            .as_mut()
            .write(self.bytes_writer.borrow_mut().write(&value))
            .unwrap();
        self.bind_buffer.unmap();

        render_pass.set_bind_group(0, &self.bind_group, &[]);
    }
}
