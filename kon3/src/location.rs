use sww::shaders::mesh::Rectangle;
use sww::vec2;
use sww::window::event::PhysicalPosition;
use sww::window::event::PhysicalSize;
use sww::Vec2;

#[derive(Clone, Copy)]
pub struct LocationPoint {
    point: Vec2,
    window_size: PhysicalSize,
}

impl LocationPoint {
    pub const fn new(point: Vec2, window_size: PhysicalSize) -> Self {
        Self { point, window_size }
    }

    pub const fn point(self) -> Vec2 {
        self.point
    }

    pub const fn window_size(self) -> PhysicalSize {
        self.window_size
    }

    pub fn window_point(self) -> PhysicalPosition {
        let point = (self.point + Vec2::ONE) / 2.;
        PhysicalPosition::new(
            (point.x * self.window_size.width as f32) as _,
            (point.y * self.window_size.height as f32) as _,
        )
    }
}

#[derive(Clone, Copy)]
pub struct LocationRect {
    rect: Rectangle,
    window_size: PhysicalSize,
}

impl LocationRect {
    pub const fn new(window_size: PhysicalSize) -> Self {
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

    pub const fn window_size(self) -> PhysicalSize {
        self.window_size
    }

    pub fn window_rect_size(self) -> PhysicalSize {
        let rect_size = self.rect.size / 2.;
        PhysicalSize::new(
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
