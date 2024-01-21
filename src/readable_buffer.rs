use {
    crate::{
        to_wgsl_bytes,
        WgslBytesWriteable,
    },
    wgpu::util::DeviceExt,
};

pub struct ReadableBuffer<T: WgslBytesWriteable> {
    buffer: wgpu::Buffer,
    value: T,
}

impl<T: WgslBytesWriteable> ReadableBuffer<T> {
    pub fn new(device: &wgpu::Device, value: T) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: &to_wgsl_bytes(&value),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        Self { buffer, value }
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    pub fn write(&mut self, queue: &wgpu::Queue, value: T) {
        self.value = value;
        queue.write_buffer(&self.buffer, 0, &to_wgsl_bytes(&self.value));
    }

    pub fn value(&self) -> &T {
        &self.value
    }
}
