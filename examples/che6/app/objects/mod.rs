use crate::pieces::PieceColor;
use crate::pieces::PieceType;
use crate::pieces::PiecesSheet;
use crate::translation;
use crate::Drawer;
use std::io;
use sww::buffers::MutVecBuffer;
use sww::media::read_image;
use sww::shaders::mesh::Transform;
use sww::vec2;
use sww::window::RenderWindow;
use sww::Mat2;

mod pieces;
mod scalables;
mod tiles;

pub use pieces::*;
pub use scalables::*;
pub use tiles::*;

pub struct Objects<'w> {
    rw: &'w RenderWindow<'w>,
    pub scalables: Scalables,
    pub tiles: Tiles<'w>,
    pub pieces: Pieces<'w>,
}

impl<'w> Objects<'w> {
    pub fn new(rw: &'w RenderWindow<'w>) -> Self {
        let mut scalables = Scalables::default();

        let tiles = Tiles::new(rw, &mut scalables);

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

            Pieces::new(rw, &mut scalables, sheet, transforms)
        };

        Self {
            rw,
            scalables,
            tiles,
            pieces,
        }
    }

    pub fn scale(&mut self, ratio: f32) {
        let scale = 1_f32;
        let matrix =
            Mat2::from_diagonal(vec2((scale / ratio).min(scale), (scale * ratio).min(scale)));

        for &mut Scalable {
            ref mut transform_buffer,
            base_scale,
        } in &mut self.scalables
        {
            let mut transform = transform_buffer.value_mut(self.rw.queue());
            transform.matrix = matrix * Mat2::from_diagonal(base_scale);
        }
    }
}

impl<'e> Objects<'_> {
    pub fn draw(&'e self, drawer: &'e Drawer, render_pass: &mut wgpu::RenderPass<'e>) {
        self.tiles.draw(drawer, render_pass);
        self.pieces.draw(drawer, render_pass);
    }
}

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

    MutVecBuffer::new_vertex(rw, piece_transforms)
}
