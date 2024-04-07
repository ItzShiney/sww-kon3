mod app;
mod sheet;

use app::*;
use sww::*;
use winit::event_loop::EventLoop;

pub fn main() {
    let event_loop = EventLoop::new().unwrap();

    let window = winit::window::WindowBuilder::new()
        .with_title("che6")
        .with_inner_size(winit::dpi::PhysicalSize::new(400, 200))
        .build(&event_loop)
        .unwrap();

    env_logger::init();

    let app_info = AppInfo::new(&window, &DefaultAppSettings);
    let mut app = App::new(&app_info, &window);

    event_loop
        .run(|event, target| app.event_handler(event, target))
        .unwrap();
}
