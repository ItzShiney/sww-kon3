pub use winit::dpi::PhysicalSize;
pub use winit::event::WindowEvent;
pub use winit::window::Window;
pub use winit::window::WindowBuilder;

pub type Event = winit::event::Event<()>;
pub type EventLoop = winit::event_loop::EventLoop<()>;
pub type EventLoopError = winit::error::EventLoopError;
pub type EventLoopResult = Result<(), EventLoopError>;
pub type EventLoopTarget = winit::event_loop::EventLoopWindowTarget<()>;

pub fn window_builder(title: &str, width: u32, height: u32) -> WindowBuilder {
    WindowBuilder::default()
        .with_title(title)
        .with_inner_size(PhysicalSize::new(width, height))
}

pub fn event_loop() -> EventLoop {
    EventLoop::new().unwrap()
}
