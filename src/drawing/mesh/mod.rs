use crate::app::AppInfo;
use crate::shaders::mesh::in_vertex;
use crate::shaders::mesh::InVertex;
use crate::Color;
use glam::vec2;
use glam::Vec2;
use wgpu::util::DeviceExt;

mod drawer;

pub use drawer::*;

pub type Index = u32;
pub const INDEX_FORMAT: wgpu::IndexFormat = wgpu::IndexFormat::Uint32;

pub struct Mesh {
    vertex_buffer: wgpu::Buffer,
    vertices_count: usize,
    index_buffer: Option<(wgpu::Buffer, usize)>,
}

impl Mesh {
    pub fn new(device: &wgpu::Device, vertices: &[InVertex]) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            vertex_buffer,
            vertices_count: vertices.len(),
            index_buffer: None,
        }
    }

    pub fn new_indexed(device: &wgpu::Device, vertices: &[InVertex], indices: &[Index]) -> Self {
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            index_buffer: Some((index_buffer, indices.len())),
            ..Self::new(device, vertices)
        }
    }

    pub fn rect(app_info: &AppInfo, size: Vec2) -> Self {
        Self::new_indexed(
            app_info.device(),
            &[
                in_vertex(vec2(0., 0.), Color::WHITE.into(), vec2(0., 0.)),
                in_vertex(vec2(0., size.y), Color::WHITE.into(), vec2(0., 1.)),
                in_vertex(vec2(size.x, size.y), Color::WHITE.into(), vec2(1., 1.)),
                in_vertex(vec2(size.x, 0.), Color::WHITE.into(), vec2(1., 0.)),
            ],
            &[0, 1, 2, 0, 2, 3],
        )
    }

    pub fn square(app_info: &AppInfo, size: f32, ratio: f32) -> Self {
        Self::rect(app_info, vec2(size, size * ratio))
    }

    pub fn vertex_buffer(&self) -> &wgpu::Buffer {
        &self.vertex_buffer
    }

    pub fn vertices_count(&self) -> usize {
        self.vertices_count
    }

    pub fn index_buffer(&self) -> Option<(&wgpu::Buffer, usize)> {
        if let Some((ref index_buffer, indices_count)) = self.index_buffer {
            Some((index_buffer, indices_count))
        } else {
            None
        }
    }
}

impl AppInfo<'_> {
    pub fn mesh_rect(&self, size: Vec2) -> Mesh {
        Mesh::rect(self, size)
    }

    pub fn mesh_square(&self, size: f32, ratio: f32) -> Mesh {
        Mesh::square(self, size, ratio)
    }
}
