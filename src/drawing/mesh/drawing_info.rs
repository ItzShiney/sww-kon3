use super::Mesh;
use super::INDEX_FORMAT;
use crate::buffers::MutVecBuffer;
use crate::drawing::MeshPipeline;
use crate::shaders::mesh::BindGroups;
use crate::shaders::mesh::BindGroupsOwned;
use crate::shaders::mesh::Transform;

pub struct MeshDrawingInfoOwned {
    pub mesh: Mesh,
    pub bind_groups: BindGroupsOwned,
}

pub struct MeshDrawingInfo<'e> {
    pub mesh: &'e Mesh,
    pub bind_groups: BindGroups<'e>,
}

impl<'s> From<&'s MeshDrawingInfoOwned> for MeshDrawingInfo<'s> {
    fn from(
        MeshDrawingInfoOwned {
            mesh,
            bind_groups:
                BindGroupsOwned {
                    bind_group0,
                    bind_group1,
                },
        }: &'s MeshDrawingInfoOwned,
    ) -> Self {
        MeshDrawingInfo {
            mesh,
            bind_groups: BindGroups {
                bind_group0,
                bind_group1,
            },
        }
    }
}

impl<'e> MeshDrawingInfo<'e> {
    pub fn new(mesh: &'e Mesh, bind_groups: BindGroups<'e>) -> Self {
        Self { mesh, bind_groups }
    }

    pub fn draw(
        &self,
        render_pass: &mut wgpu::RenderPass<'e>,
        pipeline: &MeshPipeline,
        transforms: &mut MutVecBuffer<Transform>,
    ) {
        pipeline.set(render_pass);
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
