#![allow(clippy::derivable_impls)]
pub mod mesh {
    include!(concat!(env!("OUT_DIR"), "/mesh.rs"));
}

use glam::vec2;

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
