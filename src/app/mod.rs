use crate::window::event::*;
use crate::window::WindowId;
use crate::Lazy;
use winit::application::ApplicationHandler;

mod handle_event;
mod pack;
mod render_window;
mod settings;

pub use handle_event::*;
pub use pack::*;
pub use render_window::*;
pub use settings::*;

pub struct App<F>(Lazy<AppPack, F>);

impl<F: FnOnce(&ActiveEventLoop) -> AppPack> App<F> {
    pub fn new(f: F) -> Self {
        Self(Lazy::new(f))
    }
}

impl<F: FnOnce(&ActiveEventLoop) -> AppPack> ApplicationHandler<()> for App<F> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // TODO
        self.0.value(event_loop);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let windowed_app = self.0.value(event_loop);

        windowed_app.with_mut(|fields| {
            fields
                .event_handler
                .handle_event(fields.window, event_loop, window_id, event)
        })
    }
}
