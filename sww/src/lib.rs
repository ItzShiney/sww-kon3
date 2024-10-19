pub use wgpu;

pub mod app;
pub mod buffers;
pub mod drawing;
pub mod media;
pub mod shaders;
pub mod utility;
pub mod window;

pub use color::*;

mod color;

pub use glam::dvec2;
pub use glam::vec2;
pub use glam::vec4;
pub use glam::DVec2;
pub use glam::Mat2;
pub use glam::Vec2;
pub use glam::Vec4;
pub use winit::application::ApplicationHandler;
