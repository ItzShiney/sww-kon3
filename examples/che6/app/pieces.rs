use crate::Drawer;
use crate::Scalers;
use std::io;
use sww::buffers::Binding;
use sww::buffers::MutBuffer;
use sww::buffers::MutVecBuffer;
use sww::media;
use sww::media::DefaultView;
use sww::shaders;
use sww::shaders::mesh::Transform;
use sww::utility::PushLast;
use sww::window::RenderWindow;

pub struct Pieces<'w> {
    pub transforms: MutVecBuffer<'w, Transform>,
    bind_group0: shaders::mesh::BindGroup0,
    bind_group1: shaders::mesh::BindGroup1,
}

impl<'w> Pieces<'w> {
    pub fn new(
        rw: &'w RenderWindow,
        scalers: &mut Scalers,
        transforms: MutVecBuffer<'w, Transform>,
    ) -> Self {
        let global_transform = scalers.push_last(MutBuffer::new(rw.device(), Transform::default()));

        let texture_view = media::read_texture(
            rw.device(),
            rw.queue(),
            io::Cursor::new(include_bytes!("../pieces.png")),
        )
        .unwrap()
        .default_view();

        let bind_group0 = shaders::mesh::BindGroup0::from_bindings(
            rw.device(),
            global_transform.buffer().binding().into(),
        );

        let bind_group1 = shaders::mesh::BindGroup1::from_bindings(
            rw.device(),
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
}

impl<'c> Pieces<'_> {
    pub fn draw(&'c self, drawer: &'c Drawer, render_pass: &mut wgpu::RenderPass<'c>) {
        drawer.draw_squares(
            render_pass,
            self.transforms.slice(..),
            shaders::mesh::BindGroups {
                bind_group0: &self.bind_group0,
                bind_group1: &self.bind_group1,
            },
        );
    }
}
