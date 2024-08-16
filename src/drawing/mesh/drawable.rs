use super::Mesh;
use super::INDEX_FORMAT;
use crate::buffers::MutVecBuffer;
use crate::drawing::MeshPipeline;
use crate::shaders;
use crate::shaders::mesh::Transform;

pub struct DrawableMesh<'e, 't, 'w> {
    pub mesh: &'e Mesh,
    pub transforms: &'t mut MutVecBuffer<'w, Transform>,
    pub bind_groups: shaders::mesh::BindGroups<'e>,
    pub pipeline: &'e MeshPipeline,
}

impl<'e> DrawableMesh<'e, '_, '_> {
    pub fn draw(&mut self, render_pass: &mut wgpu::RenderPass<'e>) {
        self.pipeline.set(render_pass);
        render_pass.set_vertex_buffer(0, self.mesh.vertices().buffer().slice(..));
        render_pass.set_vertex_buffer(1, self.transforms.update_buffer());

        self.bind_groups.set(render_pass);

        let instances = 0..self.transforms.values().len() as _;
        if let Some(indices) = self.mesh.indices() {
            render_pass.set_index_buffer(indices.buffer().slice(..), INDEX_FORMAT);
            render_pass.draw_indexed(0..indices.count() as _, 0, instances);
        } else {
            render_pass.draw(0..self.mesh.vertices().count() as _, instances);
        }
    }
}
