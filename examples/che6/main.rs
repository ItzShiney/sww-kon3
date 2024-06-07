mod app;
mod sheet;

use app::*;
use sww::app::app_info_builder;
use sww::app::DefaultAppSettings;
use sww::app::LazyWindowedApp;
use sww::app::WindowedApp;
use sww::app_builder;
use sww::window::event_loop;
use sww::window::window_attributes;

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
