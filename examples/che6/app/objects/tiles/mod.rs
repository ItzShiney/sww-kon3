use crate::translation;
use crate::Drawer;
use crate::Scalables;
use sww::buffers::MutVecBuffer;
use sww::media;
use sww::shaders;
use sww::shaders::mesh::Transform;
use sww::window::RenderWindow;
use sww::Color;

mod single_color;

pub use single_color::*;

pub fn make_white_black_tranforms<'w>(
    rw: &'w RenderWindow,
) -> (MutVecBuffer<'w, Transform>, MutVecBuffer<'w, Transform>) {
    let mut white = Vec::default();
    let mut black = Vec::default();

    for y in -4..4_i32 {
        for x in -4..4_i32 {
            let translation = translation(x, y);
            let colored_transforms = if (x + y).rem_euclid(2) == 0 {
                &mut black
            } else {
                &mut white
            };

            colored_transforms.push(Transform {
                translation,
                ..Default::default()
            });
        }
    }

    (
        MutVecBuffer::new_vertex(rw, white),
        MutVecBuffer::new_vertex(rw, black),
    )
}

pub struct Tiles<'w> {
    white: SingleColorTiles<'w>,
    black: SingleColorTiles<'w>,
    bind_group1: shaders::mesh::BindGroup1,
}

impl<'w> Tiles<'w> {
    pub fn new(rw: &'w RenderWindow, scalables: &mut Scalables) -> Self {
        let (white_transforms, black_transforms) = make_white_black_tranforms(rw);
        let white = SingleColorTiles::new(rw, scalables, Color::splat(0.45), white_transforms);
        let black = SingleColorTiles::new(rw, scalables, Color::splat(0.25), black_transforms);

        let bind_group1 = {
            let default_texture = media::make_default_texture(rw.device(), rw.queue());
            let default_texture_view = default_texture.create_view(&Default::default());

            shaders::mesh::BindGroup1::from_bindings(
                rw.device(),
                shaders::mesh::BindGroupLayout1 {
                    texture: &default_texture_view,
                },
            )
        };

        Self {
            white,
            black,
            bind_group1,
        }
    }
}

impl<'e> Tiles<'_> {
    pub fn draw(&'e self, drawer: &'e Drawer, render_pass: &mut wgpu::RenderPass<'e>) {
        self.white.draw(drawer, render_pass, &self.bind_group1);
        self.black.draw(drawer, render_pass, &self.bind_group1);
    }
}
