use crate::shared;
use crate::shared::Shared;
use crate::DrawPass;
use crate::Drawers;
use crate::Element;
use crate::Event;
use crate::InvalidateCache;
use crate::Location;
use std::sync as arc;
use std::sync::Arc;
use std::sync::Mutex;
use sww::app::App as SwwApp;
use sww::app::EventHandlerBuilder;
use sww::app::EventInfo;
use sww::app::HandleEvent;
use sww::app::RenderWindowBuilder;
use sww::app::WindowBuilder;
use sww::wgpu;
use sww::window::event::ActiveEventLoop;
use sww::window::event::DeviceId;
use sww::window::event::ElementState;
use sww::window::event::EventLoopError;
use sww::window::event::MouseButton;
use sww::window::event::PhysicalSize;
use sww::window::event_loop;
use sww::window::rw_builder;
use sww::window::window_attributes;
use sww::window::DefaultRenderWindowSettings;
use sww::window::RenderWindow;
use sww::window::RenderWindowSettings;

// FIXME autogenerate from UI
mod resources {
    #[allow(clippy::wildcard_imports)]
    use crate::resources::mesh::*;
    use crate::resources::Resource;
    use crate::resources::ResourceFrom;
    use sww::window::RenderWindow;

    pub struct Resources(&'static (UnitSquareTopLeft, NoGlobalTransform, DefaultTexture));

    impl Resources {
        pub fn new(rw: &RenderWindow) -> Self {
            Self(Box::leak(Box::new((
                Resource::new(rw),
                Resource::new(rw),
                Resource::new(rw),
            ))))
        }
    }

    impl ResourceFrom<Resources> for UnitSquareTopLeft {
        fn resource_from(resources: &Resources) -> &'static Self {
            &(resources.0).0
        }
    }

    impl ResourceFrom<Resources> for NoGlobalTransform {
        fn resource_from(resources: &Resources) -> &'static Self {
            &(resources.0).1
        }
    }

    impl ResourceFrom<Resources> for DefaultTexture {
        fn resource_from(resources: &Resources) -> &'static Self {
            &(resources.0).2
        }
    }
}
pub use resources::*;

// FIXME `Resources` -> `U::RequiredResources`
pub fn build_settings<E: Element<Resources> + 'static>(
    element_builder: impl FnOnce(&SharedBuilder) -> E,
    settings: impl RenderWindowSettings + 'static,
) -> App<
    E,
    impl WindowBuilder,
    impl RenderWindowBuilder,
    impl EventHandlerBuilder<EventHandler<Resources, E>>,
> {
    App(Arc::new_cyclic(|app| {
        let app = arc::Weak::<SwwApp<_, _, _, _>>::clone(app);
        let app = &SharedBuilder(app);
        let element = element_builder(app);

        SwwApp::new(
            |event_loop: &ActiveEventLoop| {
                event_loop
                    .create_window(window_attributes("kon3", 550, 310))
                    .expect("failed to create window")
            },
            rw_builder(settings),
            |rw| EventHandler {
                rw: Arc::clone(rw),
                element,
                resources: Resources::new(rw),
                drawers: Mutex::new(Drawers::default()),
            },
        )
    }))
}

pub fn run_settings<E: Element<Resources> + 'static>(
    element_builder: impl FnOnce(&SharedBuilder) -> E,
    settings: impl RenderWindowSettings + 'static,
) -> Result<(), EventLoopError> {
    build_settings(element_builder, settings).run()
}

pub fn run<E: Element<Resources> + 'static>(
    element_builder: impl FnOnce(&SharedBuilder) -> E,
) -> Result<(), EventLoopError> {
    build_settings(element_builder, DefaultRenderWindowSettings).run()
}

#[allow(clippy::type_complexity)]
pub struct App<
    E: Element<Resources>,
    WB: WindowBuilder,
    RB: RenderWindowBuilder,
    EB: EventHandlerBuilder<EventHandler<Resources, E>>,
>(Arc<SwwApp<EventHandler<Resources, E>, WB, RB, EB>>);

impl<
        E: Element<Resources>,
        WB: WindowBuilder,
        RB: RenderWindowBuilder,
        EB: EventHandlerBuilder<EventHandler<Resources, E>>,
    > InvalidateCache for SwwApp<EventHandler<Resources, E>, WB, RB, EB>
{
    fn invalidate_cache(&self, addr: shared::Addr) -> bool {
        if self.event_handler().unwrap().element.invalidate_cache(addr) {
            self.window().unwrap().request_redraw();
            true
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct SharedBuilder(arc::Weak<dyn InvalidateCache>);

impl SharedBuilder {
    pub fn shared<T>(&self, value: T) -> Shared<T> {
        Shared::new(value, self.clone())
    }
}

impl InvalidateCache for SharedBuilder {
    fn invalidate_cache(&self, addr: shared::Addr) -> bool {
        let app = self.0.upgrade().unwrap();
        app.invalidate_cache(addr)
    }
}

impl<
        E: Element<Resources>,
        WB: WindowBuilder,
        RB: RenderWindowBuilder,
        EB: EventHandlerBuilder<EventHandler<Resources, E>>,
    > App<E, WB, RB, EB>
{
    pub fn run(&mut self) -> Result<(), EventLoopError> {
        event_loop().run_app(&mut &*self.0)
    }
}

pub struct EventHandler<R, E: Element<R>> {
    rw: Arc<RenderWindow>,
    element: E,
    resources: R,
    drawers: Mutex<Drawers>,
}

impl<R, E: Element<R>> HandleEvent for EventHandler<R, E> {
    fn on_resized(&self, _info: EventInfo, new_size: PhysicalSize) {
        self.rw.resize_surface(new_size);
    }

    fn on_redraw_requested(&self, info: EventInfo) {
        let mut frame = self.rw.start_drawing();
        let mut render_pass =
            (frame.commands.encoder()).begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: frame.surface.view(),
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });

        let mut drawers = self.drawers.lock().unwrap();
        let mut pass = {
            let rw = Arc::clone(&self.rw);
            DrawPass::new(rw, &mut render_pass, &mut drawers)
        };
        let location = {
            let window_size = info.window.inner_size();
            Location::new(window_size)
        };

        self.element.draw(&mut pass, &self.resources, location);
    }

    fn on_mouse_input(
        &self,
        _info: EventInfo,
        _device_id: DeviceId,
        state: ElementState,
        _button: MouseButton,
    ) {
        if state == ElementState::Released {
            _ = self.element.handle_event(&Event::Click);
        }
    }
}
