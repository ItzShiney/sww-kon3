use super::Coord;
use strum::EnumCount;
use strum_macros::EnumCount;

#[derive(Clone, Copy, EnumCount)]
pub enum PieceType {
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
