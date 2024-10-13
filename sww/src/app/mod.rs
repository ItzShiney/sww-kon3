use crate::utility::Lazy;
use crate::window::event::ActiveEventLoop;
use crate::window::event::DeviceEvent;
use crate::window::event::DeviceId;
use crate::window::event::WindowEvent;
use crate::window::RenderWindow;
use crate::window::Window;
use crate::window::WindowId;
use std::sync::Arc;
use winit::application::ApplicationHandler;

mod handle_event;

pub use handle_event::*;

pub trait EventHandlerBuilder<E: EventHandler>: FnOnce(&Arc<RenderWindow>) -> E {}
impl<E: EventHandler, T: FnOnce(&Arc<RenderWindow>) -> E> EventHandlerBuilder<E> for T {}

pub trait WindowInfoBuilder: FnOnce(&ActiveEventLoop) -> WindowInfo {}
impl<T: FnOnce(&ActiveEventLoop) -> WindowInfo> WindowInfoBuilder for T {}

// TODO rename
pub struct WindowInfo {
    window: Arc<Window>,
    rw: Arc<RenderWindow>,
}

impl WindowInfo {
    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn rw(&self) -> &RenderWindow {
        &self.rw
    }
}

pub struct App<WIB: WindowInfoBuilder, E: EventHandler, EB: EventHandlerBuilder<E>> {
    window_info: Lazy<Arc<WindowInfo>, WIB>,
    event_handler: Lazy<E, EB>,
}

// TODO rename
pub fn app_new<E: EventHandler, EB: EventHandlerBuilder<E>>(
    window_builder: impl FnOnce(&ActiveEventLoop) -> Window,
    rw_builder: impl FnOnce(&Arc<Window>) -> RenderWindow,
    event_handler_builder: EB,
) -> App<impl WindowInfoBuilder, E, EB> {
    App {
        window_info: Lazy::new(move |event_loop: &_| {
            let window = Arc::new(window_builder(event_loop));
            let rw = Arc::new(rw_builder(&window));
            WindowInfo { window, rw }
        }),
        event_handler: Lazy::new(event_handler_builder),
    }
}

impl<WIB: WindowInfoBuilder, E: EventHandler, EB: EventHandlerBuilder<E>> App<WIB, E, EB> {
    pub fn window_info(&self) -> Option<&Arc<WindowInfo>> {
        self.window_info.get()
    }

    pub fn event_handler(&self) -> Option<&E> {
        self.event_handler.get()
    }

    pub fn event_handler_mut(&mut self) -> Option<&mut E> {
        self.event_handler.get_mut()
    }
}

impl<WIB: WindowInfoBuilder, E: EventHandler, EB: EventHandlerBuilder<E>> ApplicationHandler
    for App<WIB, E, EB>
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_info = &*self.window_info.get_or_init_map(event_loop, Arc::new);
        self.event_handler.get_or_init(&window_info.rw);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let event_handler = self.event_handler_mut().unwrap();
        event_handler.handle_event(event_loop, window_id, event);
    }

    fn device_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        device_id: DeviceId,
        event: DeviceEvent,
    ) {
        let event_handler = self.event_handler_mut().unwrap();
        event_handler.device_event(event_loop, device_id, event);
    }
}
