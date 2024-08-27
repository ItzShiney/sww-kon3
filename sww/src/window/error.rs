#[derive(Debug)]
pub enum AppInfoError {
    NoAdapter,
    NoDevice(wgpu::RequestDeviceError),
}

impl From<wgpu::RequestDeviceError> for AppInfoError {
    fn from(value: wgpu::RequestDeviceError) -> Self {
        Self::NoDevice(value)
    }
}
