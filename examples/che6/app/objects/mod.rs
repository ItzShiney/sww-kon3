use crate::sheet::make_piece_transform;
use crate::sheet::PieceColor;
use crate::sheet::PieceType;
use crate::Drawer;
use crate::Pieces;
use sww::shaders::mesh::Transform;
use sww::vec2;
use sww::window::RenderWindow;
use sww::Mat2;
use sww::ReadableBuffer;
use sww::VecBuffer;

mod tiles;

pub use tiles::*;

pub type Scaler = ReadableBuffer<Transform>;
pub type Scalers = Vec<Scaler>;

fn make_piece_transforms<'w>(rw: &'w RenderWindow) -> VecBuffer<'w, Transform> {
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

    rw.vec_buffer_vertex(piece_transforms)
}

pub struct Objects<'w> {
    rw: &'w RenderWindow<'w>,
    pub scalers: Scalers,
    pub tiles: Tiles<'w>,
    pub pieces: Pieces<'w>,
}

impl<'w> Objects<'w> {
    pub fn new(rw: &'w RenderWindow<'w>) -> Self {
        let mut scalers = Scalers::default();

        let tiles = Tiles::new(rw, &mut scalers);
        let pieces = Pieces::new(rw, &mut scalers, make_piece_transforms(rw));

        Self {
            rw,
            scalers,
            tiles,
            pieces,
        }
    }

    pub fn scale(&mut self, ratio: f32) {
        let scale = 1. / 4_f32;
        let matrix = Mat2::from_diagonal(vec2(scale.min(scale / ratio), scale.min(scale * ratio)));

        for transform_buffer in self.scalers.iter_mut() {
            let mut transform = transform_buffer.value_mut(self.rw.queue());
            transform.matrix = matrix;
        }
    }
}

impl<'c> Objects<'_> {
    pub fn draw(&'c self, drawer: &'c Drawer, render_pass: &mut wgpu::RenderPass<'c>) {
        self.tiles.draw(drawer, render_pass);
        self.pieces.draw(drawer, render_pass);
    }
}
