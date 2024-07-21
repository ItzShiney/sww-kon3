use crate::pieces::PiecesSheet;
use crate::pieces::PiecesSheetCoord;
use crate::Drawer;
use crate::Scalable;
use crate::Scalables;
use sww::buffers::Binding;
use sww::buffers::MutBuffer;
use sww::buffers::MutVecBuffer;
use sww::shaders;
use sww::shaders::mesh::Transform;
use sww::utility::PushLast;
use sww::window::RenderWindow;
use sww::Vec2;

pub fn make_piece_transform(
    sheet: &PiecesSheet,
    translation: Vec2,
    coord: PiecesSheetCoord,
) -> Transform {
    let texture_rect = sheet.texture_rect(coord);

    Transform {
        translation,
        texture_rect,
        ..Default::default()
    }
}

pub struct Pieces<'w> {
    pub transforms: MutVecBuffer<'w, Transform>,
    sheet: PiecesSheet,
    bind_group0: shaders::mesh::BindGroup0,
    bind_group1: shaders::mesh::BindGroup1,
}

impl<'w> Pieces<'w> {
    pub fn new(
        rw: &'w RenderWindow,
        scalables: &mut Scalables,
        sheet: PiecesSheet,
        transforms: MutVecBuffer<'w, Transform>,
    ) -> Self {
        let scalable = scalables.push_last(Scalable::new(
            MutBuffer::new(rw.device(), Transform::default()),
            Vec2::splat(2. / 8.),
        ));

        let bind_group0 = shaders::mesh::BindGroup0::from_bindings(
            rw.device(),
            scalable.transform_buffer.buffer().binding().into(),
        );

        let bind_group1 = shaders::mesh::BindGroup1::from_bindings(
            rw.device(),
            shaders::mesh::BindGroupLayout1 {
                texture: sheet.texture_view(),
            },
        );

        Self {
            transforms,
            sheet,
            bind_group0,
            bind_group1,
        }
    }

    pub fn sheet(&self) -> &PiecesSheet {
        &self.sheet
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
