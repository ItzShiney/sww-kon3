use strum::EnumCount;
use strum_macros::EnumCount;
use sww::media::SheetCoord;

#[derive(Clone, Copy, EnumCount)]
pub enum PieceType {
    King,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    PawnShadow,
    Chariot,
    Boat,
    Dragon,
    Spy,
}

impl From<PieceType> for usize {
    fn from(value: PieceType) -> Self {
        value as _
    }
}

impl SheetCoord for PieceType {
    type Output = f32;

    fn coord(self) -> Self::Output {
        self as usize as f32 * Self::size()
    }

    fn size() -> Self::Output {
        1. / Self::COUNT as f32
    }
}
