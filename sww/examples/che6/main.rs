mod app;
mod pieces;

use app::*;
use sww::app::app_new;
use sww::window::*;

struct Settings;
impl WindowSettings for Settings {
    fn window_attributes(&self) -> WindowAttributes {
        window_attributes("che6", 400, 200)
    }
}

fn main() {
    let settings = Settings;
    let window_attributes = settings.window_attributes();

    let mut app = app_new(
        |event_loop| event_loop.create_window(window_attributes).unwrap(),
        rw_builder(settings),
        EventHandler::new,
    );

    event_loop().run_app(&mut app).unwrap();
}
