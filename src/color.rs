use glam::Vec4;

#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
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
    pub const BLACK: Self = Self::splat(0.);
    pub const RED: Self = Self::new_rgb(1., 0., 0.);
    pub const GREEN: Self = Self::new_rgb(0., 1., 0.);
    pub const BLUE: Self = Self::new_rgb(0., 0., 1.);
    pub const YELLOW: Self = Self::new_rgb(1., 1., 0.);
    pub const MAGENTA: Self = Self::new_rgb(1., 0., 1.);
    pub const CYAN: Self = Self::new_rgb(0., 1., 1.);
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

    pub const fn to_vec4(self) -> Vec4 {
        Vec4::from_array(self.to_array())
    }

    pub const fn with_r(self, r: f32) -> Self {
        Self::new_rgba(r, self.g, self.b, self.a)
    }

    pub const fn with_g(self, g: f32) -> Self {
        Self::new_rgba(self.r, g, self.b, self.a)
    }

    pub const fn with_b(self, b: f32) -> Self {
        Self::new_rgba(self.r, self.g, b, self.a)
    }

    pub const fn with_a(self, a: f32) -> Self {
        Self::new_rgba(self.r, self.g, self.b, a)
    }
}

impl From<Color> for Vec4 {
    fn from(value: Color) -> Self {
        value.to_vec4()
    }
}
