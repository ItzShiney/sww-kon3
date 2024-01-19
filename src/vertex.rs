use {
    crate::{
        Color,
        FieldAttributes,
    },
    glam::Vec2,
};

#[derive(Clone, Copy, encase::ShaderType)]
#[repr(C)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
}

impl Vertex {
    pub fn new(position: Vec2, color: Color) -> Self {
        Self {
            position: position.to_array(),
            color: color.to_array(),
        }
    }

    pub fn new_white(position: Vec2) -> Self {
        Self::new(position, Color::WHITE)
    }
}

impl FieldAttributes for Vertex {
    fn field_attributes(start: u32) -> Box<[wgpu::VertexAttribute]> {
        Box::new(wgpu::vertex_attr_array![start => Float32x2, start + 1 => Float32x4])
    }
}

pub type Index = u32;
pub const INDEX_FORMAT: wgpu::IndexFormat = wgpu::IndexFormat::Uint32;
