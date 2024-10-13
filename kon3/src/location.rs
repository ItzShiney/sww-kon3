use sww::shaders::mesh::Rectangle;
use sww::vec2;
use sww::window::event::IntPosition;
use sww::window::event::IntSize;
use sww::DVec2;

#[derive(Clone, Copy)]
pub struct LocationPoint {
    point: DVec2,
    window_size: IntSize,
}

impl LocationPoint {
    pub const fn new(point: DVec2, window_size: IntSize) -> Self {
        Self { point, window_size }
    }

    pub const fn point(self) -> DVec2 {
        self.point
    }

    pub const fn window_size(self) -> IntSize {
        self.window_size
    }

    pub fn window_point(self) -> IntPosition {
        let point = (self.point + DVec2::ONE) / 2.;
        IntPosition::new(
            (point.x * self.window_size.width as f64) as _,
            (point.y * self.window_size.height as f64) as _,
        )
    }
}

#[derive(Clone, Copy)]
pub struct LocationRect {
    rect: Rectangle,
    window_size: IntSize,
}

impl LocationRect {
    pub const fn new(window_size: IntSize) -> Self {
        Self {
            // FIXME
            rect: Rectangle {
                top_left: vec2(-1., 1.),
                size: vec2(2., -2.),
            },
            window_size,
        }
    }

    pub const fn rect(self) -> Rectangle {
        self.rect
    }

    pub const fn window_size(self) -> IntSize {
        self.window_size
    }

    pub fn window_rect_size(self) -> IntSize {
        let rect_size = self.rect.size / 2.;
        IntSize::new(
            (rect_size.x * self.window_size.width as f32) as _,
            (rect_size.y * self.window_size.height as f32) as _,
        )
    }

    pub fn subrect(self, rect: Rectangle) -> Self {
        Self {
            rect: self.rect.subrect(rect),
            window_size: self.window_size,
        }
    }
}
