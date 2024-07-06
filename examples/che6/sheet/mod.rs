use crate::translation;
use image::RgbaImage;
use sww::media::make_texture;
use sww::media::DefaultView;
use sww::shaders;
use sww::shaders::mesh::Transform;
use sww::vec2;
use sww::window::RenderWindow;
use sww::Mat2;
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

pub struct Sheet {
    size: Vec2,
    texture_view: wgpu::TextureView,
}

impl Sheet {
    pub fn new(rw: &RenderWindow, image: RgbaImage) -> Self {
        Self {
            size: vec2(image.width() as _, image.height() as _),
            texture_view: make_texture(rw.device(), rw.queue(), &image).default_view(),
        }
    }
}

impl Sheet {
    pub fn make_piece_transform(
        &self,
        x: i32,
        y: i32,
        piece_type: PieceType,
        piece_color: PieceColor,
    ) -> Transform {
        let translation = translation(x, y);
        let texture_rect = texture_rect((piece_type, piece_color));

        Transform {
            // matrix: Mat2::from_scale_angle(texture_rect.size / self.size, 0.),
            translation,
            texture_rect,
            ..Default::default()
        }
    }

    pub fn texture_view(&self) -> &wgpu::TextureView {
        &self.texture_view
    }
}
