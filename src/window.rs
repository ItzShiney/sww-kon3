use event::*;

pub mod event {
    pub type AxisId = winit::event::AxisId;
    pub type DeviceId = winit::event::DeviceId;
    pub type ElementState = winit::event::ElementState;
    pub type Ime = winit::event::Ime;
    pub type InnerSizeWriter = winit::event::InnerSizeWriter;
    pub type KeyEvent = winit::event::KeyEvent;
    pub type MouseButton = winit::event::MouseButton;
    pub type MouseScrollDelta = winit::event::MouseScrollDelta;
    pub type Touch = winit::event::Touch;
    pub type TouchPhase = winit::event::TouchPhase;
    pub type WindowEvent = winit::event::WindowEvent;
    pub type ActiveEventLoop = winit::event_loop::ActiveEventLoop;
    pub type AsyncRequestSerial = winit::event_loop::AsyncRequestSerial;
    pub type ActivationToken = winit::window::ActivationToken;
    pub type Theme = winit::window::Theme;

    pub type EventLoop = winit::event_loop::EventLoop<()>;
    pub type EventLoopError = winit::error::EventLoopError;
    pub type EventLoopResult = Result<(), EventLoopError>;
    pub type PhysicalSize = winit::dpi::PhysicalSize<u32>;
    pub type PhysicalPosition = winit::dpi::PhysicalPosition<i32>;
    pub type FilePath = std::path::PathBuf;
    pub type CursorPosition = winit::dpi::PhysicalPosition<f64>;
    pub type PanDelta = winit::dpi::PhysicalPosition<f32>;
    pub type KeyboardModifiers = winit::event::Modifiers;
}

pub type Window = winit::window::Window;
pub type WindowAttributes = winit::window::WindowAttributes;
pub type WindowId = winit::window::WindowId;

pub fn window_attributes(title: &str, width: u32, height: u32) -> WindowAttributes {
    WindowAttributes::default()
        .with_title(title)
        .with_inner_size(PhysicalSize::new(width, height))
}

pub fn event_loop() -> EventLoop {
    EventLoop::new().unwrap()
}
