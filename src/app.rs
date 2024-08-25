use crate::DrawPass;
use crate::Drawers;
use crate::Element;
use crate::Event;
use crate::Location;
use sww::app::App as AppRaw;
use sww::app::AppPack;
use sww::app::EventInfo;
use sww::app::HandleEvent;
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

pub struct App<F: FnOnce(&ActiveEventLoop) -> AppPack>(AppRaw<F>);

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
#[allow(clippy::wildcard_imports)]
pub use resources::*;

// FIXME `Resources` -> `U::RequiredResources`
pub fn build_settings<U: Element<Resources> + 'static>(
    ui: U,
    settings: &impl RenderWindowSettings,
) -> App<impl FnOnce(&ActiveEventLoop) -> AppPack + '_> {
    App(AppRaw::new(move |event_loop| {
        let window = event_loop
            .create_window(window_attributes("kon3", 550, 310))
            .expect("failed to create window");

        AppPack::new(window, rw_builder(settings), move |rw| {
            Box::new(EventHandler {
                rw,
                ui,
                resources: Resources::new(rw),
                drawers: Drawers::default(),
            })
        })
    }))
}

pub fn build(
    ui: impl Element<Resources> + 'static,
) -> App<impl FnOnce(&ActiveEventLoop) -> AppPack> {
    build_settings(ui, &DefaultRenderWindowSettings)
}

impl<F: FnOnce(&ActiveEventLoop) -> AppPack> App<F> {
    pub fn run(&mut self) -> Result<(), EventLoopError> {
        event_loop().run_app(&mut self.0)
    }
}

struct EventHandler<'w, R, E: Element<R>> {
    rw: &'w RenderWindow<'w>,
    ui: E,
    resources: R,
    drawers: Drawers<'w>,
}

impl<R, E: Element<R>> HandleEvent for EventHandler<'_, R, E> {
    fn on_resized(&mut self, _info: EventInfo, new_size: PhysicalSize) {
        self.rw.resize_surface(new_size);
    }

    fn on_redraw_requested(&mut self, info: EventInfo) {
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

        let window_size = info.window.inner_size();
        let location = Location::new(window_size);

        let mut pass = DrawPass::new(self.rw, &mut render_pass, &mut self.drawers);
        (self.ui).draw(&mut pass, &self.resources, location);
    }

    fn on_mouse_input(
        &mut self,
        _info: EventInfo,
        _device_id: DeviceId,
        state: ElementState,
        _button: MouseButton,
    ) {
        if state == ElementState::Released {
            _ = self.ui.handle_event(&Event::Click);
        }
    }
}
