mod polygon;
mod polygon_drawer;
mod vertex;
mod vertex_attribute;
mod vertex_attributes;

use winit::{
    event::{
        Event,
        WindowEvent,
    },
    event_loop::EventLoop,
    window::Window,
};
pub use {
    polygon::*,
    polygon_drawer::*,
    vertex::*,
    vertex_attribute::*,
    vertex_attributes::*,
};

pub fn main() {
    let event_loop = EventLoop::new().unwrap();

    let window = winit::window::WindowBuilder::new()
        .with_title("wgpu")
        .build(&event_loop)
        .unwrap();

    env_logger::init();
    pollster::block_on(run(event_loop, window));
}

async fn run(event_loop: EventLoop<()>, window: Window) {
    let mut size = window.inner_size();
    size.width = size.width.max(1);
    size.height = size.height.max(1);

    let instance = wgpu::Instance::default();

    let surface = unsafe { instance.create_surface(&window) }.unwrap();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: Default::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .expect("failed to find an adapter");

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
            },
            None,
        )
        .await
        .expect("failed to create device");

    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];

    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: size.width,
        height: size.height,
        present_mode: Default::default(),
        alpha_mode: swapchain_capabilities.alpha_modes[0],
        view_formats: vec![],
    };

    surface.configure(&device, &config);

    let color1 = [0.3, 0.5, 0.7, 1.];
    let color2 = [0.7, 0.1, 0.9, 1.];
    let size = 0.5;

    let polygon1 = Polygon::new_indexed(
        &device,
        &[
            Vertex::new([-size, -size], color1),
            Vertex::new([-size, size], color1),
            Vertex::new([size, size], color2),
            Vertex::new([size, -size], color1),
        ],
        &[0, 1, 2, 0, 2, 3],
    );
    let polygon2 = Polygon::new_indexed(
        &device,
        &[
            Vertex::new([0., -0.1], color2),
            Vertex::new([-0.3, 0.7], color2),
            Vertex::new([0., 0.6], color2),
            Vertex::new([0.3, -0.2], color2),
        ],
        &[0, 1, 2, 0, 2, 3],
    );
    let polygon_drawer = PolygonDrawer::new(&device, swapchain_format);

    event_loop
        .run(move |event, target| {
            if let Event::WindowEvent {
                window_id: _,
                event,
            } = event
            {
                match event {
                    WindowEvent::Resized(new_size) => {
                        config.width = new_size.width.max(1);
                        config.height = new_size.height.max(1);
                        surface.configure(&device, &config);

                        window.request_redraw();
                    }

                    WindowEvent::RedrawRequested => {
                        let frame = surface
                            .get_current_texture()
                            .expect("failed to acquire next swapchain texture");

                        let view = frame.texture.create_view(&Default::default());
                        let mut command_encoder =
                            device.create_command_encoder(&Default::default());

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

                            polygon_drawer.draw(&mut render_pass, &polygon1);
                            polygon_drawer.draw(&mut render_pass, &polygon2);
                        }

                        queue.submit(Some(command_encoder.finish()));
                        frame.present();
                    }

                    WindowEvent::CloseRequested => target.exit(),

                    _ => {}
                };
            }
        })
        .unwrap();
}
