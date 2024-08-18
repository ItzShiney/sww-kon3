use sww::buffers::MutVecBuffer;
use sww::drawing::Mesh;
use sww::drawing::MeshPipeline;
use sww::shaders;
use sww::shaders::mesh::Transform;
use sww::window::RenderWindow;
use sww::Vec2;

pub struct Drawer {
    mesh_pipeline: MeshPipeline,
    square: Mesh,
}

impl Drawer {
    pub fn new(rw: &RenderWindow) -> Self {
        let mesh_pipeline = MeshPipeline::new(rw);
        let square = Mesh::rect(rw, Vec2::ONE);

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
        (self.square).draw(render_pass, &self.mesh_pipeline, bind_groups, transforms)
    }
}
