use sww::drawing::Mesh;
use sww::drawing::MeshDrawer;
use sww::shaders;
use sww::shaders::mesh::Transform;
use sww::vec2;
use sww::window::RenderWindow;
use sww::VecBufferSlice;

pub struct Drawer {
    mesh_drawer: MeshDrawer,
    square: Mesh,
}

impl Drawer {
    pub fn new(rw: &RenderWindow) -> Self {
        let mesh_drawer = rw.mesh_drawer();
        let square = rw.mesh_rect(vec2(1., 1.));

        Self {
            mesh_drawer,
            square,
        }
    }

    pub fn draw_squares<'s>(
        &'s self,
        render_pass: &mut wgpu::RenderPass<'s>,
        transforms: VecBufferSlice<'s, Transform>,
        bind_groups: &shaders::mesh::BindGroups<'s>,
    ) {
        self.mesh_drawer
            .draw(render_pass, &self.square, transforms, bind_groups)
    }
}
