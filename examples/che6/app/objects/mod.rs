use crate::pieces::PieceColor;
use crate::pieces::PieceType;
use crate::pieces::PiecesSheet;
use crate::translation;
use crate::Drawer;
use std::io;
use sww::prelude::*;

mod pieces;
mod tiles;

pub use pieces::*;
pub use tiles::*;

pub type Scaler = MutBuffer<Transform>;
pub type Scalers = Vec<Scaler>;

fn make_piece_transforms<'w>(
    rw: &'w RenderWindow,
    sheet: &PiecesSheet,
) -> MutVecBuffer<'w, Transform> {
    let mut piece_transforms = Vec::with_capacity(8 * 8);

    for (y, piece_color) in [(-3, PieceColor::White), (3 - 1, PieceColor::Black)] {
        for x in -4..4 {
            piece_transforms.push(make_piece_transform(
                sheet,
                translation(x, y),
                (PieceType::Pawn, piece_color),
            ));
        }
    }

    for (y, piece_color) in [(-4, PieceColor::White), (4 - 1, PieceColor::Black)] {
        for (pos, piece_type) in [
            (2, PieceType::Bishop),
            (3, PieceType::Knight),
            (4, PieceType::Rook),
        ] {
            for x in [-pos, pos - 1] {
                piece_transforms.push(make_piece_transform(
                    sheet,
                    translation(x, y),
                    (piece_type, piece_color),
                ));
            }
        }

        piece_transforms.push(make_piece_transform(
            sheet,
            translation(-1, y),
            (PieceType::Queen, piece_color),
        ));
        piece_transforms.push(make_piece_transform(
            sheet,
            translation(0, y),
            (PieceType::King, piece_color),
        ));
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

        let pieces = {
            let sheet = PiecesSheet::new(
                rw,
                read_image(io::Cursor::new(include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/res/pieces.png"
                ))))
                .unwrap(),
            );
            let transforms = make_piece_transforms(rw, &sheet);

            Pieces::new(rw, &mut scalers, sheet, transforms)
        };

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
