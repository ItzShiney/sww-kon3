use crate::VertexAttribute;

#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
}

impl Vertex {
    pub fn new(position: [f32; 2], color: [f32; 4]) -> Self {
        Self { position, color }
    }
}

impl VertexAttribute for Vertex {
    fn vertex_attributes(start: u32) -> Box<[wgpu::VertexAttribute]> {
        Box::new(wgpu::vertex_attr_array![start => Float32x2, start + 1 => Float32x4])
    }
}

pub type Index = u16;
pub const INDEX_FORMAT: wgpu::IndexFormat = wgpu::IndexFormat::Uint16;
