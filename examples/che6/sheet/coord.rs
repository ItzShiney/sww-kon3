use sww::vec2;
use sww::Vec2;

pub trait Coord {
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
