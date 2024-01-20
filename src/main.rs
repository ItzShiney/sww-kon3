mod bytes;
mod color;
mod instances;
mod mesh;
mod mesh_drawer;
mod readable_buffer;
pub mod shaders;

pub use {
    bytes::*,
    color::*,
    instances::*,
    mesh::*,
    mesh_drawer::*,
    readable_buffer::*,
};
use {
    glam::{
        vec2,
        Mat2,
    },
    shaders::mesh::Transform,
    std::{
        iter,
        mem,
    },
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
                required_features: wgpu::Features::default(),
                required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
            },
            None,
        )
        .await
        .expect("failed to create device");

    surface.configure(&device, &config);

    let mesh_drawer = MeshDrawer::new(&device, swapchain_format);
    let square = Mesh::rect(&device, vec2(1., 1.));

    let mut white_transforms = Vec::default();
    let mut black_transforms = Vec::default();
    {
        for y in -4..4_i32 {
            for x in -4..4_i32 {
                let translation = vec2(x as f32, y as f32);
                let colored_transforms = if (x + y).rem_euclid(2) == 0 {
                    &mut black_transforms
                } else {
                    &mut white_transforms
                };

                colored_transforms.push(Transform {
                    matrix: Default::default(),
                    translation,
                    color: Color::WHITE.into(),
                });
            }
        }
    }

    let white_instances = Instances::new(&device, &white_transforms);

    let white_global_transform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: mem::size_of::<shaders::mesh::Transform>() as _,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let mut white_global_transform = ReadableBuffer::new(
        &white_global_transform_buffer,
        Transform {
            matrix: Default::default(),
            translation: Default::default(),
            color: Color::splat(0.7).into(),
        },
    );

    let white_bind_group0 = shaders::mesh::bind_groups::BindGroup0::from_bindings(
        &device,
        shaders::mesh::bind_groups::BindGroupLayout0 {
            global_transform: wgpu::BufferBinding {
                buffer: &white_global_transform_buffer,
                offset: 0,
                size: None,
            },
        },
    );

    let black_instances = Instances::new(&device, &black_transforms);

    let black_global_transform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: mem::size_of::<shaders::mesh::Transform>() as _,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let mut black_global_transform = ReadableBuffer::new(
        &black_global_transform_buffer,
        Transform {
            matrix: Default::default(),
            translation: Default::default(),
            color: Color::splat(0.3).into(),
        },
    );

    let black_bind_group0 = shaders::mesh::bind_groups::BindGroup0::from_bindings(
        &device,
        shaders::mesh::bind_groups::BindGroupLayout0 {
            global_transform: wgpu::BufferBinding {
                buffer: &black_global_transform_buffer,
                offset: 0,
                size: None,
            },
        },
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
                    {
                        let ratio = {
                            let size = window.inner_size();
                            size.width as f32 / size.height as f32
                        };

                        let scale = 1. / 4_f32;
                        let matrix = Mat2::from_diagonal(vec2(
                            scale.min(scale / ratio),
                            scale.min(scale * ratio),
                        ));

                        for transform_buffer in
                            [&mut white_global_transform, &mut black_global_transform]
                        {
                            let mut transform = *transform_buffer.value();
                            transform.matrix = matrix;
                            transform_buffer.write(&queue, transform);
                        }
                    }

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

                        mesh_drawer.draw(
                            &mut render_pass,
                            &square,
                            &white_instances,
                            &white_bind_group0,
                        );
                        mesh_drawer.draw(
                            &mut render_pass,
                            &square,
                            &black_instances,
                            &black_bind_group0,
                        );
                    }

                    queue.submit(iter::once(command_encoder.finish()));
                    frame.present();
                }

                WindowEvent::CloseRequested => target.exit(),

                _ => {}
            },

            _ => {}
        })
        .unwrap();
}
