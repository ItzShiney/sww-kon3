use crate::translation;
use crate::Drawer;
use crate::Scalers;
use sww::app::RenderWindow;
use sww::media;
use sww::shaders;
use sww::shaders::mesh::Transform;
use sww::Color;
use sww::VecBuffer;

mod single_color;

pub use single_color::*;

pub fn make_white_black_tranforms<'q>(
    rw: &'q RenderWindow,
) -> (VecBuffer<'q, Transform>, VecBuffer<'q, Transform>) {
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

    (rw.vec_buffer_vertex(white), rw.vec_buffer_vertex(black))
}

pub struct Tiles<'q> {
    white: SingleColorTiles<'q>,
    black: SingleColorTiles<'q>,
    bind_group1: shaders::mesh::BindGroup1,
}

impl<'q> Tiles<'q> {
    pub fn new(rw: &'q RenderWindow, scalers: &mut Scalers) -> Self {
        let (white_transforms, black_transforms) = make_white_black_tranforms(rw);
        let white = SingleColorTiles::new(rw, scalers, Color::splat(0.45), white_transforms);
        let black = SingleColorTiles::new(rw, scalers, Color::splat(0.25), black_transforms);

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

    pub fn draw<'s>(&'s self, drawer: &'s Drawer, render_pass: &mut wgpu::RenderPass<'s>) {
        self.white.draw(drawer, render_pass, &self.bind_group1);
        self.black.draw(drawer, render_pass, &self.bind_group1);
    }
}
