use glam::{
    vec2,
    Affine2,
    Mat2,
    Vec2,
};

#[derive(Default, Clone, Copy, encase::ShaderType)]
pub struct Affine {
    pub matrix: Mat2,
    pub translation: Vec2,
}

impl From<Affine2> for Affine {
    fn from(value: Affine2) -> Self {
        Self::new(value)
    }
}

impl Affine {
    pub const ZERO: Self = Self::new(Affine2::ZERO);
    pub const IDENTITY: Self = Self::new(Affine2::IDENTITY);

    pub const fn new(affine: Affine2) -> Self {
        Self {
            matrix: affine.matrix2,
            translation: affine.translation,
        }
    }

    pub fn from_scale(scale: Vec2) -> Self {
        Self::from(Affine2::from_scale(scale))
    }

    pub fn from_scale_splat(scale: f32) -> Self {
        Self::from_scale(vec2(scale, scale))
    }
}
