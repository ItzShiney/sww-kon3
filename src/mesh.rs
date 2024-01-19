use {
    crate::{
        to_wgsl_bytes,
        Index,
        Vertex,
    },
    glam::{
        vec2,
        Vec2,
    },
    wgpu::util::DeviceExt,
};

pub struct Mesh {
    vertex_buffer: wgpu::Buffer,
    vertices_count: usize,
    index_buffer: Option<(wgpu::Buffer, usize)>,
}

impl Mesh {
    pub fn new(device: &wgpu::Device, vertices: &[Vertex]) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: &to_wgsl_bytes(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            vertex_buffer,
            vertices_count: vertices.len(),
            index_buffer: None,
        }
    }

    pub fn new_indexed(device: &wgpu::Device, vertices: &[Vertex], indices: &[Index]) -> Self {
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: &to_wgsl_bytes(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            index_buffer: Some((index_buffer, indices.len())),
            ..Self::new(device, vertices)
        }
    }

    pub fn rect(device: &wgpu::Device, size: Vec2) -> Self {
        Self::new_indexed(
            &device,
            &[
                Vertex::new_white(vec2(0., 0.)),
                Vertex::new_white(vec2(0., size.y)),
                Vertex::new_white(vec2(size.x, size.y)),
                Vertex::new_white(vec2(size.x, 0.)),
            ],
            &[0, 1, 2, 0, 2, 3],
        )
    }

    pub fn square(device: &wgpu::Device, size: f32, ratio: f32) -> Self {
        Self::rect(device, vec2(size, size * ratio))
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
