#![allow(clippy::derivable_impls)]
use super::PADDING;
use glam::vec2;
use glam::Mat2;
use glam::Vec2;

include!(concat!(env!("OUT_DIR"), "/mesh.rs"));

pub use bind_groups::*;

impl<'s> From<wgpu::BufferBinding<'s>> for BindGroupLayout0<'s> {
    fn from(global_transform: wgpu::BufferBinding<'s>) -> Self {
        Self { global_transform }
    }
}

pub fn in_vertex(position: glam::Vec2, color: glam::Vec4, texture_coord: glam::Vec2) -> InVertex {
    InVertex {
        position,
        _1: PADDING,
        color,
        texture_coord,
        _2: PADDING,
    }
}

pub fn transform(
    matrix: Mat2,
    translation: Vec2,
    color: glam::Vec4,
    texture_rect: Rectangle,
) -> Transform {
    Transform {
        matrix,
        translation,
        _1: PADDING,
        color,
        texture_rect,
    }
}

impl Default for Transform {
    fn default() -> Self {
        transform(
            Default::default(),
            Default::default(),
            [1.; 4].into(),
            Default::default(),
        )
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            top_left: vec2(0., 0.),
            size: vec2(1., 1.),
        }
    }
}

impl Rectangle {
    pub fn offset(self, offset: Vec2) -> Self {
        Self {
            top_left: self.top_left + offset,
            size: self.size,
        }
    }

    pub fn mul_size(self, times: Vec2) -> Self {
        Self {
            top_left: self.top_left,
            size: self.size * times,
        }
    }

    pub fn subrect(self, other: Rectangle) -> Rectangle {
        let top_left = self.top_left + other.top_left * self.size;
        let size = self.size * other.size;
        return Rectangle { top_left, size };
    }
}
