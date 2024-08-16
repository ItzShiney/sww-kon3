use sww::buffers::MutVecBuffer;
use sww::drawing::DrawableMesh;
use sww::drawing::Mesh;
use sww::drawing::MeshPipeline;
use sww::shaders;
use sww::shaders::mesh::Transform;
use sww::vec2;
use sww::window::RenderWindow;

pub struct Drawer {
    mesh_pipeline: MeshPipeline,
    square: Mesh,
}

impl Drawer {
    pub fn new(rw: &RenderWindow) -> Self {
        let mesh_pipeline = MeshPipeline::new(rw);
        let square = Mesh::rect(rw, vec2(1., 1.));

        Self {
            mesh_pipeline,
            square,
        }
    }
}

impl Drawer {
    pub fn draw_squares<'e>(
        &'e self,
        render_pass: &mut wgpu::RenderPass<'e>,
        transforms: &mut MutVecBuffer<Transform>,
        bind_groups: shaders::mesh::BindGroups<'e>,
    ) {
        DrawableMesh {
            mesh: &self.square,
            transforms,
            bind_groups,
            pipeline: &self.mesh_pipeline,
        }
        .draw(render_pass)
    }
}
