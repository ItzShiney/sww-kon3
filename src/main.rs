mod app_info;
mod app_settings;
mod bytes;
mod color;
mod images;
mod mesh;
mod mesh_drawer;
mod readable_buffer;
pub mod shaders;
mod vec_buffer;

pub use {
    app_info::*,
    app_settings::*,
    bytes::*,
    color::*,
    images::*,
    mesh::*,
    mesh_drawer::*,
    readable_buffer::*,
    vec_buffer::*,
};
use {
    glam::{
        vec2,
        Mat2,
    },
    shaders::mesh::Transform,
    std::{
        io,
        iter,
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
        .with_title("sww")
        .with_inner_size(winit::dpi::PhysicalSize::new(400, 200))
        .build(&event_loop)
        .unwrap();

    env_logger::init();

    init(window, event_loop);
}

fn init(window: Window, event_loop: EventLoop<()>) {
    let mut app_info = AppInfo::new(&window, &DefaultAppSettings);

    let mesh_drawer = MeshDrawer::new(&app_info.device, app_info.swapchain_format);
    let square = Mesh::rect(&app_info.device, vec2(1., 1.));

    let (white_transforms, black_transforms) = {
        let mut white = Vec::default();
        let mut black = Vec::default();

        for y in -4..4_i32 {
            for x in -4..4_i32 {
                let translation = vec2(x as f32, y as f32);
                let colored_transforms = if (x + y).rem_euclid(2) == 0 {
                    &mut black
                } else {
                    &mut white
                };

                colored_transforms.push(Transform {
                    translation,
                    color: Color::WHITE.into(),
                    ..Default::default()
                });
            }
        }

        (
            VecBuffer::new(&app_info.device, white, wgpu::BufferUsages::VERTEX),
            VecBuffer::new(&app_info.device, black, wgpu::BufferUsages::VERTEX),
        )
    };

    fn make_piece_transform(
        x: i32,
        y: i32,
        piece_type: u32,
        is_white: bool,
    ) -> shaders::mesh::Transform {
        const PIECE_TYPES_COUNT: u32 = 11;
        const COLORS_COUNT: u32 = 2;

        let texture_rect_y = !is_white as u32 as f32 / COLORS_COUNT as f32;

        shaders::mesh::Transform {
            translation: vec2(x as f32, y as f32),
            texture_rect: shaders::mesh::Rectangle {
                top_left: vec2(piece_type as f32 / PIECE_TYPES_COUNT as f32, texture_rect_y),
                size: vec2(1. / PIECE_TYPES_COUNT as f32, 1. / COLORS_COUNT as f32),
            },
            ..Default::default()
        }
    }

    let mut piece_transforms = {
        let mut piece_transforms = Vec::default();
        piece_transforms.reserve(8 * 8);

        for (y, is_white) in [(-3, true), (3 - 1, false)] {
            for x in -4..4 {
                piece_transforms.push(make_piece_transform(x, y, 1, is_white));
            }
        }

        for (y, is_white) in [(-4, true), (4 - 1, false)] {
            for (pos, piece_type) in [(4, 4), (3, 2), (2, 3)] {
                for x in [-pos, pos - 1] {
                    piece_transforms.push(make_piece_transform(x, y, piece_type, is_white));
                }
            }

            piece_transforms.push(make_piece_transform(-1, y, 5, is_white));
            piece_transforms.push(make_piece_transform(0, y, 0, is_white));
        }

        VecBuffer::new(
            &app_info.device,
            piece_transforms,
            wgpu::BufferUsages::VERTEX,
        )
    };

    piece_transforms.push(&app_info.queue, make_piece_transform(0, 0, 8, true));
    piece_transforms.push(&app_info.queue, make_piece_transform(-1, -1, 8, false));

    let mut white_global_transform = ReadableBuffer::new(
        &app_info.device,
        Transform {
            color: Color::splat(0.45).into(),
            ..Default::default()
        },
    );

    let white_bind_group0 = shaders::mesh::bind_groups::BindGroup0::from_bindings(
        &app_info.device,
        shaders::mesh::bind_groups::BindGroupLayout0 {
            global_transform: wgpu::BufferBinding {
                buffer: white_global_transform.buffer(),
                offset: 0,
                size: None,
            },
        },
    );

    let mut black_global_transform = ReadableBuffer::new(
        &app_info.device,
        Transform {
            color: Color::splat(0.25).into(),
            ..Default::default()
        },
    );

    let black_bind_group0 = shaders::mesh::bind_groups::BindGroup0::from_bindings(
        &app_info.device,
        shaders::mesh::bind_groups::BindGroupLayout0 {
            global_transform: wgpu::BufferBinding {
                buffer: black_global_transform.buffer(),
                offset: 0,
                size: None,
            },
        },
    );

    let default_texture = make_default_texture(&app_info.device, &app_info.queue);
    let default_texture_view = default_texture.create_view(&Default::default());

    let bind_group1 = shaders::mesh::bind_groups::BindGroup1::from_bindings(
        &app_info.device,
        shaders::mesh::bind_groups::BindGroupLayout1 {
            texture: &default_texture_view,
        },
    );

    let pieces_texture = read_texture(
        &app_info.device,
        &app_info.queue,
        io::Cursor::new(include_bytes!("../assets/pieces.png")),
    );
    let pieces_texture_view = pieces_texture.create_view(&Default::default());

    let mut pieces_global_transform = ReadableBuffer::new(&app_info.device, Transform::default());

    let pieces_bind_group0 = shaders::mesh::bind_groups::BindGroup0::from_bindings(
        &app_info.device,
        shaders::mesh::bind_groups::BindGroupLayout0 {
            global_transform: wgpu::BufferBinding {
                buffer: pieces_global_transform.buffer(),
                offset: 0,
                size: None,
            },
        },
    );

    let pieces_bind_group1 = shaders::mesh::bind_groups::BindGroup1::from_bindings(
        &app_info.device,
        shaders::mesh::bind_groups::BindGroupLayout1 {
            texture: &pieces_texture_view,
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
                    app_info.surface_config.width = new_size.width.max(1);
                    app_info.surface_config.height = new_size.height.max(1);
                    app_info
                        .surface
                        .configure(&app_info.device, &app_info.surface_config);

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

                        for transform_buffer in [
                            &mut white_global_transform,
                            &mut black_global_transform,
                            &mut pieces_global_transform,
                        ] {
                            let mut transform = transform_buffer.value_mut(&app_info.queue);
                            transform.matrix = matrix;
                        }
                    }

                    let frame = app_info
                        .surface
                        .get_current_texture()
                        .expect("failed to acquire next swapchain texture");

                    let view = frame.texture.create_view(&Default::default());
                    let mut command_encoder =
                        app_info.device.create_command_encoder(&Default::default());

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
                            white_transforms.slice(..),
                            &shaders::mesh::bind_groups::BindGroups {
                                bind_group0: &white_bind_group0,
                                bind_group1: &bind_group1,
                            },
                        );
                        mesh_drawer.draw(
                            &mut render_pass,
                            &square,
                            black_transforms.slice(..),
                            &shaders::mesh::bind_groups::BindGroups {
                                bind_group0: &black_bind_group0,
                                bind_group1: &bind_group1,
                            },
                        );
                        mesh_drawer.draw(
                            &mut render_pass,
                            &square,
                            piece_transforms.slice(..),
                            &shaders::mesh::bind_groups::BindGroups {
                                bind_group0: &pieces_bind_group0,
                                bind_group1: &pieces_bind_group1,
                            },
                        );
                    }

                    app_info.queue.submit(iter::once(command_encoder.finish()));
                    frame.present();
                }

                WindowEvent::CloseRequested => target.exit(),

                _ => {}
            },

            _ => {}
        })
        .unwrap();
}
