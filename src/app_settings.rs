use winit::dpi::PhysicalSize;

pub trait AppSettings {
    fn instance_descriptor(&self) -> wgpu::InstanceDescriptor {
        Default::default()
    }

    fn request_adapter_options<'surface>(
        &self,
        surface: &'surface wgpu::Surface,
    ) -> wgpu::RequestAdapterOptions<'_, 'surface> {
        wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(surface),
        }
    }

    fn device_descriptor(&self, adapter: &wgpu::Adapter) -> wgpu::DeviceDescriptor {
        wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::default(),
            required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                .using_resolution(adapter.limits()),
        }
    }

    fn surface_config(
        &self,
        size: PhysicalSize<u32>,
        swapchain_format: wgpu::TextureFormat,
        swapchain_alpha_mode: wgpu::CompositeAlphaMode,
    ) -> wgpu::SurfaceConfiguration {
        wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Immediate,
            desired_maximum_frame_latency: 2,
            alpha_mode: swapchain_alpha_mode,
            view_formats: Default::default(),
        }
    }
}

pub struct DefaultAppSettings;

impl AppSettings for DefaultAppSettings {}
