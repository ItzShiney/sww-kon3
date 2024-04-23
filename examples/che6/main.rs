mod app;
mod sheet;

use app::*;
use sww::*;

pub fn main() {
    env_logger::init();

    let event_loop = event_loop();
    let window = window_builder("che6", 400, 200).build(&event_loop).unwrap();

    let app_info = AppInfo::new(&window, &DefaultAppSettings);
    let mut app = MyApp::new(&app_info, &window);

    app.run(event_loop).unwrap();
}
