use glam::vec2;

pub mod mesh;

impl Clone for mesh::bind_groups::BindGroups<'_> {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for mesh::bind_groups::BindGroups<'_> {}

impl Default for mesh::Rectangle {
    fn default() -> Self {
        mesh::Rectangle {
            start: vec2(0., 0.),
            end: vec2(1., 1.),
        }
    }
}
