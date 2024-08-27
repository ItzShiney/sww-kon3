use crate::window::*;
use event::*;

#[derive(Clone, Copy)]
pub struct EventInfo<'w> {
    pub window: &'w Window,
    pub event_loop: &'w ActiveEventLoop,
    pub window_id: WindowId,
}

#[rustfmt::skip]
#[allow(unused)]
pub trait HandleEvent {
    fn handle_event(&self, window: &Window, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        let info = EventInfo {
            window,
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

    fn on_activation_token_done(&self, info: EventInfo, serial: AsyncRequestSerial, token: ActivationToken) {}
    fn on_resized(&self, info: EventInfo, new_size: PhysicalSize);
    fn on_moved(&self, info: EventInfo, new_position: PhysicalPosition) {}
    fn on_close_requested(&self, info: EventInfo) {
        info.event_loop.exit();
    }
    fn on_destroyed(&self, info: EventInfo) {}
    fn on_dropped_file(&self, info: EventInfo, path: FilePath) {}
    fn on_hovered_file(&self, info: EventInfo, path: FilePath) {}
    fn on_hovered_file_cancelled(&self, info: EventInfo) {}
    fn on_focused(&self, info: EventInfo, is_focused: bool) {}
    fn on_keyboard_input(&self, info: EventInfo, device_id: DeviceId, event: KeyEvent, is_synthetic: bool) {}
    fn on_modifiers_changed(&self, info: EventInfo, modifiers: KeyboardModifiers) {}
    fn on_ime(&self, info: EventInfo, ime: Ime) {}
    fn on_cursor_moved(&self, info: EventInfo, device_id: DeviceId, position: CursorPosition) {}
    fn on_cursor_entered(&self, info: EventInfo, device_id: DeviceId) {}
    fn on_cursor_left(&self, info: EventInfo, device_id: DeviceId) {}
    fn on_mouse_wheel(&self, info: EventInfo, device_id: DeviceId, delta: MouseScrollDelta, phase: TouchPhase) {}
    fn on_mouse_input(&self, info: EventInfo, device_id: DeviceId, state: ElementState, button: MouseButton) {}
    fn on_pinch_gesture(&self, info: EventInfo, device_id: DeviceId, delta: f64, phase: TouchPhase) {}
    fn on_pan_gesture(&self, info: EventInfo, device_id: DeviceId, delta: PanDelta, phase: TouchPhase) {}
    fn on_double_tap_gesture(&self, info: EventInfo, device_id: DeviceId) {}
    fn on_rotation_gesture(&self, info: EventInfo, device_id: DeviceId, delta: f32, phase: TouchPhase) {}
    fn on_touchpad_magnify(&self, info: EventInfo, device_id: DeviceId, delta: f64, phase: TouchPhase) {}
    fn on_smart_magnify(&self, info: EventInfo, device_id: DeviceId) {}
    fn on_touchpad_rotate(&self, info: EventInfo, device_id: DeviceId, delta: f32, phase: TouchPhase) {}
    fn on_touchpad_pressure(&self, info: EventInfo, device_id: DeviceId, pressure: f32, stage: i64) {}
    fn on_axis_motion(&self, info: EventInfo, device_id: DeviceId, axis: AxisId, value: f64) {}
    fn on_touch(&self, info: EventInfo, touch: Touch) {}
    fn on_scale_factor_changed(&self, info: EventInfo, scale_factor: f64, inner_size_writer: InnerSizeWriter) {}
    fn on_theme_changed(&self, info: EventInfo, theme: Theme) {}
    fn on_occluded(&self, info: EventInfo, is_occluded: bool) {}
    fn on_redraw_requested(&self, info: EventInfo) {}
}
