use super::Mesh;
use super::INDEX_FORMAT;
use crate::buffers::VecBufferSlice;
use crate::drawing::Draw;
use crate::drawing::MeshPipeline;
use crate::shaders;
use crate::shaders::mesh::Transform;

pub struct DrawableMesh<'c> {
    pub mesh: &'c Mesh,
    pub transforms: VecBufferSlice<'c, Transform>,
    pub bind_groups: shaders::mesh::BindGroups<'c>,
    pub pipeline: &'c MeshPipeline,
}

impl<'c> Draw<'c> for DrawableMesh<'c> {
    fn draw(&self, render_pass: &mut wgpu::RenderPass<'c>) {
        self.pipeline.set(render_pass);
        render_pass.set_vertex_buffer(0, self.mesh.vertex_buffer().slice(..));
        render_pass.set_vertex_buffer(1, self.transforms.buffer.slice(..));

        self.bind_groups.set(render_pass);

        let instances = 0..self.transforms.values.len() as _;
        if let Some((index_buffer, indices_count)) = self.mesh.index_buffer() {
            render_pass.set_index_buffer(index_buffer.slice(..), INDEX_FORMAT);
            render_pass.draw_indexed(0..indices_count as _, 0, instances);
        } else {
            render_pass.draw(0..self.mesh.vertices_count() as _, instances);
        }
    }
}
