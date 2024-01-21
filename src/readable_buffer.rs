use {
    crate::{
        to_wgsl_bytes,
        WgslBytesWriteable,
    },
    std::{
        mem,
        num::NonZeroU64,
    },
};

pub struct ReadableBuffer<'buffer, T: WgslBytesWriteable> {
    buffer: &'buffer wgpu::Buffer,
    value: T,
}

impl<'buffer, T: WgslBytesWriteable> ReadableBuffer<'buffer, T> {
    pub const SIZE: wgpu::BufferAddress = mem::size_of::<T>() as _;

    pub const SIZE_NONZERO: NonZeroU64 = match NonZeroU64::new(Self::SIZE) {
        Some(res) => res,
        None => panic!("size was 0"),
    };

    pub fn new(buffer: &'buffer wgpu::Buffer, value: T) -> Self {
        Self { buffer, value }
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        self.buffer
    }

    pub fn write(&mut self, queue: &wgpu::Queue, value: T) {
        self.value = value;
        queue.write_buffer(self.buffer, 0, &to_wgsl_bytes(&self.value));
    }

    pub fn value(&self) -> &T {
        &self.value
    }
}
