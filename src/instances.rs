use {
    crate::{
        to_wgsl_bytes,
        WgslBytesWriteable,
    },
    std::mem,
    wgpu::util::DeviceExt,
};

pub struct Instances<T>
where
    [T]: WgslBytesWriteable,
{
    buffer: wgpu::Buffer,
    pub transform: T,
}

impl<T> Instances<T>
where
    [T]: WgslBytesWriteable,
{
    pub fn new(device: &wgpu::Device, transform: T, transforms: &[T]) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: &to_wgsl_bytes(transforms),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self { transform, buffer }
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    pub fn len(&self) -> usize {
        self.buffer.size() as usize / mem::size_of::<T>()
    }
}
