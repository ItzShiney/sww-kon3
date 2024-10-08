use std::ops::Deref;
use std::ops::DerefMut;
use wgpu::util::DeviceExt;

pub struct MutBuffer<T: bytemuck::NoUninit> {
    buffer: wgpu::Buffer,
    value: T,
}

impl<T: bytemuck::NoUninit> MutBuffer<T> {
    pub fn new(device: &wgpu::Device, value: T, usage: wgpu::BufferUsages) -> Self {
        let usage = usage | wgpu::BufferUsages::COPY_DST;
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::bytes_of(&value),
            usage,
        });

        Self { buffer, value }
    }

    pub fn new_uniform(device: &wgpu::Device, value: T) -> Self {
        Self::new(device, value, wgpu::BufferUsages::UNIFORM)
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    pub fn set(&mut self, queue: &wgpu::Queue, value: T) {
        self.value = value;
        self.update(queue);
    }

    fn update(&mut self, queue: &wgpu::Queue) {
        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&self.value));
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn value_mut<'w>(&'w mut self, queue: &'w wgpu::Queue) -> ReadableBufferMut<'w, T> {
        ReadableBufferMut {
            buffer: self,
            queue,
        }
    }
}

pub struct ReadableBufferMut<'w, T: bytemuck::NoUninit> {
    buffer: &'w mut MutBuffer<T>,
    queue: &'w wgpu::Queue,
}

impl<T: bytemuck::NoUninit> Deref for ReadableBufferMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.buffer.value
    }
}

impl<T: bytemuck::NoUninit> DerefMut for ReadableBufferMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer.value
    }
}

impl<T: bytemuck::NoUninit> Drop for ReadableBufferMut<'_, T> {
    fn drop(&mut self) {
        self.buffer.update(self.queue);
    }
}
