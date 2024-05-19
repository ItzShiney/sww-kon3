pub use winit::event::AxisId;
pub use winit::event::DeviceId;
pub use winit::event::ElementState;
pub use winit::event::Ime;
pub use winit::event::InnerSizeWriter;
pub use winit::event::KeyEvent;
pub use winit::event::MouseButton;
pub use winit::event::MouseScrollDelta;
pub use winit::event::Touch;
pub use winit::event::TouchPhase;
pub use winit::event::WindowEvent;
pub use winit::event_loop::AsyncRequestSerial;
pub use winit::window::ActivationToken;
pub use winit::window::Theme;
pub use winit::window::Window;
pub use winit::window::WindowAttributes;
pub use winit::window::WindowId;

pub type EventLoop = winit::event_loop::EventLoop<()>;
pub type EventLoopError = winit::error::EventLoopError;
pub type EventLoopResult = Result<(), EventLoopError>;
pub type PhysicalSize = winit::dpi::PhysicalSize<u32>;
pub type PhysicalPosition = winit::dpi::PhysicalPosition<i32>;
pub type FilePath = std::path::PathBuf;
pub type CursorPosition = winit::dpi::PhysicalPosition<f64>;
pub type PanDelta = winit::dpi::PhysicalPosition<f32>;
pub type KeyboardModifiers = winit::event::Modifiers;

pub fn window_builder(title: &str, width: u32, height: u32) -> WindowAttributes {
    WindowAttributes::default()
        .with_title(title)
        .with_inner_size(PhysicalSize::new(width, height))
}

pub fn event_loop() -> EventLoop {
    EventLoop::new().unwrap()
}
