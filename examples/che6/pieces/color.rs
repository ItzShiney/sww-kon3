use strum::EnumCount;
use strum_macros::EnumCount;
use sww::media::SheetCoord;

#[derive(Clone, Copy, EnumCount)]
pub enum PieceColor {
    White,
    Black,
}

impl From<PieceColor> for usize {
    fn from(value: PieceColor) -> Self {
        value as _
    }
}

impl SheetCoord for PieceColor {
    type Output = f32;

    fn coord(self) -> Self::Output {
        self as usize as f32 * Self::size()
    }

    fn size() -> Self::Output {
        1. / Self::COUNT as f32
    }
}
