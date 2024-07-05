use crate::window::RenderWindow;
use std::mem::ManuallyDrop;

pub struct FrameCommands<'w> {
    info: &'w RenderWindow<'w>,
    encoder: ManuallyDrop<wgpu::CommandEncoder>,
}

impl<'w> FrameCommands<'w> {
    pub(super) fn new(info: &'w RenderWindow<'w>, command_encoder: wgpu::CommandEncoder) -> Self {
        Self {
            info,
            encoder: ManuallyDrop::new(command_encoder),
        }
    }

    pub fn encoder(&mut self) -> &mut wgpu::CommandEncoder {
        &mut self.encoder
    }
}

impl Drop for FrameCommands<'_> {
    fn drop(&mut self) {
        let command_encoder = unsafe { ManuallyDrop::take(&mut self.encoder) };
        self.info.queue.submit(Some(command_encoder.finish()));
    }
}
