use crate::{
    Affine2,
    Color,
    FieldAttributes,
};

#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
#[repr(C)]
pub struct Transform {
    affine: Affine2,
    color: Color,
}

impl Default for Transform {
    fn default() -> Self {
        Self::new(Affine2::IDENTITY, Color::WHITE)
    }
}

impl Transform {
    pub fn new(affine: impl Into<Affine2>, color: Color) -> Self {
        Self {
            affine: affine.into(),
            color,
        }
    }
}

impl FieldAttributes for Transform {
    fn field_attributes(start: u32) -> Box<[wgpu::VertexAttribute]> {
        Box::new(
            wgpu::vertex_attr_array![start => Float32x2, start + 1 => Float32x2, start + 2 => Float32x2, start + 3 => Float32x4],
        )
    }
}
