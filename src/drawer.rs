use sww::buffers::MutVecBuffer;
use sww::drawing::MeshDrawingInfo;
use sww::drawing::MeshPipeline;
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

    pub const fn rw(&self) -> &'e RenderWindow<'e> {
        self.rw
    }

    pub fn render_pass(&'e mut self) -> &'e mut wgpu::RenderPass<'e> {
        self.render_pass
    }
}

pub struct Drawable<'e, T> {
    drawing_info: &'e mut DrawingInfo<'e>,
    value: T,
}

impl<'e, T> Drawable<'e, T> {
    pub fn new(drawing_info: &'e mut DrawingInfo<'e>, value: T) -> Self {
        Self {
            drawing_info,
            value,
        }
    }
}

enum ActiveDrawer<'e> {
    Mesh(MeshDrawerInfo<'e>),
    _1,
}

impl<'e> From<MeshDrawerInfo<'e>> for ActiveDrawer<'e> {
    fn from(mesh: MeshDrawerInfo<'e>) -> Self {
        Self::Mesh(mesh)
    }
}

impl<'e> ActiveDrawer<'e> {
    pub fn mesh(&mut self, rw: &'e RenderWindow<'e>) -> &mut MeshDrawerInfo<'e> {
        if !matches!(self, Self::Mesh(_)) {
            *self = Self::Mesh(MeshDrawerInfo::new(rw));
        }

        let Self::Mesh(mesh) = self else {
            unreachable!()
        };

        mesh
    }
}

// TODO:
// mesh: Option<MeshDrawerInfo<'e>>,
// active: Option<ActiveDrawer>,
pub struct Drawer<'e> {
    drawing_info: DrawingInfo<'e>,
    active: Option<ActiveDrawer<'e>>,
}

impl<'e> Drawer<'e> {
    pub const fn new(drawing_info: DrawingInfo<'e>) -> Self {
        Self {
            drawing_info,
            active: None,
        }
    }

    pub fn mesh(&'e mut self) -> MeshDrawer<'e> {
        let mesh = self
            .active
            .get_or_insert_with(|| MeshDrawerInfo::new(self.drawing_info.rw).into())
            .mesh(self.drawing_info.rw);

        Drawable::new(&mut self.drawing_info, mesh)
    }
}

pub struct MeshDrawerInfo<'e> {
    transforms: MutVecBuffer<'e, Transform>,
    pipeline: MeshPipeline,
    current_mesh: Option<&'e MeshDrawingInfo<'e>>,
}

impl<'e> MeshDrawerInfo<'e> {
    pub fn new(rw: &'e RenderWindow) -> Self {
        Self {
            transforms: MutVecBuffer::default_vertex(rw),
            pipeline: MeshPipeline::new(rw),
            current_mesh: None,
        }
    }
}

pub type MeshDrawer<'e> = Drawable<'e, &'e mut MeshDrawerInfo<'e>>;

impl<'e> MeshDrawer<'e> {
    pub fn draw(&mut self, mesh: &'e MeshDrawingInfo<'e>, transform: Transform) {
        if self
            .value
            .current_mesh
            .map(|current_mesh| current_mesh.mesh.id())
            != Some(mesh.mesh.id())
        {
            self.flush();
        }

        self.value.current_mesh = Some(mesh);
        self.value.transforms.push(transform);
    }

    fn flush(&mut self) {
        if let Some(mesh_drawing_info) = self.value.current_mesh {
            mesh_drawing_info.draw(
                self.drawing_info.render_pass,
                &self.value.pipeline,
                &mut self.value.transforms,
            );
        }
    }
}
