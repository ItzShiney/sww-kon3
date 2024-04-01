use {
    crate::AppSettings,
    pollster::FutureExt,
    winit::window::Window,
};

pub struct AppInfo<'window> {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface<'window>,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub swapchain_format: wgpu::TextureFormat,
}

impl<'window> AppInfo<'window> {
    pub fn new(window: &'window Window, settings: &impl AppSettings) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(settings.instance_descriptor());

        let surface = instance.create_surface(window).unwrap();
        let adapter = instance
            .request_adapter(&settings.request_adapter_options(&surface))
            .block_on()
            .expect("failed to find an adapter");

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];
        let swapchain_alpha_mode = swapchain_capabilities.alpha_modes[0];

        let (device, queue) = adapter
            .request_device(&settings.device_descriptor(&adapter), None)
            .block_on()
            .expect("failed to create device");

        let surface_config = settings.surface_config(size, swapchain_format, swapchain_alpha_mode);
        surface.configure(&device, &surface_config);

        AppInfo {
            device,
            queue,
            surface,
            surface_config,
            swapchain_format,
        }
    }
}
