use {
    crate::{
        to_wgsl_bytes,
        WgslBytesWriteable,
    },
    std::{
        marker::PhantomData,
        mem,
    },
    wgpu::util::DeviceExt,
};

pub struct Instances<T>
where
    [T]: WgslBytesWriteable,
{
    buffer: wgpu::Buffer,
    phantom: PhantomData<T>,
}

impl<T> Instances<T>
where
    [T]: WgslBytesWriteable,
{
    pub fn new(device: &wgpu::Device, transforms: &[T]) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: &to_wgsl_bytes(transforms),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            buffer,
            phantom: PhantomData,
        }
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    pub fn len(&self) -> usize {
        self.buffer.size() as usize / mem::size_of::<T>()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
