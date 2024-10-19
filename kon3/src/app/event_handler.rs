use super::Signaler;
use crate::resources::Resources;
use crate::DrawPass;
use crate::Drawers;
use crate::Element;
use crate::Event;
use crate::LocationPoint;
use crate::LocationRect;
use std::collections::HashMap;
use std::sync::Arc;
use sww::app::EventHandler as SwwEventHandler;
use sww::dvec2;
use sww::wgpu;
use sww::window::event::*;
use sww::window::*;
use sww::DVec2;

pub struct EventHandler<E: Element> {
    rw: Arc<RenderWindow>,
    element: E,
    resources: Resources,
    drawers: Drawers,
    cursor_positions: HashMap<DeviceId, DVec2>,
    signaler: Signaler,
}

impl<E: Element> EventHandler<E> {
    pub fn new(
        rw: Arc<RenderWindow>,
        element: E,
        resources: Resources,
        drawers: Drawers,
        cursor_positions: HashMap<DeviceId, DVec2>,
        signaler: Signaler,
    ) -> Self {
        Self {
            rw,
            element,
            resources,
            drawers,
            cursor_positions,
            signaler,
        }
    }

    pub fn element(&self) -> &E {
        &self.element
    }

    pub fn signaler(&self) -> &Signaler {
        &self.signaler
    }
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

                    _ = self.element.handle_event(&self.signaler, &event);
                }
            }

            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            _ => {}
        }
    }
}
