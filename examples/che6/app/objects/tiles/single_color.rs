use crate::Drawer;
use crate::Scalers;
use sww::shaders;
use sww::shaders::mesh::Transform;
use sww::AppInfo;
use sww::Binding;
use sww::Color;
use sww::ReadableBuffer;
use sww::VecBuffer;
use sww::VecExtensions;

pub struct SingleColorTiles<'q> {
    pub transforms: VecBuffer<'q, Transform>,
    bind_group0: shaders::mesh::BindGroup0,
}

impl<'q> SingleColorTiles<'q> {
    pub fn new(
        app_info: &'q AppInfo,
        scalers: &mut Scalers,
        color: Color,
        transforms: VecBuffer<'q, Transform>,
    ) -> Self {
        let global_transform = scalers.push_last(ReadableBuffer::new(
            app_info.device(),
            Transform {
                color: color.into(),
                ..Default::default()
            },
        ));

        let bind_group0 = {
            let global_transform = global_transform.buffer().binding();
            shaders::mesh::BindGroup0::from_bindings(app_info.device(), global_transform.into())
        };

        Self {
            transforms,
            bind_group0,
        }
    }

    pub fn draw<'s>(
        &'s self,
        drawer: &'s Drawer,
        render_pass: &mut wgpu::RenderPass<'s>,
        bind_group1: &'s shaders::mesh::BindGroup1,
    ) {
        drawer.draw_squares(
            render_pass,
            self.transforms.slice(..),
            &shaders::mesh::BindGroups {
                bind_group0: &self.bind_group0,
                bind_group1,
            },
        );
    }
}
