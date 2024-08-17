use std::mem::ManuallyDrop;
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
    DrawingInfo(ManuallyDrop<DrawingInfo<'e>>),
    Mesh(ManuallyDrop<MeshDrawer<'e>>),
}

impl<'e> DrawerInner<'e> {
    pub const fn new(drawing_info: DrawingInfo<'e>) -> Self {
        Self::DrawingInfo(ManuallyDrop::new(drawing_info))
    }

    pub fn mesh(&mut self) -> &mut MeshDrawer<'e> {
        if !matches!(self, Self::Mesh(_)) {
            // SAFETY:
            // * `self` is being written to immediately after moving out of it using `ManuallyDrop`
            // * nothing here panics
            let drawing_info = unsafe { self.flush_take_drawing_info() };
            *self = Self::Mesh(ManuallyDrop::new(MeshDrawer::new(drawing_info)));
        }

        let Self::Mesh(mesh) = self else {
            unreachable!()
        };

        mesh
    }

    unsafe fn flush_take_drawing_info(&mut self) -> DrawingInfo<'e> {
        match self {
            DrawerInner::DrawingInfo(drawing_info) => ManuallyDrop::take(drawing_info),

            DrawerInner::Mesh(mesh_drawer) => {
                let mut mesh_drawer = ManuallyDrop::take(mesh_drawer);
                mesh_drawer.flush();
                mesh_drawer.drawing_info
            }
        }
    }
}

impl Drop for DrawerInner<'_> {
    fn drop(&mut self) {
        // SAFETY: if everything else is safe, then `self` currently holds a value, which `DrawingInfo` can be taken from
        unsafe {
            self.flush_take_drawing_info();
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
