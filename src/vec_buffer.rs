use crate::create_buffer_partially_init;
use crate::to_wgsl_bytes;
use crate::AppInfo;
use crate::WgslBytesWriteable;
use crate::WgslBytesWriteableSized;
use std::mem;
use std::ops::Index;
use std::ops::IndexMut;
use std::slice::SliceIndex;

pub struct VecBuffer<T: WgslBytesWriteable> {
    buffer: wgpu::Buffer,
    values: Vec<T>,
}

impl<T: WgslBytesWriteableSized> VecBuffer<T> {
    pub fn new(app_info: &AppInfo, values: Vec<T>, usage: wgpu::BufferUsages) -> Self {
        let buffer = create_buffer_partially_init(
            &app_info.device,
            &values,
            usage | wgpu::BufferUsages::COPY_DST,
        );

        Self { buffer, values }
    }

    pub fn new_vertex(app_info: &AppInfo, values: Vec<T>) -> Self {
        Self::new(app_info, values, wgpu::BufferUsages::VERTEX)
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    pub fn values(&self) -> &[T] {
        &self.values
    }

    pub fn slice<'s>(&'s self, range: impl SliceIndex<[T], Output = [T]>) -> VecBufferSlice<'s, T> {
        VecBufferSlice {
            buffer: &self.buffer,
            values: &self.values[range],
        }
    }

    pub fn slice_mut<'s>(
        &'s mut self,
        queue: &'s wgpu::Queue,
        range: impl SliceIndex<[T], Output = [T]>,
    ) -> VecBufferSliceMut<'s, T> {
        let start_ptr = self.values.as_ptr();
        let values = &mut self.values[range];
        let start = (values.as_ptr() as usize - start_ptr as usize) / mem::size_of::<T>();

        VecBufferSliceMut {
            buffer: &self.buffer,
            queue,

            values,
            start,
        }
    }

    pub fn push(&mut self, queue: &wgpu::Queue, value: T) {
        if !self.can_push() {
            panic!("pushing to full VecBuffer");
        }

        self.values.push(value);
        self.update(queue, self.len() - 1..);
    }

    pub fn can_push(&self) -> bool {
        self.len() != self.capacity()
    }

    fn update(&self, queue: &wgpu::Queue, range: impl SliceIndex<[T], Output = [T]>) {
        let slice = &self.values[range];
        let start = (slice.as_ptr() as usize - self.values.as_ptr() as usize) / mem::size_of::<T>();

        let offset = start as wgpu::BufferAddress * T::SHADER_SIZE.get();

        queue.write_buffer(&self.buffer, offset, &to_wgsl_bytes(slice))
    }

    pub fn pop(&mut self) -> Option<T> {
        self.values.pop()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() != 0
    }

    pub fn capacity(&self) -> usize {
        self.values.capacity()
    }

    pub fn size(&self) -> wgpu::BufferAddress {
        (self.len() * mem::size_of::<T>()) as u64
    }
}

impl AppInfo<'_> {
    pub fn vec_buffer<T: WgslBytesWriteableSized>(
        &self,
        values: Vec<T>,
        usage: wgpu::BufferUsages,
    ) -> VecBuffer<T> {
        VecBuffer::new(self, values, usage)
    }

    pub fn vec_buffer_vertex<T: WgslBytesWriteableSized>(&self, values: Vec<T>) -> VecBuffer<T> {
        VecBuffer::new_vertex(self, values)
    }
}

#[derive(Clone, Copy)]
pub struct VecBufferSlice<'s, T: WgslBytesWriteable> {
    pub buffer: &'s wgpu::Buffer,
    pub values: &'s [T],
}

pub struct VecBufferSliceMut<'s, T: WgslBytesWriteableSized> {
    buffer: &'s wgpu::Buffer,
    queue: &'s wgpu::Queue,

    values: &'s mut [T],
    start: usize,
}

impl<'s, T: WgslBytesWriteableSized> Index<usize> for VecBufferSliceMut<'s, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl<'s, T: WgslBytesWriteableSized> IndexMut<usize> for VecBufferSliceMut<'s, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

impl<T: WgslBytesWriteableSized> Drop for VecBufferSliceMut<'_, T> {
    fn drop(&mut self) {
        let size = <T as encase::ShaderSize>::SHADER_SIZE.get();

        self.queue.write_buffer(
            &self.buffer,
            self.start as u64 * size,
            &to_wgsl_bytes(&self.values),
        );
    }
}
