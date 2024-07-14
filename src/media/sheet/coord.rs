use crate::vec2;
use crate::Vec2;

pub trait SheetCoord {
    type Output;

    fn coord(self) -> Self::Output;
    fn size() -> Self::Output;
}

impl<A: SheetCoord<Output = f32>, B: SheetCoord<Output = f32>> SheetCoord for (A, B) {
    type Output = Vec2;

    fn coord(self) -> Self::Output {
        vec2(self.0.coord(), self.1.coord())
    }

    fn size() -> Self::Output {
        vec2(A::size(), B::size())
    }
}
