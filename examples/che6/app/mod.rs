use crate::pieces::PieceColor;
use crate::pieces::PieceType;
use event::*;
use sww::app::EventInfo;
use sww::app::HandleEvent;
use sww::vec2;
use sww::window::*;
use sww::Vec2;

mod drawer;
mod objects;

pub use drawer::*;
pub use objects::*;

pub fn translation(x: i32, y: i32) -> Vec2 {
    vec2(x as _, y as _)
}

pub struct MyApp<'w> {
    rw: &'w RenderWindow<'w>,
    objects: Objects<'w>,
    drawer: Drawer,
}

impl<'w> MyApp<'w> {
    pub fn new(rw: &'w RenderWindow<'w>) -> Self {
        let drawer = Drawer::new(rw);
        let mut objects = Objects::new(rw);

        objects.pieces.transforms.push(make_piece_transform(
            objects.pieces.sheet(),
            translation(0, 0),
            (PieceType::Boat, PieceColor::White),
        ));
        objects.pieces.transforms.push(make_piece_transform(
            objects.pieces.sheet(),
            translation(-1, -1),
            (PieceType::Boat, PieceColor::Black),
        ));

        Self {
            rw,
            drawer,
            objects,
        }
    }
}

impl HandleEvent for MyApp<'_> {
    fn on_resized(&mut self, _info: EventInfo, new_size: PhysicalSize) {
        self.rw.resize_surface(new_size);
        self.rw.window().request_redraw();
    }

    fn on_redraw_requested(&mut self, _info: EventInfo) {
        let Ok(mut frame) = self.rw.start_drawing() else {
            return;
        };

        self.objects.scale(self.rw.window().ratio());
        self.draw(&mut frame);
    }
}

impl<'c> MyApp<'_> {
    fn draw(&mut self, frame: &mut Frame<'c>) {
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
