mod app;
mod sheet;

use app::*;
use sww::*;
use window::event_loop;
use window::window_attributes;

pub fn main() {
    env_logger::init();

    let mut app = LazyWindowedApp::new(|event_loop| {
        let window = event_loop
            .create_window(window_attributes("che6", 400, 200))
            .unwrap();

        WindowedApp::new(
            window,
            app_info_builder(&DefaultAppSettings),
            app_builder!(MyApp::new),
        )
    });

    event_loop().run_app(&mut app).unwrap();
}
