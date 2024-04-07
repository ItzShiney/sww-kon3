#![allow(clippy::derivable_impls)]
pub mod mesh {
    include!(concat!(env!("OUT_DIR"), "/mesh.rs"));

    pub use bind_groups::*;

    impl<'s> From<wgpu::BufferBinding<'s>> for BindGroupLayout0<'s> {
        fn from(global_transform: wgpu::BufferBinding<'s>) -> Self {
            Self { global_transform }
        }
    }
}

use crate::AppInfo;
use glam::vec2;

impl AppInfo<'_> {
    pub fn mesh_bind_group0(&self) -> mesh::BindGroup0 {
        todo!()
    }
}

impl Default for mesh::Transform {
    fn default() -> Self {
        Self {
            matrix: Default::default(),
            translation: Default::default(),
            color: [1.; 4].into(),
            texture_rect: Default::default(),
        }
    }
}

impl Default for mesh::Rectangle {
    fn default() -> Self {
        mesh::Rectangle {
            top_left: vec2(0., 0.),
            size: vec2(1., 1.),
        }
    }
}
