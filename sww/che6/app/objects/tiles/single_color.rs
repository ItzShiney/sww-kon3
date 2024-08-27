use crate::Drawer;
use crate::Scalable;
use crate::Scalables;
use std::sync::Arc;
use sww::buffers::Binding;
use sww::buffers::MutBuffer;
use sww::buffers::MutVecBuffer;
use sww::shaders;
use sww::shaders::mesh::Transform;
use sww::window::RenderWindow;
use sww::Color;
use sww::Vec2;

pub struct SingleColorTiles {
    pub transforms: MutVecBuffer<Transform>,
    bind_group0: shaders::mesh::BindGroup0,
}

impl SingleColorTiles {
    pub fn new(
        rw: &Arc<RenderWindow>,
        scalables: &mut Scalables,
        color: Color,
        transforms: MutVecBuffer<Transform>,
    ) -> Self {
        scalables.push(Scalable::new(
            MutBuffer::new_uniform(
                rw.device(),
                Transform {
                    color: color.into(),
                    ..Default::default()
                },
            ),
            Vec2::splat(2. / 8.),
        ));
        let scalable = scalables.last().unwrap();

        let bind_group0 = {
            let global_transform = scalable.transform_buffer.buffer().binding();
            shaders::mesh::BindGroup0::from_bindings(rw.device(), global_transform.into())
        };

        Self {
            transforms,
            bind_group0,
        }
    }
}

impl SingleColorTiles {
    pub fn draw<'e>(
        &'e self,
        drawer: &'e Drawer,
        render_pass: &mut wgpu::RenderPass<'e>,
        bind_group1: &'e shaders::mesh::BindGroup1,
    ) {
        drawer.draw_squares(
            render_pass,
            &self.transforms,
            shaders::mesh::BindGroups {
                bind_group0: &self.bind_group0,
                bind_group1,
            },
        );
    }
}
