use crate::app::WindowedApp;
use crate::window::event::*;
use crate::window::WindowId;
use crate::Lazy;
use winit::application::ApplicationHandler;

pub struct LazyWindowedApp<F>(Lazy<WindowedApp, F>);

impl<F: FnOnce(&ActiveEventLoop) -> WindowedApp> LazyWindowedApp<F> {
    pub fn new(f: F) -> Self {
        Self(Lazy::new(f))
    }
}

impl<F: FnOnce(&ActiveEventLoop) -> WindowedApp> ApplicationHandler<()> for LazyWindowedApp<F> {
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
                .app
                .handle_event(fields.window, event_loop, window_id, event)
        })
    }
}
