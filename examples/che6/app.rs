mod drawer;
mod objects;
mod pieces;

use crate::sheet::make_piece_transform;
use crate::sheet::PieceColor;
use crate::sheet::PieceType;
pub use drawer::*;
pub use objects::*;
pub use pieces::*;
use sww::*;

pub fn translation(x: i32, y: i32) -> Vec2 {
    vec2(x as _, y as _)
}

pub struct MyApp<'i, 'w> {
    info: &'i AppInfo<'w>,
    window: &'w Window,

    objects: Objects<'i, 'w>,
    drawer: Drawer,
}

impl<'i, 'w> MyApp<'i, 'w> {
    pub fn new(info: &'i AppInfo<'w>, window: &'w Window) -> Self {
        let drawer = Drawer::new(info);
        let mut objects = Objects::new(info);

        objects.pieces.transforms.push(
            &info.queue,
            make_piece_transform(0, 0, PieceType::Boat, PieceColor::White),
        );
        objects.pieces.transforms.push(
            &info.queue,
            make_piece_transform(-1, -1, PieceType::Boat, PieceColor::Black),
        );

        Self {
            info,
            window,

            drawer,
            objects,
        }
    }
}

impl App for MyApp<'_, '_> {
    fn on_resized(&mut self, _info: EventInfo, new_size: PhysicalSize) {
        self.info.resize_surface(new_size);
        self.window.request_redraw();
    }

    fn on_redraw_requested(&mut self, _info: EventInfo) {
        self.objects.scale(self.window.ratio());

        let mut frame = self.info.start_drawing();
        self.draw(&mut frame);
    }
}

impl MyApp<'_, '_> {
    fn draw(&mut self, frame: &mut Frame) {
        let mut render_pass =
            frame
                .commands
                .encoder()
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: frame.surface.view(),
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    ..Default::default()
                });

        self.objects.draw(&self.drawer, &mut render_pass);
    }
}
