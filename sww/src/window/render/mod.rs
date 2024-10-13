use crate::window::*;
use event::*;
use pollster::FutureExt;
use std::sync::Arc;
use std::sync::Mutex;

mod frame;

pub use frame::*;

pub struct RenderWindow {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    surface_config: Mutex<wgpu::SurfaceConfiguration>,
    swapchain_format: wgpu::TextureFormat,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl RenderWindow {
    pub fn new(
        window: Arc<Window>,
        settings: &impl RenderWindowSettings,
    ) -> Result<Self, AppInfoError> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(settings.instance_descriptor());

        let surface = instance.create_surface(Arc::clone(&window)).unwrap();
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

    pub fn window(&self) -> &Arc<Window> {
        &self.window
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn start_drawing(&self) -> Frame {
        let command_encoder = self.device.create_command_encoder(&Default::default());
        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("failed to get surface texture");

        Frame::new(self, command_encoder, surface_texture)
    }

    pub fn resize_surface(&self, new_size: IntSize) {
        let mut surface_config = self.surface_config.lock().unwrap();

        surface_config.width = new_size.width.max(1);
        surface_config.height = new_size.height.max(1);

        self.surface.configure(&self.device, &surface_config);
    }

    pub fn swapchain_format(&self) -> wgpu::TextureFormat {
        self.swapchain_format
    }
}

pub fn rw_builder(settings: impl RenderWindowSettings) -> impl Fn(&Arc<Window>) -> RenderWindow {
    move |window| RenderWindow::new(Arc::clone(window), &settings).unwrap()
}

pub fn rw_builder_default() -> fn(&Arc<Window>) -> RenderWindow {
    move |window| RenderWindow::new(Arc::clone(window), &DefaultRenderWindowSettings).unwrap()
}
