pub use winit::dpi::PhysicalSize;
pub use winit::window::Window;
pub use winit::window::WindowBuilder;

pub type EventLoop = winit::event_loop::EventLoop<()>;
pub type EventLoopError = winit::error::EventLoopError;
pub type EventLoopResult = Result<(), EventLoopError>;

pub fn window_builder(title: &str, width: u32, height: u32) -> WindowBuilder {
    WindowBuilder::default()
        .with_title(title)
        .with_inner_size(PhysicalSize::new(width, height))
}

pub fn event_loop() -> EventLoop {
    EventLoop::new().unwrap()
}
