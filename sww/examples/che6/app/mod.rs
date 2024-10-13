use crate::pieces::PieceColor;
use crate::pieces::PieceType;
use event::*;
use std::sync::Arc;
use sww::app::EventHandler as SwwEventHandler;
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

pub struct EventHandler {
    rw: Arc<RenderWindow>,
    objects: Objects,
    drawer: Drawer,
}

impl EventHandler {
    pub fn new(rw: &Arc<RenderWindow>) -> Self {
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
            rw: Arc::clone(rw),
            drawer,
            objects,
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let (commands, surface) = frame.commands_surface();
        let mut render_pass = (commands.encoder()).begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: surface.view(),
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

impl SwwEventHandler for EventHandler {
    fn handle_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::Resized(new_size) => {
                self.rw.resize_surface(new_size);
                self.rw.window().request_redraw();
            }

            WindowEvent::RedrawRequested => {
                let mut frame = self.rw.start_drawing();

                self.objects.scale(self.rw.window().ratio());
                self.draw(&mut frame);
            }

            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        _event: DeviceEvent,
    ) {
    }
}
