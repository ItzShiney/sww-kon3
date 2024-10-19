mod app;
mod pieces;

use app::*;
use sww::app::app_new;
use sww::window::*;

fn main() {
    let mut app = app_new(
        |event_loop| {
            event_loop
                .create_window(window_attributes("che6", 400, 200))
                .unwrap()
        },
        rw_builder_default(),
        EventHandler::new,
    );

    event_loop().run_app(&mut app).unwrap();
}
