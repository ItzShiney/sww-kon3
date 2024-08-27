use crate::window::RenderWindow;

pub struct FrameCommands<'w> {
    info: &'w RenderWindow,
    encoder: Option<wgpu::CommandEncoder>,
}

impl<'w> FrameCommands<'w> {
    pub(super) fn new(info: &'w RenderWindow, command_encoder: wgpu::CommandEncoder) -> Self {
        Self {
            info,
            encoder: Some(command_encoder),
        }
    }

    pub fn encoder(&mut self) -> &mut wgpu::CommandEncoder {
        self.encoder.as_mut().unwrap()
    }
}

impl Drop for FrameCommands<'_> {
    fn drop(&mut self) {
        let command_encoder = self.encoder.take().unwrap();
        self.info.queue.submit(Some(command_encoder.finish()));
    }
}
