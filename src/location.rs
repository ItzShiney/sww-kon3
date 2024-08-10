use sww::shaders::mesh::Rectangle;
use sww::vec2;
use sww::window::event::PhysicalSize;

pub struct Location {
    rect: Rectangle,
    window_size: PhysicalSize,
}

impl Location {
    pub const fn new(window_size: PhysicalSize) -> Self {
        Self {
            rect: Rectangle {
                top_left: vec2(-1., -1.),
                size: vec2(2., 2.),
            },
            window_size,
        }
    }

    pub const fn rect(&self) -> Rectangle {
        self.rect
    }

    pub const fn window_size(&self) -> PhysicalSize {
        self.window_size
    }

    pub fn window_rect_size(&self) -> PhysicalSize {
        let rect_size = self.rect.size * 0.5;
        PhysicalSize::new(
            (rect_size.x * self.window_size.width as f32).round() as _,
            (rect_size.y * self.window_size.height as f32).round() as _,
        )
    }

    pub fn subrect(self, rect: Rectangle) -> Self {
        Self {
            rect: self.rect.subrect(rect),
            window_size: self.window_size,
        }
    }
}
