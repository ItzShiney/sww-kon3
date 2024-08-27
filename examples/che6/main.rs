mod app;
mod pieces;

use app::*;
use sww::app::App;
use sww::window::*;

fn main() {
    env_logger::init();

    let app = App::new(
        |event_loop| {
            event_loop
                .create_window(window_attributes("che6", 400, 200))
                .unwrap()
        },
        rw_builder_default(),
        EventHandler::new,
    );

    event_loop().run_app(&mut &app).unwrap();
}
