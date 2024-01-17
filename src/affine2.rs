#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
#[repr(C)]
pub struct Affine2 {
    matrix2: [[f32; 2]; 2],
    translation: [f32; 2],
}

impl Default for Affine2 {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl From<glam::Affine2> for Affine2 {
    fn from(value: glam::Affine2) -> Self {
        Self::new(value)
    }
}

impl Affine2 {
    pub const ZERO: Self = Self::new(glam::Affine2::ZERO);
    pub const IDENTITY: Self = Self::new(glam::Affine2::IDENTITY);
    pub const NAN: Self = Self::new(glam::Affine2::NAN);

    pub const fn new(affine: glam::Affine2) -> Self {
        Self {
            matrix2: affine.matrix2.to_cols_array_2d(),
            translation: affine.translation.to_array(),
        }
    }
}
