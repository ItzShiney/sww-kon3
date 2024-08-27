use super::make_texture;
use super::DefaultView;
use super::RgbaImage;
use crate::shaders;
use crate::vec2;
use crate::window::RenderWindow;
use crate::Vec2;
use std::marker::PhantomData;

mod coord;

pub use coord::*;

pub struct Sheet<T> {
    size: Vec2,
    texture_view: wgpu::TextureView,
    phantom: PhantomData<T>,
}

impl<T> Sheet<T> {
    pub fn new(rw: &RenderWindow, image: RgbaImage) -> Self {
        Self {
            size: vec2(image.width() as _, image.height() as _),
            texture_view: make_texture(rw.device(), rw.queue(), &image).default_view(),
            phantom: PhantomData,
        }
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }

    pub fn texture_view(&self) -> &wgpu::TextureView {
        &self.texture_view
    }
}

impl<T: SheetCoord<Output = Vec2>> Sheet<T> {
    pub fn texture_rect(&self, coord: T) -> shaders::mesh::Rectangle {
        let size = T::size();
        let top_left = coord.coord();

        shaders::mesh::Rectangle { top_left, size }
    }
}
