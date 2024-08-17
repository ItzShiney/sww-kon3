use crate::AnchorsTree;
use crate::BuildElement;
use crate::Element;
use crate::Location;
use sww::app::App as AppRaw;
use sww::app::AppPack;
use sww::app::EventInfo;
use sww::app::HandleEvent;
use sww::wgpu;
use sww::window::event::ActiveEventLoop;
use sww::window::event::EventLoopError;
use sww::window::event_loop;
use sww::window::rw_builder;
use sww::window::window_attributes;
use sww::window::DefaultRenderWindowSettings;
use sww::window::RenderWindow;
use sww::window::RenderWindowSettings;

pub struct App<F: FnOnce(&ActiveEventLoop) -> AppPack>(AppRaw<F>);

pub fn build_settings<B: BuildElement<Built: 'static>>(
    mut ui_builder: B,
    settings: &impl RenderWindowSettings,
) -> App<impl FnOnce(&ActiveEventLoop) -> AppPack + '_> {
    B::AnchorsSet::resolve_anchors(&mut ui_builder);
    let ui = ui_builder.build();

    App(AppRaw::new(move |event_loop| {
        let window = event_loop
            .create_window(window_attributes("kon3", 400, 200))
            .expect("failed to create window");

        AppPack::new(window, rw_builder(settings), move |rw| {
            Box::new(EventHandler { rw, ui })
        })
    }))
}

pub fn build<B: BuildElement<Built: 'static>>(
    ui_builder: B,
) -> App<impl FnOnce(&ActiveEventLoop) -> AppPack> {
    build_settings(ui_builder, &DefaultRenderWindowSettings)
}

impl<F: FnOnce(&ActiveEventLoop) -> AppPack> App<F> {
    pub fn run(&mut self) -> Result<(), EventLoopError> {
        event_loop().run_app(&mut self.0)
    }
}

struct EventHandler<'w, E: Element> {
    rw: &'w RenderWindow<'w>,
    ui: E,
}

impl<E: Element> HandleEvent for EventHandler<'_, E> {
    fn on_redraw_requested(&mut self, info: EventInfo) {
        let mut frame = self.rw.start_drawing();
        let mut render_pass =
            frame
                .commands
                .encoder()
                .begin_render_pass(&wgpu::RenderPassDescriptor {
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
        self.ui.draw(&mut render_pass, location);
    }
}
