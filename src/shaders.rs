pub mod mesh;

impl Clone for mesh::bind_groups::BindGroups<'_> {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for mesh::bind_groups::BindGroups<'_> {}
