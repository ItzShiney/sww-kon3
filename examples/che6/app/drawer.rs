use sww::buffers::VecBufferSlice;
use sww::drawing::Draw;
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
        let pipeline = rw.create_mesh_pipeline();
        let square = rw.mesh_rect(vec2(1., 1.));

        Self {
            mesh_pipeline: pipeline,
            square,
        }
    }
}

impl<'c> Drawer {
    pub fn draw_squares(
        &'c self,
        render_pass: &mut wgpu::RenderPass<'c>,
        transforms: VecBufferSlice<'c, Transform>,
        bind_groups: shaders::mesh::BindGroups<'c>,
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
