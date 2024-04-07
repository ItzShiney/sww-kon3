use std::io;
use std::iter;
use strum::EnumCount;
use strum_macros::EnumCount;
use sww::shaders::mesh::Transform;
use sww::*;
use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
use winit::event_loop::EventLoopWindowTarget;
use winit::window::Window;

////////////////////////////////////////////////////////////
// sheet/piece_type.rs
////////////////////////////////////////////////////////////
#[derive(Clone, Copy, EnumCount)]
enum PieceType {
    King,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    _PawnShadow,
    _Chariot,
    Boat,
    _Dragon,
    _Spy,
}

impl From<PieceType> for usize {
    fn from(value: PieceType) -> Self {
        value as _
    }
}

impl Coord for PieceType {
    type Output = f32;

    fn coord(self) -> Self::Output {
        self as usize as f32 * Self::size()
    }

    fn size() -> Self::Output {
        1. / Self::COUNT as f32
    }
}

////////////////////////////////////////////////////////////
// sheet/piece_color.rs
////////////////////////////////////////////////////////////
#[derive(Clone, Copy, EnumCount)]
enum PieceColor {
    White,
    Black,
}

impl From<PieceColor> for usize {
    fn from(value: PieceColor) -> Self {
        value as _
    }
}

impl Coord for PieceColor {
    type Output = f32;

    fn coord(self) -> Self::Output {
        self as usize as f32 * Self::size()
    }

    fn size() -> Self::Output {
        1. / Self::COUNT as f32
    }
}

////////////////////////////////////////////////////////////
// sheet/coord.rs
////////////////////////////////////////////////////////////
trait Coord {
    type Output;

    fn coord(self) -> Self::Output;
    fn size() -> Self::Output;
}

impl<A: Coord<Output = f32>, B: Coord<Output = f32>> Coord for (A, B) {
    type Output = Vec2;

    fn coord(self) -> Self::Output {
        vec2(self.0.coord(), self.1.coord())
    }

    fn size() -> Self::Output {
        vec2(A::size(), B::size())
    }
}

////////////////////////////////////////////////////////////
// sheet.rs
////////////////////////////////////////////////////////////
fn translation(x: i32, y: i32) -> Vec2 {
    vec2(x as _, y as _)
}

fn texture_rect<T: Coord<Output = Vec2>>(coord: T) -> shaders::mesh::Rectangle {
    let size = coord.coord();
    let top_left = T::size();

    shaders::mesh::Rectangle { top_left, size }
}

fn make_piece_transforms(app_info: &AppInfo) -> VecBuffer<Transform> {
    let mut piece_transforms = Vec::with_capacity(8 * 8);

    for (y, piece_color) in [(-3, PieceColor::White), (3 - 1, PieceColor::Black)] {
        for x in -4..4 {
            piece_transforms.push(make_piece_transform(x, y, PieceType::Pawn, piece_color));
        }
    }

    for (y, piece_color) in [(-4, PieceColor::White), (4 - 1, PieceColor::Black)] {
        for (pos, piece_type) in [
            (2, PieceType::Bishop),
            (3, PieceType::Knight),
            (4, PieceType::Rook),
        ] {
            for x in [-pos, pos - 1] {
                piece_transforms.push(make_piece_transform(x, y, piece_type, piece_color));
            }
        }

        piece_transforms.push(make_piece_transform(-1, y, PieceType::Queen, piece_color));
        piece_transforms.push(make_piece_transform(0, y, PieceType::King, piece_color));
    }

    app_info.vec_buffer_vertex(piece_transforms)
}

////////////////////////////////////////////////////////////
// app/objects/tiles/single_color.rs
////////////////////////////////////////////////////////////
struct SingleColorTiles {
    transforms: VecBuffer<Transform>,
    bind_group0: shaders::mesh::BindGroup0,
}

impl SingleColorTiles {
    fn new(
        app_info: &AppInfo,
        scalers: &mut Scalers,
        color: Color,
        transforms: VecBuffer<Transform>,
    ) -> Self {
        let global_transform = scalers.push_last(ReadableBuffer::new(
            &app_info.device,
            Transform {
                color: color.into(),
                ..Default::default()
            },
        ));

        let bind_group0 = {
            let global_transform = global_transform.buffer().binding();
            shaders::mesh::BindGroup0::from_bindings(&app_info.device, global_transform.into())
        };

        Self {
            transforms,
            bind_group0,
        }
    }

    fn draw<'s>(
        &'s self,
        drawer: &'s Drawer,
        render_pass: &mut wgpu::RenderPass<'s>,
        bind_group1: &'s shaders::mesh::BindGroup1,
    ) {
        drawer.draw_squares(
            render_pass,
            self.transforms.slice(..),
            &shaders::mesh::BindGroups {
                bind_group0: &self.bind_group0,
                bind_group1,
            },
        );
    }
}

////////////////////////////////////////////////////////////
// app/objects/tiles.rs
////////////////////////////////////////////////////////////
fn make_white_black_tranforms(app_info: &AppInfo) -> (VecBuffer<Transform>, VecBuffer<Transform>) {
    let mut white = Vec::default();
    let mut black = Vec::default();

    for y in -4..4_i32 {
        for x in -4..4_i32 {
            let translation = translation(x, y);
            let colored_transforms = if (x + y).rem_euclid(2) == 0 {
                &mut black
            } else {
                &mut white
            };

            colored_transforms.push(Transform {
                translation,
                ..Default::default()
            });
        }
    }

    (
        app_info.vec_buffer_vertex(white),
        app_info.vec_buffer_vertex(black),
    )
}

struct Tiles {
    white: SingleColorTiles,
    black: SingleColorTiles,
    bind_group1: shaders::mesh::BindGroup1,
}

impl Tiles {
    fn new(app_info: &AppInfo, scalers: &mut Scalers) -> Self {
        let (white_transforms, black_transforms) = make_white_black_tranforms(app_info);
        let white = SingleColorTiles::new(app_info, scalers, Color::splat(0.45), white_transforms);
        let black = SingleColorTiles::new(app_info, scalers, Color::splat(0.25), black_transforms);

        let bind_group1 = {
            let default_texture = make_default_texture(&app_info.device, &app_info.queue);
            let default_texture_view = default_texture.create_view(&Default::default());

            shaders::mesh::BindGroup1::from_bindings(
                &app_info.device,
                shaders::mesh::BindGroupLayout1 {
                    texture: &default_texture_view,
                },
            )
        };

        Self {
            white,
            black,
            bind_group1,
        }
    }

    fn draw<'s>(&'s self, drawer: &'s Drawer, render_pass: &mut wgpu::RenderPass<'s>) {
        self.white.draw(drawer, render_pass, &self.bind_group1);
        self.black.draw(drawer, render_pass, &self.bind_group1);
    }
}

////////////////////////////////////////////////////////////
// app/objects/pieces.rs
////////////////////////////////////////////////////////////
fn make_piece_transform(
    x: i32,
    y: i32,
    piece_type: PieceType,
    piece_color: PieceColor,
) -> Transform {
    let translation = translation(x, y);
    let texture_rect = texture_rect((piece_type, piece_color));

    Transform {
        translation,
        texture_rect,
        ..Default::default()
    }
}

struct Pieces {
    transforms: VecBuffer<Transform>,
    bind_group0: shaders::mesh::BindGroup0,
    bind_group1: shaders::mesh::BindGroup1,
}

impl Pieces {
    fn new(app_info: &AppInfo, scalers: &mut Scalers, transforms: VecBuffer<Transform>) -> Self {
        let global_transform =
            scalers.push_last(ReadableBuffer::new(&app_info.device, Transform::default()));

        let texture = read_texture(
            &app_info.device,
            &app_info.queue,
            io::Cursor::new(include_bytes!("pieces.png")),
        );
        let texture_view = texture.default_view();

        let bind_group0 = shaders::mesh::BindGroup0::from_bindings(
            &app_info.device,
            global_transform.buffer().binding().into(),
        );

        let bind_group1 = shaders::mesh::BindGroup1::from_bindings(
            &app_info.device,
            shaders::mesh::BindGroupLayout1 {
                texture: &texture_view,
            },
        );

        Self {
            transforms,
            bind_group0,
            bind_group1,
        }
    }

    fn draw<'s>(&'s self, drawer: &'s Drawer, render_pass: &mut wgpu::RenderPass<'s>) {
        drawer.draw_squares(
            render_pass,
            self.transforms.slice(..),
            &shaders::mesh::BindGroups {
                bind_group0: &self.bind_group0,
                bind_group1: &self.bind_group1,
            },
        );
    }
}

////////////////////////////////////////////////////////////
// app/objects.rs
////////////////////////////////////////////////////////////
type Scaler = ReadableBuffer<Transform>;
type Scalers = Vec<Scaler>;

struct Objects<'info, 'window> {
    app_info: &'info AppInfo<'window>,
    scalers: Scalers,
    tiles: Tiles,
    pieces: Pieces,
}

impl<'info, 'window> Objects<'info, 'window> {
    fn new(app_info: &'info AppInfo<'window>) -> Self {
        let mut scalers = Scalers::default();

        let tiles = Tiles::new(app_info, &mut scalers);
        let pieces = Pieces::new(app_info, &mut scalers, make_piece_transforms(app_info));

        Self {
            app_info,
            scalers,
            tiles,
            pieces,
        }
    }

    fn scale(&mut self, ratio: f32) {
        let scale = 1. / 4_f32;
        let matrix = Mat2::from_diagonal(vec2(scale.min(scale / ratio), scale.min(scale * ratio)));

        for transform_buffer in self.scalers.iter_mut() {
            let mut transform = transform_buffer.value_mut(&self.app_info.queue);
            transform.matrix = matrix;
        }
    }

    fn draw<'s>(&'s self, drawer: &'s Drawer, render_pass: &mut wgpu::RenderPass<'s>) {
        self.tiles.draw(drawer, render_pass);
        self.pieces.draw(drawer, render_pass);
    }
}

////////////////////////////////////////////////////////////
// app/drawer.rs
////////////////////////////////////////////////////////////
struct Drawer {
    mesh_drawer: MeshDrawer,
    square: Mesh,
}

impl Drawer {
    fn new(info: &AppInfo) -> Self {
        let mesh_drawer = info.mesh_drawer();
        let square = info.mesh_rect(vec2(1., 1.));

        Self {
            mesh_drawer,
            square,
        }
    }

    fn draw_squares<'s>(
        &'s self,
        render_pass: &mut wgpu::RenderPass<'s>,
        transforms: VecBufferSlice<'s, Transform>,
        bind_groups: &shaders::mesh::BindGroups<'s>,
    ) {
        self.mesh_drawer
            .draw(render_pass, &self.square, transforms, bind_groups)
    }
}

////////////////////////////////////////////////////////////
// app.rs
////////////////////////////////////////////////////////////
struct App<'info, 'window> {
    info: &'info AppInfo<'window>,
    window: &'window Window,

    objects: Objects<'info, 'window>,
    drawer: Drawer,
}

impl<'info, 'window> App<'info, 'window> {
    fn new(info: &'info AppInfo<'window>, window: &'window Window) -> Self {
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

    fn event_handler(&mut self, event: Event<()>, target: &EventLoopWindowTarget<()>) {
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
}

////////////////////////////////////////////////////////////
// main.rs
////////////////////////////////////////////////////////////
pub fn main() {
    let event_loop = EventLoop::new().unwrap();

    let window = winit::window::WindowBuilder::new()
        .with_title("che6")
        .with_inner_size(winit::dpi::PhysicalSize::new(400, 200))
        .build(&event_loop)
        .unwrap();

    env_logger::init();

    let app_info = AppInfo::new(&window, &DefaultAppSettings);
    let mut app = App::new(&app_info, &window);

    event_loop
        .run(|event, target| app.event_handler(event, target))
        .unwrap();
}
