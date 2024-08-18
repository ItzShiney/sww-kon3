use sww::app::App;
use sww::app::AppPack;
use sww::app::HandleEvent;
use sww::event_handler_builder;
use sww::window::DefaultRenderWindowSettings;
use sww::window::*;

pub struct MyApp<'w> {
    #[allow(unused)]
    rw: &'w RenderWindow<'w>,
}

impl<'w> MyApp<'w> {
    pub fn new(rw: &'w RenderWindow) -> Self {
        Self { rw }
    }
}

impl HandleEvent for MyApp<'_> {
    fn on_resized(&mut self, _info: sww::app::EventInfo, new_size: event::PhysicalSize) {
        self.rw.resize_surface(new_size);
    }
}

pub fn main() {
    env_logger::init();

    let mut app = App::new(|event_loop| {
        let window = event_loop
            .create_window(window_attributes("minimal", 1280, 720))
            .unwrap();

        AppPack::new(
            window,
            rw_builder(&DefaultRenderWindowSettings),
            event_handler_builder!(MyApp::new),
        )
    });

    event_loop().run_app(&mut app).unwrap();
}
