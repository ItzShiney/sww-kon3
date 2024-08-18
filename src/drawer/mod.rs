pub mod resources;

use sww::buffers::MutVecBuffer;
use sww::drawing::Mesh;
use sww::drawing::MeshPipeline;
use sww::shaders::mesh::BindGroups;
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

pub struct WithDrawingInfo<'d, 'e, T> {
    drawing_info: &'d mut DrawingInfo<'e>,
    value: T,
}

impl<'d, 'e, T> WithDrawingInfo<'d, 'e, T> {
    pub fn new(drawing_info: &'d mut DrawingInfo<'e>, value: T) -> Self {
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

impl<'r, 'e> From<MeshDrawerInfo<'e>> for ActiveDrawer<'e> {
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
    pub fn new(drawing_info: DrawingInfo<'e>) -> Self {
        Self {
            drawing_info,
            active: None,
        }
    }

    pub fn mesh(&mut self) -> MeshDrawer<'_, 'e> {
        let mesh = self
            .active
            .get_or_insert_with(|| MeshDrawerInfo::new(self.drawing_info.rw).into())
            .mesh(self.drawing_info.rw);

        MeshDrawer::new(&mut self.drawing_info, mesh)
    }
}

impl Drop for Drawer<'_> {
    fn drop(&mut self) {
        if let Some(active) = &mut self.active {
            match active {
                ActiveDrawer::Mesh(mesh) => MeshDrawer::new(&mut self.drawing_info, mesh).flush(),
                ActiveDrawer::_1 => unreachable!(),
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct MeshDrawingInfo<'e> {
    pub mesh: &'e Mesh,
    pub bind_groups: BindGroups<'e>,
}

pub struct MeshDrawerInfo<'e> {
    transforms: MutVecBuffer<'e, Transform>,
    pipeline: MeshPipeline,
    current_mesh: Option<MeshDrawingInfo<'e>>,
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

pub type MeshDrawer<'d, 'e> = WithDrawingInfo<'d, 'e, &'d mut MeshDrawerInfo<'e>>;

impl<'e> MeshDrawer<'_, 'e> {
    pub fn draw(&mut self, mesh: MeshDrawingInfo<'e>, transform: Transform) {
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
        if let Some(MeshDrawingInfo { mesh, bind_groups }) = self.value.current_mesh {
            mesh.draw(
                self.drawing_info.render_pass,
                &self.value.pipeline,
                bind_groups,
                &mut self.value.transforms,
            );
            self.value.transforms.clear();
        }
    }
}
