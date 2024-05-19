use crate::ActivationToken;
use crate::AsyncRequestSerial;
use crate::AxisId;
use crate::CursorPosition;
use crate::DeviceId;
use crate::ElementState;
use crate::EventLoop;
use crate::EventLoopResult;
use crate::FilePath;
use crate::Ime;
use crate::InnerSizeWriter;
use crate::KeyEvent;
use crate::KeyboardModifiers;
use crate::MouseButton;
use crate::MouseScrollDelta;
use crate::PanDelta;
use crate::PhysicalPosition;
use crate::PhysicalSize;
use crate::Theme;
use crate::Touch;
use crate::TouchPhase;
use crate::WindowEvent;
use crate::WindowId;
use winit::event::Event;
use winit::event_loop::ActiveEventLoop;

#[derive(Clone, Copy)]
pub struct EventInfo<'e> {
    pub event_loop: &'e ActiveEventLoop,
    pub window_id: WindowId,
}

#[rustfmt::skip]
#[allow(unused)]
pub trait App {
    fn handle_event(&mut self, event_loop: &ActiveEventLoop, event: Event<()>) {
        let Event::WindowEvent { window_id, event } = event else {
            return;
        };

        let info = EventInfo {
            event_loop,
            window_id,
        };

        match event {
            WindowEvent::ActivationTokenDone { serial, token } => self.on_activation_token_done(info, serial, token),
            WindowEvent::Resized(new_size) => self.on_resized(info, new_size),
            WindowEvent::Moved(arg) => self.on_moved(info, arg),
            WindowEvent::CloseRequested => self.on_close_requested(info),
            WindowEvent::Destroyed => self.on_destroyed(info),
            WindowEvent::DroppedFile(path) => self.on_dropped_file(info, path),
            WindowEvent::HoveredFile(path) => self.on_hovered_file(info, path),
            WindowEvent::HoveredFileCancelled => self.on_hovered_file_cancelled(info),
            WindowEvent::Focused(is_focused) => self.on_focused(info, is_focused),
            WindowEvent::KeyboardInput { device_id, event, is_synthetic } => self.on_keyboard_input(info, device_id, event, is_synthetic),
            WindowEvent::ModifiersChanged(modifiers) => self.on_modifiers_changed(info, modifiers),
            WindowEvent::Ime(ime) => self.on_ime(info, ime),
            WindowEvent::CursorMoved { device_id, position } => self.on_cursor_moved(info, device_id, position),
            WindowEvent::CursorEntered { device_id } => self.on_cursor_entered(info, device_id),
            WindowEvent::CursorLeft { device_id } => self.on_cursor_left(info, device_id),
            WindowEvent::MouseWheel { device_id, delta, phase } => self.on_mouse_wheel(info, device_id, delta, phase),
            WindowEvent::MouseInput { device_id, state, button } => self.on_mouse_input(info, device_id, state, button),
            WindowEvent::PinchGesture { device_id, delta, phase } => self.on_pinch_gesture(info, device_id, delta, phase),
            WindowEvent::PanGesture { device_id, delta, phase } => self.on_pan_gesture(info, device_id, delta, phase),
            WindowEvent::DoubleTapGesture { device_id } => self.on_double_tap_gesture(info, device_id),
            WindowEvent::RotationGesture { device_id, delta, phase } => self.on_rotation_gesture(info, device_id, delta, phase),
            WindowEvent::TouchpadPressure { device_id, pressure, stage } => self.on_touchpad_pressure(info, device_id, pressure, stage),
            WindowEvent::AxisMotion { device_id, axis, value } => self.on_axis_motion(info, device_id, axis, value),
            WindowEvent::Touch(touch) => self.on_touch(info, touch),
            WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer } => self.on_scale_factor_changed(info, scale_factor, inner_size_writer),
            WindowEvent::ThemeChanged(theme) => self.on_theme_changed(info, theme),
            WindowEvent::Occluded(is_occluded) => self.on_occluded(info, is_occluded),
            WindowEvent::RedrawRequested => self.on_redraw_requested(info),
        }
    }

    fn on_activation_token_done(&mut self, info: EventInfo, serial: AsyncRequestSerial, token: ActivationToken) {}
    fn on_resized(&mut self, info: EventInfo, new_size: PhysicalSize) {}
    fn on_moved(&mut self, info: EventInfo, new_position: PhysicalPosition) {}
    fn on_close_requested(&mut self, info: EventInfo) {
        info.event_loop.exit()
    }
    fn on_destroyed(&mut self, info: EventInfo) {}
    fn on_dropped_file(&mut self, info: EventInfo, path: FilePath) {}
    fn on_hovered_file(&mut self, info: EventInfo, path: FilePath) {}
    fn on_hovered_file_cancelled(&mut self, info: EventInfo) {}
    fn on_focused(&mut self, info: EventInfo, is_focused: bool) {}
    fn on_keyboard_input(&mut self, info: EventInfo, device_id: DeviceId, event: KeyEvent, is_synthetic: bool) {}
    fn on_modifiers_changed(&mut self, info: EventInfo, modifiers: KeyboardModifiers) {}
    fn on_ime(&mut self, info: EventInfo, ime: Ime) {}
    fn on_cursor_moved(&mut self, info: EventInfo, device_id: DeviceId, position: CursorPosition) {}
    fn on_cursor_entered(&mut self, info: EventInfo, device_id: DeviceId) {}
    fn on_cursor_left(&mut self, info: EventInfo, device_id: DeviceId) {}
    fn on_mouse_wheel(&mut self, info: EventInfo, device_id: DeviceId, delta: MouseScrollDelta, phase: TouchPhase) {}
    fn on_mouse_input(&mut self, info: EventInfo, device_id: DeviceId, state: ElementState, button: MouseButton) {}
    fn on_pinch_gesture(&mut self, info: EventInfo, device_id: DeviceId, delta: f64, phase: TouchPhase) {}
    fn on_pan_gesture(&mut self, info: EventInfo, device_id: DeviceId, delta: PanDelta, phase: TouchPhase) {}
    fn on_double_tap_gesture(&mut self, info: EventInfo, device_id: DeviceId) {}
    fn on_rotation_gesture(&mut self, info: EventInfo, device_id: DeviceId, delta: f32, phase: TouchPhase) {}
    fn on_touchpad_magnify(&mut self, info: EventInfo, device_id: DeviceId, delta: f64, phase: TouchPhase) {}
    fn on_smart_magnify(&mut self, info: EventInfo, device_id: DeviceId) {}
    fn on_touchpad_rotate(&mut self, info: EventInfo, device_id: DeviceId, delta: f32, phase: TouchPhase) {}
    fn on_touchpad_pressure(&mut self, info: EventInfo, device_id: DeviceId, pressure: f32, stage: i64) {}
    fn on_axis_motion(&mut self, info: EventInfo, device_id: DeviceId, axis: AxisId, value: f64) {}
    fn on_touch(&mut self, info: EventInfo, touch: Touch) {}
    fn on_scale_factor_changed(&mut self, info: EventInfo, scale_factor: f64, inner_size_writer: InnerSizeWriter) {}
    fn on_theme_changed(&mut self, info: EventInfo, theme: Theme) {}
    fn on_occluded(&mut self, info: EventInfo, is_occluded: bool) {}
    fn on_redraw_requested(&mut self, info: EventInfo) {}

    fn run(&mut self, event_loop: EventLoop) -> EventLoopResult {
        #[allow(deprecated)]
        event_loop.run(|event, target| self.handle_event(target, event))
    }
}
