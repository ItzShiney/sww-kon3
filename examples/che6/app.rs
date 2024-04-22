mod drawer;
mod objects;
mod pieces;

use crate::sheet::make_piece_transform;
use crate::sheet::PieceColor;
use crate::sheet::PieceType;
pub use drawer::*;
pub use objects::*;
pub use pieces::*;
use std::iter;
use sww::vec2;
use sww::AppInfo;
use sww::EventLoop;
use sww::Ratio;
use sww::Vec2;
use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::EventLoopWindowTarget;
use winit::window::Window;

pub fn translation(x: i32, y: i32) -> Vec2 {
    vec2(x as _, y as _)
}

pub struct App<'info, 'window> {
    info: &'info AppInfo<'window>,
    window: &'window Window,

    objects: Objects<'info, 'window>,
    drawer: Drawer,
}

impl<'info, 'window> App<'info, 'window> {
    pub fn new(info: &'info AppInfo<'window>, window: &'window Window) -> Self {
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

    pub fn event_handler(&mut self, event: Event<()>, target: &EventLoopWindowTarget<()>) {
        #[allow(clippy::single_match)]
        match event {
            Event::WindowEvent {
                window_id: _,
                event,
            } => match event {
                WindowEvent::Resized(new_size) => {
                    let mut surface_config = self.info.surface_config.borrow_mut();
                    surface_config.width = new_size.width.max(1);
                    surface_config.height = new_size.height.max(1);

                    self.info
                        .surface
                        .configure(&self.info.device, &surface_config);

                    self.window.request_redraw();
                }

                WindowEvent::RedrawRequested => {
                    self.objects.scale(self.window.ratio());

                    let frame = self
                        .info
                        .surface
                        .get_current_texture()
                        .expect("failed to acquire next swapchain texture");

                    let view = frame.texture.create_view(&Default::default());
                    let mut command_encoder =
                        self.info.device.create_command_encoder(&Default::default());

                    {
                        let mut render_pass =
                            command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                    view: &view,
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

                    self.info.queue.submit(iter::once(command_encoder.finish()));
                    frame.present();
                }

                WindowEvent::CloseRequested => target.exit(),

                _ => {}
            },

            _ => {}
        }
    }

    pub fn run(&mut self, event_loop: EventLoop) -> Result<(), winit::error::EventLoopError> {
        event_loop.run(|event, target| self.event_handler(event, target))
    }
}
