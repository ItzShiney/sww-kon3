use {
    crate::{
        Polygon,
        Vertex,
        VertexAttributes,
        INDEX_FORMAT,
    },
    std::borrow::Cow,
};

pub struct PolygonDrawer {
    pipeline: wgpu::RenderPipeline,
}

impl PolygonDrawer {
    pub fn new(device: &wgpu::Device, swapchain_format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("polygon.wgsl"))),
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
                buffers: &[VertexAttributes::<Vertex>::new(0).layout()],
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

    pub fn draw<'s>(&'s self, render_pass: &mut wgpu::RenderPass<'s>, polygon: &'s Polygon) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, polygon.vertex_buffer().slice(..));

        if let Some((index_buffer, indices_count)) = polygon.index_buffer() {
            render_pass.set_index_buffer(index_buffer.slice(..), INDEX_FORMAT);
            render_pass.draw_indexed(0..indices_count as _, 0, 0..1);
        } else {
            render_pass.draw(0..polygon.vertices_count() as _, 0..1);
        }
    }
}
