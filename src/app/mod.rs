use crate::utility::Lazy;
use crate::window::event::ActiveEventLoop;
use crate::window::RenderWindow;
use crate::window::Window;
use std::sync::Arc;
use winit::application::ApplicationHandler;

mod handle_event;

pub use handle_event::*;

pub trait WindowBuilder: FnOnce(&ActiveEventLoop) -> Window {}
impl<T: FnOnce(&ActiveEventLoop) -> Window> WindowBuilder for T {}

pub trait RenderWindowBuilder: FnOnce(&Arc<Window>) -> RenderWindow {}
impl<T: FnOnce(&Arc<Window>) -> RenderWindow> RenderWindowBuilder for T {}

pub trait EventHandlerBuilder<E: HandleEvent>: FnOnce(&Arc<RenderWindow>) -> E {}
impl<E: HandleEvent, T: FnOnce(&Arc<RenderWindow>) -> E> EventHandlerBuilder<E> for T {}

pub struct App<
    E: HandleEvent,
    WB: WindowBuilder,
    RB: RenderWindowBuilder,
    EB: EventHandlerBuilder<E>,
> {
    window: Lazy<Arc<Window>, WB>,
    rw: Lazy<Arc<RenderWindow>, RB>,
    event_handler: Lazy<Arc<E>, EB>,
}

impl<E: HandleEvent, WB: WindowBuilder, RB: RenderWindowBuilder, EB: EventHandlerBuilder<E>>
    App<E, WB, RB, EB>
{
    pub fn new(window_builder: WB, rw_builder: RB, event_handler_builder: EB) -> Self {
        Self {
            window: Lazy::new(window_builder),
            rw: Lazy::new(rw_builder),
            event_handler: Lazy::new(event_handler_builder),
        }
    }

    pub fn window(&self) -> Option<Arc<Window>> {
        Some(Arc::clone(&*self.window.get()?))
    }

    pub fn rw(&self) -> Option<Arc<RenderWindow>> {
        Some(Arc::clone(&*self.rw.get()?))
    }

    pub fn event_handler(&self) -> Option<Arc<E>> {
        Some(Arc::clone(&*self.event_handler.get()?))
    }
}

impl<E: HandleEvent, WB: WindowBuilder, RB: RenderWindowBuilder, EB: EventHandlerBuilder<E>>
    ApplicationHandler for &App<E, WB, RB, EB>
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = &*self.window.get_or_init_map(event_loop, Arc::new);
        let rw = &*self.rw.get_or_init_map(window, Arc::new);
        self.event_handler.get_or_init_map(rw, Arc::new);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let window = &*self.window().unwrap();
        let event_handler = self.event_handler().unwrap();
        event_handler.handle_event(window, event_loop, window_id, event)
    }
}
