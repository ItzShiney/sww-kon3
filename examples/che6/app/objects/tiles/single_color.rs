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

pub struct SingleColorTiles {
    pub transforms: VecBuffer<Transform>,
    bind_group0: shaders::mesh::BindGroup0,
}

impl SingleColorTiles {
    pub fn new(
        app_info: &AppInfo,
        scalers: &mut Scalers,
        color: Color,
        transforms: VecBuffer<Transform>,
    ) -> Self {
        let global_transform = scalers.push_last(ReadableBuffer::new(
            &app_info.device,
            Transform {
                color: color.into(),
                ..Default::default()
            },
        ));

        let bind_group0 = {
            let global_transform = global_transform.buffer().binding();
            shaders::mesh::BindGroup0::from_bindings(&app_info.device, global_transform.into())
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
