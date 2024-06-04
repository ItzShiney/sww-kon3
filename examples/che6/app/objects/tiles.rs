mod single_color;

use crate::translation;
use crate::Drawer;
use crate::Scalers;
pub use single_color::*;
use sww::make_default_texture;
use sww::shaders;
use sww::shaders::mesh::Transform;
use sww::AppInfo;
use sww::Color;
use sww::VecBuffer;

pub fn make_white_black_tranforms<'q>(
    app_info: &'q AppInfo,
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

    (
        app_info.vec_buffer_vertex(white),
        app_info.vec_buffer_vertex(black),
    )
}

pub struct Tiles<'q> {
    white: SingleColorTiles<'q>,
    black: SingleColorTiles<'q>,
    bind_group1: shaders::mesh::BindGroup1,
}

impl<'q> Tiles<'q> {
    pub fn new(app_info: &'q AppInfo, scalers: &mut Scalers) -> Self {
        let (white_transforms, black_transforms) = make_white_black_tranforms(app_info);
        let white = SingleColorTiles::new(app_info, scalers, Color::splat(0.45), white_transforms);
        let black = SingleColorTiles::new(app_info, scalers, Color::splat(0.25), black_transforms);

        let bind_group1 = {
            let default_texture = make_default_texture(app_info.device(), app_info.queue());
            let default_texture_view = default_texture.create_view(&Default::default());

            shaders::mesh::BindGroup1::from_bindings(
                app_info.device(),
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
