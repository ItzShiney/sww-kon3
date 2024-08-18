#![allow(clippy::derivable_impls)]
use super::PADDING;
use crate::Color;
use glam::vec2;
use glam::Mat2;
use glam::Vec2;

include!(concat!(env!("OUT_DIR"), "/mesh.rs"));

pub use bind_groups::*;
use glam::Vec4;

pub struct BindGroupsOwned {
    pub bind_group0: BindGroup0,
    pub bind_group1: BindGroup1,
}

impl<'s> From<&'s BindGroupsOwned> for BindGroups<'s> {
    fn from(
        BindGroupsOwned {
            bind_group0,
            bind_group1,
        }: &'s BindGroupsOwned,
    ) -> Self {
        BindGroups {
            bind_group0,
            bind_group1,
        }
    }
}

impl<'s> From<wgpu::BufferBinding<'s>> for BindGroupLayout0<'s> {
    fn from(global_transform: wgpu::BufferBinding<'s>) -> Self {
        Self { global_transform }
    }
}

pub fn in_vertex(position: glam::Vec2, color: Vec4, texture_coord: glam::Vec2) -> InVertex {
    InVertex {
        position,
        _1: PADDING,
        color,
        texture_coord,
        _2: PADDING,
    }
}

impl Transform {
    const IDENTITY: Self = Self::new(Vec2::ZERO, Color::WHITE, Rectangle::FULL);

    pub const fn new_matrix(
        translation: Vec2,
        matrix: Mat2,
        color: Vec4,
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

    pub const fn new_scale(
        translation: Vec2,
        scale: Vec2,
        color: Color,
        texture_rect: Rectangle,
    ) -> Transform {
        Transform {
            matrix: Mat2::from_diagonal(scale),
            translation,
            _1: PADDING,
            color: color.to_vec4(),
            texture_rect,
        }
    }

    pub const fn new(translation: Vec2, color: Color, texture_rect: Rectangle) -> Transform {
        Self::new_scale(translation, Vec2::ONE, color, texture_rect)
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Rectangle {
    pub const FULL: Self = Self {
        top_left: vec2(0., 0.),
        size: vec2(1., 1.),
    };

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
        Rectangle { top_left, size }
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Self::FULL
    }
}
