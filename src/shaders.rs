#![allow(clippy::derivable_impls)]
pub const PADDING: glam::Vec2 = glam::Vec2::new(0., 0.);

pub mod mesh {
    include!(concat!(env!("OUT_DIR"), "/mesh.rs"));

    use super::PADDING;
    pub use bind_groups::*;
    use glam::vec2;
    use glam::Mat2;
    use glam::Vec2;

    impl<'s> From<wgpu::BufferBinding<'s>> for BindGroupLayout0<'s> {
        fn from(global_transform: wgpu::BufferBinding<'s>) -> Self {
            Self { global_transform }
        }
    }

    pub fn in_vertex(
        position: glam::Vec2,
        color: glam::Vec4,
        texture_coord: glam::Vec2,
    ) -> InVertex {
        InVertex {
            position,
            _1: PADDING,
            color,
            texture_coord,
            _2: PADDING,
        }
    }

    pub fn transform(
        matrix: Mat2,
        translation: Vec2,
        color: glam::Vec4,
        texture_rect: Rectangle,
    ) -> Transform {
        Transform {
            matrix,
            translation,
            _1: PADDING,
            color,
            texture_rect,
        }
    }

    impl Default for Transform {
        fn default() -> Self {
            transform(
                Default::default(),
                Default::default(),
                [1.; 4].into(),
                Default::default(),
            )
        }
    }

    impl Default for Rectangle {
        fn default() -> Self {
            Self {
                top_left: vec2(0., 0.),
                size: vec2(1., 1.),
            }
        }
    }
}
