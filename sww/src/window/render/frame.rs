use crate::window::RenderWindow;

mod commands;
mod surface;

pub use commands::*;
pub use surface::*;

pub struct Frame<'w> {
    commands: FrameCommands<'w>,
    surface: FrameSurface,
}

impl<'w> Frame<'w> {
    pub fn new(
        info: &'w RenderWindow,
        command_encoder: wgpu::CommandEncoder,
        surface_texture: wgpu::SurfaceTexture,
    ) -> Self {
        Self {
            commands: FrameCommands::new(info, command_encoder),
            surface: FrameSurface::new(surface_texture),
        }
    }

    pub fn commands_surface(&mut self) -> (&mut FrameCommands<'w>, &mut FrameSurface) {
        (&mut self.commands, &mut self.surface)
    }
}
