use super::PieceColor;
use super::PieceType;
use sww::media::Sheet;

pub type PiecesSheetCoord = (PieceType, PieceColor);
pub type PiecesSheet = Sheet<PiecesSheetCoord>;
