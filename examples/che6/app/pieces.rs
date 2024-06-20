use crate::Drawer;
use crate::Scalers;
use std::io;
use sww::media;
use sww::shaders;
use sww::shaders::mesh::Transform;
use sww::window::RenderWindow;
use sww::Binding;
use sww::DefaultView;
use sww::ReadableBuffer;
use sww::VecBuffer;
use sww::VecExtensions;

pub struct Pieces<'q> {
    pub transforms: VecBuffer<'q, Transform>,
    bind_group0: shaders::mesh::BindGroup0,
    bind_group1: shaders::mesh::BindGroup1,
}

impl<'q> Pieces<'q> {
    pub fn new(
        rw: &'q RenderWindow,
        scalers: &mut Scalers,
        transforms: VecBuffer<'q, Transform>,
    ) -> Self {
        let global_transform =
            scalers.push_last(ReadableBuffer::new(rw.device(), Transform::default()));

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
