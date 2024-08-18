use crate::buffers::MutVecBuffer;
use crate::buffers::VecBuffer;
use crate::shaders::mesh::in_vertex;
use crate::shaders::mesh::BindGroups;
use crate::shaders::mesh::InVertex;
use crate::shaders::mesh::Transform;
use crate::window::RenderWindow;
use crate::Color;
use glam::vec2;
use glam::Vec2;

mod pipeline;

pub use pipeline::*;

pub type Index = u32;
pub const INDEX_FORMAT: wgpu::IndexFormat = wgpu::IndexFormat::Uint32;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MeshId(wgpu::Id<wgpu::Buffer>);

pub struct Mesh {
    vertices: VecBuffer<InVertex>,
    indices: Option<VecBuffer<Index>>,
}

impl Mesh {
    pub fn new(device: &wgpu::Device, vertices: &[InVertex]) -> Self {
        Self {
            vertices: VecBuffer::new(device, vertices, wgpu::BufferUsages::VERTEX),
            indices: None,
        }
    }

    pub fn new_indexed(device: &wgpu::Device, vertices: &[InVertex], indices: &[Index]) -> Self {
        Self {
            indices: Some(VecBuffer::new(device, indices, wgpu::BufferUsages::INDEX)),
            ..Self::new(device, vertices)
        }
    }

    pub fn rect(rw: &RenderWindow, size: Vec2) -> Self {
        Self::new_indexed(
            rw.device(),
            &[
                in_vertex(vec2(0., 0.), Color::WHITE.into(), vec2(0., 0.)),
                in_vertex(vec2(0., size.y), Color::WHITE.into(), vec2(0., 1.)),
                in_vertex(vec2(size.x, size.y), Color::WHITE.into(), vec2(1., 1.)),
                in_vertex(vec2(size.x, 0.), Color::WHITE.into(), vec2(1., 0.)),
            ],
            &[0, 1, 2, 0, 2, 3],
        )
    }

    pub fn square(rw: &RenderWindow, size: f32, ratio: f32) -> Self {
        Self::rect(rw, vec2(size, size * ratio))
    }

    pub fn vertices(&self) -> &VecBuffer<InVertex> {
        &self.vertices
    }

    pub fn indices(&self) -> Option<&VecBuffer<Index>> {
        self.indices.as_ref()
    }

    pub fn id(&self) -> MeshId {
        MeshId(self.vertices.buffer().global_id())
    }

    pub fn draw<'e>(
        &self,
        render_pass: &mut wgpu::RenderPass<'e>,
        pipeline: &MeshPipeline,
        bind_groups: BindGroups<'e>,
        transforms: &mut MutVecBuffer<Transform>,
    ) {
        pipeline.set(render_pass);
        bind_groups.set(render_pass);
        render_pass.set_vertex_buffer(0, self.vertices().buffer().slice(..));
        render_pass.set_vertex_buffer(1, transforms.update_buffer());

        let instances = 0..transforms.values().len() as _;
        if let Some(indices) = self.indices() {
            render_pass.set_index_buffer(indices.buffer().slice(..), INDEX_FORMAT);
            render_pass.draw_indexed(0..indices.count() as _, 0, instances);
        } else {
            render_pass.draw(0..self.vertices().count() as _, instances);
        }
    }
}
