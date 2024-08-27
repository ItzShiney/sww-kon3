use bytemuck::NoUninit;
use std::marker::PhantomData;
use wgpu::util::DeviceExt;

pub struct VecBuffer<T: NoUninit> {
    buffer: wgpu::Buffer,
    count: usize,
    phantom: PhantomData<T>,
}

impl<T: NoUninit> VecBuffer<T> {
    pub fn new(device: &wgpu::Device, values: &[T], usage: wgpu::BufferUsages) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(values),
            usage,
        });

        Self {
            buffer,
            count: values.len(),
            phantom: PhantomData,
        }
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    pub fn count(&self) -> usize {
        self.count
    }
}
