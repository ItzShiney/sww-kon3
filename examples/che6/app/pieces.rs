use crate::Drawer;
use crate::Scalers;
use std::io;
use sww::read_texture;
use sww::shaders;
use sww::shaders::mesh::Transform;
use sww::AppInfo;
use sww::Binding;
use sww::DefaultView;
use sww::ReadableBuffer;
use sww::VecBuffer;
use sww::VecExtensions;

pub struct Pieces {
    pub transforms: VecBuffer<Transform>,
    bind_group0: shaders::mesh::BindGroup0,
    bind_group1: shaders::mesh::BindGroup1,
}

impl Pieces {
    pub fn new(
        app_info: &AppInfo,
        scalers: &mut Scalers,
        transforms: VecBuffer<Transform>,
    ) -> Self {
        let global_transform =
            scalers.push_last(ReadableBuffer::new(&app_info.device, Transform::default()));

        let texture = read_texture(
            &app_info.device,
            app_info.queue(),
            io::Cursor::new(include_bytes!("../pieces.png")),
        );
        let texture_view = texture.default_view();

        let bind_group0 = shaders::mesh::BindGroup0::from_bindings(
            &app_info.device,
            global_transform.buffer().binding().into(),
        );

        let bind_group1 = shaders::mesh::BindGroup1::from_bindings(
            &app_info.device,
            shaders::mesh::BindGroupLayout1 {
                texture: &texture_view,
            },
        );

        Self {
            transforms,
            bind_group0,
            bind_group1,
        }
    }

    pub fn draw<'s>(&'s self, drawer: &'s Drawer, render_pass: &mut wgpu::RenderPass<'s>) {
        drawer.draw_squares(
            render_pass,
            self.transforms.slice(..),
            &shaders::mesh::BindGroups {
                bind_group0: &self.bind_group0,
                bind_group1: &self.bind_group1,
            },
        );
    }
}
