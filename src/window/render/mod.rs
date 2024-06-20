use crate::window::*;
use event::*;
use pollster::FutureExt;
use std::cell::RefCell;

mod frame;

pub use frame::*;

pub struct RenderWindow<'w> {
    window: &'w Window,
    surface: wgpu::Surface<'w>,
    surface_config: RefCell<wgpu::SurfaceConfiguration>,
    swapchain_format: wgpu::TextureFormat,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl<'w> RenderWindow<'w> {
    pub fn new(
        window: &'w Window,
        settings: &impl RenderWindowSettings,
    ) -> Result<Self, AppInfoError> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(settings.instance_descriptor());

        let surface = instance.create_surface(window).unwrap();
        let adapter = instance
            .request_adapter(&settings.request_adapter_options(&surface))
            .block_on()
            .ok_or(AppInfoError::NoAdapter)?;

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];
        let swapchain_alpha_mode = swapchain_capabilities.alpha_modes[0];

        let (device, queue) = adapter
            .request_device(&settings.device_descriptor(&adapter), None)
            .block_on()?;

        let surface_config = settings.surface_config(size, swapchain_format, swapchain_alpha_mode);
        surface.configure(&device, &surface_config);
        let surface_config = surface_config.into();

        Ok(Self {
            window,
            surface,
            surface_config,
            swapchain_format,
            device,
            queue,
        })
    }

    pub fn window(&self) -> &Window {
        self.window
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn start_drawing(&self) -> Result<Frame, wgpu::SurfaceError> {
        let command_encoder = self.device.create_command_encoder(&Default::default());
        let surface_texture = self.surface.get_current_texture()?;

        Ok(Frame::new(self, command_encoder, surface_texture))
    }

    pub fn resize_surface(&self, new_size: PhysicalSize) {
        let mut surface_config = self.surface_config.borrow_mut();

        surface_config.width = new_size.width.max(1);
        surface_config.height = new_size.height.max(1);

        self.surface.configure(&self.device, &surface_config);
    }

    pub fn swapchain_format(&self) -> wgpu::TextureFormat {
        self.swapchain_format
    }
}

pub fn render_window_builder<'s>(
    settings: &'s impl RenderWindowSettings,
) -> impl FnOnce(&Window) -> RenderWindow + 's {
    move |window| RenderWindow::new(window, settings).unwrap()
}
