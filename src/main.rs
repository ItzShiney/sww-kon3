mod affine;
mod bytes;
mod color;
mod field_attributes;
mod instances;
mod mesh;
mod mesh_drawer;
mod transform;
mod vertex;
mod vertex_attributes;

pub use {
    affine::*,
    bytes::*,
    color::*,
    field_attributes::*,
    instances::*,
    mesh::*,
    mesh_drawer::*,
    transform::*,
    vertex::*,
    vertex_attributes::*,
};
use {
    glam::vec2,
    winit::{
        event::{
            Event,
            WindowEvent,
        },
        event_loop::EventLoop,
        window::Window,
    },
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

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());

    let surface = instance.create_surface(&window).unwrap();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .expect("failed to find an adapter");

    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];

    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Immediate,
        desired_maximum_frame_latency: 2,
        alpha_mode: swapchain_capabilities.alpha_modes[0],
        view_formats: Vec::default(),
    };

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::MAPPABLE_PRIMARY_BUFFERS,
                required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
            },
            None,
        )
        .await
        .expect("failed to create device");

    surface.configure(&device, &config);

    let mesh_drawer = MeshDrawer::new(&device, swapchain_format);

    let mut transforms = Vec::default();
    {
        let white = Color::splat(0.7);
        let black = Color::splat(0.3);
        let side = 2. / 8.;
        let size = vec2(side, side);

        for y in -4..4_i32 {
            for x in -4..4_i32 {
                let translation = vec2(x as f32, y as f32) * size;
                let color = [black, white][(x + y).rem_euclid(2) as usize];

                let affine_scale = glam::Affine2::from_scale(size);
                let affine_translation = glam::Affine2::from_translation(translation);
                let affine = affine_translation * affine_scale;

                transforms.push(Transform::new(affine, color));
            }
        }
    }

    let square = Mesh::rect(&device, vec2(1., 1.));
    let white_instances = Instances::new(
        &device,
        Affine::from_scale_splat(1. / 4.).into(),
        &transforms,
    );

    event_loop
        .run(|event, target| match event {
            Event::WindowEvent {
                window_id: _,
                event,
            } => match event {
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
                    let mut command_encoder = device.create_command_encoder(&Default::default());

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

                        mesh_drawer.draw(&mut render_pass, &square, &white_instances);
                    }

                    queue.submit(Some(command_encoder.finish()));
                    frame.present();
                }

                WindowEvent::CloseRequested => target.exit(),

                _ => {}
            },

            _ => {}
        })
        .unwrap();
}
