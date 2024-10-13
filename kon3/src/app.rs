use crate::resources::Resources;
use crate::shared;
use crate::shared::Shared;
use crate::DrawPass;
use crate::Drawers;
use crate::Element;
use crate::Event;
use crate::LocationPoint;
use crate::LocationRect;
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use sww::app::app_new;
use sww::app::App as SwwApp;
use sww::app::EventHandler as SwwEventHandler;
use sww::app::EventHandlerBuilder;
use sww::app::WindowInfoBuilder;
use sww::dvec2;
use sww::wgpu;
use sww::window::event::*;
use sww::window::*;
use sww::DVec2;

pub fn build_settings<E: Element + 'static>(
    element_builder: impl FnOnce(&SignalSender) -> E,
    settings: impl RenderWindowSettings + 'static,
) -> App<impl WindowInfoBuilder, E, impl EventHandlerBuilder<EventHandler<E>>> {
    let (signal_sender, signal_receiver) = channel();
    let signal_sender = SignalSender(signal_sender);

    let element = element_builder(&signal_sender);
    let app = app_new(
        |event_loop: &ActiveEventLoop| {
            event_loop
                .create_window(window_attributes("kon3", 550, 310))
                .expect("failed to create window")
        },
        rw_builder(settings),
        |rw| EventHandler {
            rw: Arc::clone(rw),
            element,
            resources: Resources::new(Arc::clone(rw)),
            drawers: Drawers::default(),
            cursor_positions: Default::default(),
        },
    );

    App {
        app,
        signal_receiver,
    }
}

pub fn run_settings<E: Element + 'static>(
    element_builder: impl FnOnce(&SignalSender) -> E,
    settings: impl RenderWindowSettings + 'static,
) -> Result<(), EventLoopError> {
    build_settings(element_builder, settings).run()
}

pub fn run<E: Element + 'static>(
    element_builder: impl FnOnce(&SignalSender) -> E,
) -> Result<(), EventLoopError> {
    build_settings(element_builder, DefaultRenderWindowSettings).run()
}

pub struct App<WIB: WindowInfoBuilder, E: Element, EB: EventHandlerBuilder<EventHandler<E>>> {
    app: SwwApp<WIB, EventHandler<E>, EB>,
    signal_receiver: Receiver<Signal>,
}

impl<WIB: WindowInfoBuilder, E: Element, EB: EventHandlerBuilder<EventHandler<E>>>
    sww::winit::application::ApplicationHandler for App<WIB, E, EB>
{
    fn resumed(&mut self, event_loop: &sww::winit::event_loop::ActiveEventLoop) {
        self.app.resumed(event_loop);
    }

    fn window_event(
        &mut self,
        event_loop: &sww::winit::event_loop::ActiveEventLoop,
        window_id: sww::winit::window::WindowId,
        event: sww::winit::event::WindowEvent,
    ) {
        self.app.window_event(event_loop, window_id, event);
        self.handle_signals();
    }
}

pub enum Signal {
    Redraw,
    InvalidateCache(shared::Addr),
}

#[derive(Clone)]
pub struct SignalSender(Sender<Signal>);

impl SignalSender {
    pub fn shared<T>(&self, value: T) -> Shared<T> {
        Shared::new(value, self.clone())
    }

    pub fn send(&mut self, signal: Signal) {
        self.0.send(signal).unwrap();
    }
}

impl<WIB: WindowInfoBuilder, E: Element, EB: EventHandlerBuilder<EventHandler<E>>> App<WIB, E, EB> {
    pub fn run(&mut self) -> Result<(), EventLoopError> {
        event_loop().run_app(self)
    }

    fn handle_signals(&mut self) {
        for signal in self.signal_receiver.try_iter() {
            match signal {
                Signal::Redraw => self.app.window_info().unwrap().window().request_redraw(),

                Signal::InvalidateCache(addr) => {
                    self.app
                        .event_handler()
                        .unwrap()
                        .element
                        .invalidate_cache(addr);
                }
            }
        }
    }
}

pub struct EventHandler<E: Element> {
    rw: Arc<RenderWindow>,
    element: E,
    resources: Resources,
    drawers: Drawers,
    cursor_positions: HashMap<DeviceId, DVec2>,
}

impl<E: Element> SwwEventHandler for EventHandler<E> {
    fn handle_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::Resized(new_size) => {
                self.rw.resize_surface(new_size);
            }

            WindowEvent::RedrawRequested => {
                let mut frame = self.rw.start_drawing();
                let (commands, surface) = frame.commands_surface();
                let mut render_pass =
                    (commands.encoder()).begin_render_pass(&wgpu::RenderPassDescriptor {
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: surface.view(),
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        ..Default::default()
                    });

                let mut pass = {
                    let rw = Arc::clone(&self.rw);
                    DrawPass::new(rw, &mut render_pass, &mut self.drawers)
                };
                let location = {
                    let window_size = self.rw.window().inner_size();
                    LocationRect::new(window_size)
                };

                self.element.draw(&mut pass, &self.resources, location);
            }

            WindowEvent::CursorMoved {
                device_id,
                position: FloatPosition { x, y },
            } => {
                self.cursor_positions.insert(device_id, dvec2(x, y));
            }

            WindowEvent::CursorLeft { device_id } => {
                self.cursor_positions.remove(&device_id);
            }

            WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => {
                if let Some(position) = self.cursor_positions.get(&device_id).copied() {
                    let event = match state {
                        ElementState::Released => Event::Click {
                            point: LocationPoint::new(position, self.rw.window().inner_size()),
                            button,
                        },

                        _ => return, // FIXME
                    };

                    _ = self.element.handle_event(&event);
                }
            }

            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            _ => {}
        }
    }
}
