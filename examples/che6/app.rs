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
use sww::*;
use winit::window::Window;

pub fn translation(x: i32, y: i32) -> Vec2 {
    vec2(x as _, y as _)
}

pub struct MyApp<'info, 'window> {
    info: &'info AppInfo<'window>,
    window: &'window Window,

    objects: Objects<'info, 'window>,
    drawer: Drawer,
}

impl<'info, 'window> MyApp<'info, 'window> {
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
}

impl App for MyApp<'_, '_> {
    fn on_resized(&mut self, _info: AppEventInfo, new_size: PhysicalSize) {
        let mut surface_config = self.info.surface_config.borrow_mut();
        surface_config.width = new_size.width.max(1);
        surface_config.height = new_size.height.max(1);

        self.info
            .surface
            .configure(&self.info.device, &surface_config);

        self.window.request_redraw();
    }

    fn on_redraw_requested(&mut self, _info: AppEventInfo) {
        self.objects.scale(self.window.ratio());

        let frame = self
            .info
            .surface
            .get_current_texture()
            .expect("failed to acquire next swapchain texture");
        let mut command_encoder = self.info.device.create_command_encoder(&Default::default());

        self.draw(&mut command_encoder, &frame);

        self.info.queue.submit(iter::once(command_encoder.finish()));
        frame.present();
    }
}

impl MyApp<'_, '_> {
    fn draw(&mut self, command_encoder: &mut wgpu::CommandEncoder, frame: &wgpu::SurfaceTexture) {
        let view = frame.texture.create_view(&Default::default());

        let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
}
