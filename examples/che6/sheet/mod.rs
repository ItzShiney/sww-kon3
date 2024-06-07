use crate::translation;
use sww::shaders;
use sww::shaders::mesh::Transform;
use sww::Vec2;

mod coord;
mod piece_color;
mod piece_type;

pub use coord::*;
pub use piece_color::*;
pub use piece_type::*;

pub fn texture_rect<T: Coord<Output = Vec2>>(coord: T) -> shaders::mesh::Rectangle {
    let size = T::size();
    let top_left = coord.coord();

    shaders::mesh::Rectangle { top_left, size }
}

pub fn make_piece_transform(
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
