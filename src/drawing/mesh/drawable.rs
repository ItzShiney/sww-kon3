use super::Mesh;
use super::INDEX_FORMAT;
use crate::buffers::MutVecBuffer;
use crate::drawing::MeshPipeline;
use crate::shaders;
use crate::shaders::mesh::Transform;

pub struct MeshDrawingInfo<'e> {
    pub mesh: &'e Mesh,
    pub pipeline: &'e MeshPipeline,
    pub bind_groups: shaders::mesh::BindGroups<'e>,
}

impl<'e> MeshDrawingInfo<'e> {
    pub fn new(
        mesh: &'e Mesh,
        pipeline: &'e MeshPipeline,
        bind_groups: shaders::mesh::BindGroups<'e>,
    ) -> Self {
        Self {
            mesh,
            bind_groups,
            pipeline,
        }
    }

    pub fn draw(
        &self,
        render_pass: &mut wgpu::RenderPass<'e>,
        transforms: &mut MutVecBuffer<Transform>,
    ) {
        self.pipeline.set(render_pass);
        render_pass.set_vertex_buffer(0, self.mesh.vertices().buffer().slice(..));
        render_pass.set_vertex_buffer(1, transforms.update_buffer());

        self.bind_groups.set(render_pass);

        let instances = 0..transforms.values().len() as _;
        if let Some(indices) = self.mesh.indices() {
            render_pass.set_index_buffer(indices.buffer().slice(..), INDEX_FORMAT);
            render_pass.draw_indexed(0..indices.count() as _, 0, instances);
        } else {
            render_pass.draw(0..self.mesh.vertices().count() as _, instances);
        }
    }
}
