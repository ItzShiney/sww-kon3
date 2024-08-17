use sww::buffers::MutVecBuffer;
use sww::drawing::MeshDrawingInfo;
use sww::shaders::mesh::Transform;
use sww::wgpu;
use sww::window::RenderWindow;

pub struct DrawingInfo<'e> {
    rw: &'e RenderWindow<'e>,
    render_pass: &'e mut wgpu::RenderPass<'e>,
}

impl<'e> DrawingInfo<'e> {
    pub fn new(rw: &'e RenderWindow<'e>, render_pass: &'e mut wgpu::RenderPass<'e>) -> Self {
        Self { rw, render_pass }
    }
}

enum DrawerInner<'e> {
    DrawingInfo(Option<DrawingInfo<'e>>),
    Mesh(Option<MeshDrawer<'e>>),
}

impl<'e> DrawerInner<'e> {
    pub const fn new(drawing_info: DrawingInfo<'e>) -> Self {
        Self::DrawingInfo(Some(drawing_info))
    }

    pub fn mesh(&mut self) -> &mut MeshDrawer<'e> {
        if !matches!(self, Self::Mesh(Some(_))) {
            let drawing_info = self.flush_take_drawing_info();
            *self = Self::Mesh(Some(MeshDrawer::new(drawing_info)));
        }

        let Self::Mesh(Some(mesh)) = self else {
            unreachable!()
        };

        mesh
    }

    fn flush_take_drawing_info(&mut self) -> DrawingInfo<'e> {
        match self {
            DrawerInner::DrawingInfo(drawing_info) => drawing_info.take().unwrap(),

            DrawerInner::Mesh(mesh_drawer) => {
                let mut mesh_drawer = mesh_drawer.take().unwrap();
                mesh_drawer.flush();
                mesh_drawer.drawing_info
            }
        }
    }
}

pub struct Drawer<'e>(DrawerInner<'e>);

impl<'e> Drawer<'e> {
    pub const fn new(drawing_info: DrawingInfo<'e>) -> Self {
        Self(DrawerInner::new(drawing_info))
    }

    pub fn mesh(&mut self) -> &mut MeshDrawer<'e> {
        self.0.mesh()
    }
}

pub struct MeshDrawer<'e> {
    drawing_info: DrawingInfo<'e>,
    transforms: MutVecBuffer<'e, Transform>,
    mesh_drawing_info: Option<&'e MeshDrawingInfo<'e>>,
}

impl<'e> MeshDrawer<'e> {
    pub fn new(drawing_info: DrawingInfo<'e>) -> Self {
        let rw = drawing_info.rw;
        Self {
            drawing_info,
            transforms: MutVecBuffer::default_vertex(rw),
            mesh_drawing_info: None,
        }
    }

    pub fn draw(&mut self, mesh_drawing_info: &'e MeshDrawingInfo, transform: Transform) {
        if self
            .mesh_drawing_info
            .map(|self_mesh_drawing_info| self_mesh_drawing_info.mesh.id())
            != Some(mesh_drawing_info.mesh.id())
        {
            self.flush();
        }

        self.mesh_drawing_info = Some(mesh_drawing_info);
        self.transforms.push(transform);
    }

    fn flush(&mut self) {
        if let Some(mesh_drawing_info) = self.mesh_drawing_info {
            mesh_drawing_info.draw(self.drawing_info.render_pass, &mut self.transforms);
        }
    }
}
