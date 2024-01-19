use {
    crate::{
        to_wgsl_bytes,
        BindBuffer,
        WgslBytesWriteable,
    },
    std::mem,
    wgpu::util::DeviceExt,
};

pub struct Instances<T>
where
    T: WgslBytesWriteable,
    [T]: WgslBytesWriteable,
{
    buffer: wgpu::Buffer,
    transform: BindBuffer<T>,
}

impl<T> Instances<T>
where
    T: WgslBytesWriteable,
    [T]: WgslBytesWriteable,
{
    pub fn new(device: &wgpu::Device, transforms: &[T], transform: BindBuffer<T>) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: &to_wgsl_bytes(transforms),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self { buffer, transform }
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    pub fn len(&self) -> usize {
        self.buffer.size() as usize / mem::size_of::<T>()
    }

    pub fn transform(&self) -> &BindBuffer<T> {
        &self.transform
    }

    pub fn transform_mut(&mut self) -> &mut BindBuffer<T> {
        &mut self.transform
    }
}
