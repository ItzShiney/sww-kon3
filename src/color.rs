#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
#[repr(C)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Default for Color {
    fn default() -> Self {
        Self::WHITE
    }
}

impl Color {
    pub const WHITE: Self = Self::splat(1.);
    pub const RED: Self = Self::new_rgb(1., 0., 0.);
    pub const GREEN: Self = Self::new_rgb(0., 1., 0.);
    pub const BLUE: Self = Self::new_rgb(0., 0., 1.);
    pub const BLACK: Self = Self::splat(0.);
    pub const TRANSPARENT: Self = Self::splat_a(0.);

    pub const fn new_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub const fn new_rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new_rgba(r, g, b, 1.)
    }

    pub const fn splat(v: f32) -> Self {
        Self::new_rgba(v, v, v, 1.)
    }

    pub const fn splat_a(v: f32) -> Self {
        Self::new_rgba(v, v, v, v)
    }

    pub const fn to_array(self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}
