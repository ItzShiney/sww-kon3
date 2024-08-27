pub mod resources;

use std::ptr;
use std::sync::Arc;
use sww::buffers::MutVecBuffer;
use sww::drawing::Mesh;
use sww::drawing::MeshPipeline;
use sww::shaders::mesh::BindGroups;
use sww::shaders::mesh::Transform;
use sww::wgpu;
use sww::window::RenderWindow;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ActiveDrawer {
    Mesh,
}

#[derive(Default)]
pub struct Drawers {
    active: Option<ActiveDrawer>,
    mesh: Option<MeshDrawerInfo>,
}

pub struct DrawPass<'s, 'e> {
    rw: Arc<RenderWindow>,
    render_pass: &'s mut wgpu::RenderPass<'e>,
    drawers: &'s mut Drawers,
}

impl<'s, 'e> DrawPass<'s, 'e> {
    pub fn new(
        rw: Arc<RenderWindow>,
        render_pass: &'s mut wgpu::RenderPass<'e>,
        drawers: &'s mut Drawers,
    ) -> Self {
        Self {
            rw,
            render_pass,
            drawers,
        }
    }
}

impl<'e> DrawPass<'_, 'e> {
    pub fn mesh(&mut self) -> MeshDrawer<'_, 'e> {
        self.set_active(ActiveDrawer::Mesh);
        let info =
            (self.drawers.mesh).get_or_insert_with(|| MeshDrawerInfo::new(Arc::clone(&self.rw)));

        MeshDrawer {
            render_pass: self.render_pass,
            info,
        }
    }

    fn set_active(&mut self, active: ActiveDrawer) {
        if (self.drawers.active).is_some_and(|self_active| self_active != active) {
            self.flush();
        }
        self.drawers.active = Some(active);
    }

    fn flush(&mut self) {
        if let Some(active) = self.drawers.active {
            match active {
                ActiveDrawer::Mesh => {
                    let info = self.drawers.mesh.as_mut().unwrap();
                    MeshDrawer {
                        render_pass: self.render_pass,
                        info,
                    }
                    .flush();
                }
            }
        }
    }
}

impl Drop for DrawPass<'_, '_> {
    fn drop(&mut self) {
        self.flush();
    }
}

#[derive(Clone)]
pub struct MeshDrawingInfo {
    pub mesh: &'static Mesh,
    pub bind_groups: BindGroups<'static>,
}

impl PartialEq for MeshDrawingInfo {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(&self.mesh, &other.mesh)
    }
}

impl Eq for MeshDrawingInfo {}

pub struct MeshDrawerInfo {
    transforms: MutVecBuffer<Transform>,
    pipeline: MeshPipeline,
    current_mesh_info: Option<MeshDrawingInfo>,
}

impl MeshDrawerInfo {
    pub fn new(rw: Arc<RenderWindow>) -> Self {
        let pipeline = MeshPipeline::new(&rw);
        let transforms = MutVecBuffer::default_vertex(rw);

        Self {
            transforms,
            pipeline,
            current_mesh_info: None,
        }
    }
}

pub struct MeshDrawer<'s, 'e> {
    render_pass: &'s mut wgpu::RenderPass<'e>,
    info: &'s mut MeshDrawerInfo,
}

impl<'e> MeshDrawer<'_, 'e> {
    pub fn draw(&mut self, mesh_info: &MeshDrawingInfo, transform: Transform) {
        if self.info.current_mesh_info.as_ref() == Some(mesh_info) {
            self.flush();
        }

        self.info.current_mesh_info = Some(mesh_info.clone());
        self.info.transforms.push(transform);
    }

    fn flush(&mut self) {
        if let Some(MeshDrawingInfo { mesh, bind_groups }) = self.info.current_mesh_info {
            mesh.draw(
                self.render_pass,
                &self.info.pipeline,
                bind_groups,
                &self.info.transforms,
            );
            self.info.transforms.clear();
        }
    }
}
