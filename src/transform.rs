use crate::{
    Affine,
    Color,
    FieldAttributes,
};

#[derive(Default, Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
#[repr(C)]
pub struct Transform {
    affine: Affine,
    color: Color,
}

impl Transform {
    pub fn new(affine: impl Into<Affine>, color: Color) -> Self {
        Self {
            affine: affine.into(),
            color,
        }
    }
}

impl From<Affine> for Transform {
    fn from(affine: Affine) -> Self {
        Self {
            affine,
            color: Color::WHITE,
        }
    }
}

impl From<glam::Affine2> for Transform {
    fn from(affine: glam::Affine2) -> Self {
        Self::from(Affine::from(affine))
    }
}

impl From<Color> for Transform {
    fn from(color: Color) -> Self {
        Self {
            affine: Affine::IDENTITY,
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
