use crate::resources::Resources;
use crate::shared::SharedAddr;
use crate::Drawers;
use crate::Element;
use crate::Event;
use std::collections::BTreeSet;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use sww::app::app_new as sww_app_new;
use sww::app::App as SwwApp;
use sww::app::EventHandlerBuilder;
use sww::app::RenderWindowBuilder;
use sww::window::event::*;
use sww::window::*;
use sww::ApplicationHandler;

mod event_handler;
mod signals;

pub use event_handler::*;
pub use signals::*;

pub struct App<WIB: RenderWindowBuilder, E: Element, EB: EventHandlerBuilder<EventHandler<E>>> {
    app: SwwApp<WIB, EventHandler<E>, EB>,
    signal_receiver: Receiver<Signal>,
    updated_shareds: BTreeSet<SharedAddr>,
}

pub fn app<E: Element + 'static>(
    element: E,
    settings: impl WindowSettings + 'static,
) -> App<impl RenderWindowBuilder, E, impl EventHandlerBuilder<EventHandler<E>>> {
    let (signal_sender, signal_receiver) = channel();
    let signal_sender = SignalSender(signal_sender);

    let window_attributes = settings.window_attributes();
    let app = sww_app_new(
        move |event_loop: &ActiveEventLoop| {
            event_loop
                .create_window(window_attributes)
                .expect("failed to create window")
        },
        rw_builder(settings),
        |rw| {
            EventHandler::new(
                Arc::clone(rw),
                element,
                Resources::new(Arc::clone(rw)),
                Drawers::default(),
                Default::default(),
                signal_sender,
            )
        },
    );

    App {
        app,
        signal_receiver,
        updated_shareds: Default::default(),
    }
}

impl<WIB: RenderWindowBuilder, E: Element, EB: EventHandlerBuilder<EventHandler<E>>>
    ApplicationHandler for App<WIB, E, EB>
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.app.resumed(event_loop);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        self.app.window_event(event_loop, window_id, event);
        self.handle_signals();
    }
}

impl<WIB: RenderWindowBuilder, E: Element, EB: EventHandlerBuilder<EventHandler<E>>>
    App<WIB, E, EB>
{
    pub fn run(&mut self) -> Result<(), EventLoopError> {
        event_loop().run_app(self)
    }

    fn handle_signals(&mut self) {
        loop {
            let mut signals = self.signal_receiver.try_iter().peekable();
            if signals.peek().is_none() {
                break;
            }

            for signal in signals {
                match signal {
                    Signal::Redraw => {
                        self.app.rw().unwrap().window().request_redraw();
                    }

                    Signal::SharedUpdated(addr) => {
                        self.updated_shareds.insert(addr);
                    }
                }
            }

            let updated_shareds = self.updated_shareds.iter().copied();
            if !updated_shareds.len() == 0 {
                break;
            }

            let element = &self.app.event_handler().unwrap().element();
            let signal_sender = &self.app.event_handler().unwrap().signal_sender();

            for addr in updated_shareds {
                _ = element.handle_event(signal_sender, &Event::SharedUpdated(addr));
            }
            self.updated_shareds.clear();
        }
    }
}
