pub mod mesh;

impl Clone for mesh::bind_groups::BindGroups<'_> {
    fn clone(&self) -> Self {
        Self {
            bind_group0: self.bind_group0,
        }
    }
}

impl Copy for mesh::bind_groups::BindGroups<'_> {}
