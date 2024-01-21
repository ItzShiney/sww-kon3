pub mod mesh;

use glam::vec2;

impl Default for mesh::Rectangle {
    fn default() -> Self {
        mesh::Rectangle {
            start: vec2(0., 0.),
            end: vec2(1., 1.),
        }
    }
}
