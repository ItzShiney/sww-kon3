use crate::buffers::VecBuffer;
use crate::shaders::mesh::in_vertex;
use crate::shaders::mesh::InVertex;
use crate::window::RenderWindow;
use crate::Color;
use glam::vec2;
use glam::Vec2;

mod drawable;
mod pipeline;

pub use drawable::*;
pub use pipeline::*;

pub type Index = u32;
pub const INDEX_FORMAT: wgpu::IndexFormat = wgpu::IndexFormat::Uint32;

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
}

impl RenderWindow<'_> {
    pub fn mesh_rect(&self, size: Vec2) -> Mesh {
        Mesh::rect(self, size)
    }

    pub fn mesh_square(&self, size: f32, ratio: f32) -> Mesh {
        Mesh::square(self, size, ratio)
    }
}
