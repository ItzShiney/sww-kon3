use std::io::{
    BufRead,
    Seek,
};

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
    image::EncodableLayout,
    shaders::mesh::Transform,
    std::{
        io,
        iter,
    },
    wgpu::util::DeviceExt,
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

    let mut white_global_transform = ReadableBuffer::new(
        &device,
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
                buffer: white_global_transform.buffer(),
                offset: 0,
                size: None,
            },
        },
    );

    let black_instances = Instances::new(&device, &black_transforms);

    let mut black_global_transform = ReadableBuffer::new(
        &device,
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
                buffer: black_global_transform.buffer(),
                offset: 0,
                size: None,
            },
        },
    );

    #[allow(unused)]
    let texture_rect = ReadableBuffer::new(
        &device,
        shaders::mesh::Rectangle {
            start: vec2(0., 0.),
            end: vec2(1., 1.),
        },
    );

    fn read_texture(reader: impl BufRead + Seek) -> image::RgbaImage {
        image::io::Reader::new(reader)
            .with_guessed_format()
            .expect("failed to guess texture format")
            .decode()
            .expect("failed to decode texture")
            .into_rgba8()
    }

    let texture = read_texture(io::Cursor::new(include_bytes!("1x1.png")));
    let texture = device.create_texture_with_data(
        &queue,
        &wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: texture.width(),
                height: texture.height(),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        },
        wgpu::util::TextureDataOrder::MipMajor,
        texture.as_bytes(),
    );

    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor {
        label: None,
        format: None,
        dimension: None,
        aspect: wgpu::TextureAspect::All,
        base_mip_level: 0,
        mip_level_count: None,
        base_array_layer: 0,
        array_layer_count: None,
    });

    let bind_group1 = shaders::mesh::bind_groups::BindGroup1::from_bindings(
        &device,
        shaders::mesh::bind_groups::BindGroupLayout1 {
            texture_rect: wgpu::BufferBinding {
                buffer: &texture_rect.buffer(),
                offset: 0,
                size: None,
            },
            texture: &texture_view,
        },
    );

    #[allow(clippy::single_match)]
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
                            &shaders::mesh::bind_groups::BindGroups {
                                bind_group0: &white_bind_group0,
                                bind_group1: &bind_group1,
                            },
                        );
                        mesh_drawer.draw(
                            &mut render_pass,
                            &square,
                            &black_instances,
                            &shaders::mesh::bind_groups::BindGroups {
                                bind_group0: &black_bind_group0,
                                bind_group1: &bind_group1,
                            },
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
