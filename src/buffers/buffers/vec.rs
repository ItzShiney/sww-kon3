use crate::app::RenderWindow;
use crate::create_buffer_partially_init;
use std::mem;
use std::ops::Index;
use std::ops::IndexMut;
use std::slice::SliceIndex;

pub struct VecBuffer<'q, T> {
    buffer: wgpu::Buffer,
    values: Vec<T>,
    queue: &'q wgpu::Queue,
}

impl<'q, T: bytemuck::NoUninit + Sized> VecBuffer<'q, T> {
    pub fn new(rw: &'q RenderWindow, values: Vec<T>, usage: wgpu::BufferUsages) -> Self {
        let buffer = create_buffer_partially_init(
            rw.device(),
            &values,
            usage | wgpu::BufferUsages::COPY_DST,
        );

        Self {
            buffer,
            values,
            queue: rw.queue(),
        }
    }

    pub fn new_vertex(rw: &'q RenderWindow, values: Vec<T>) -> Self {
        Self::new(rw, values, wgpu::BufferUsages::VERTEX)
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    pub fn values(&self) -> &[T] {
        &self.values
    }

    pub fn slice(&self, range: impl SliceIndex<[T], Output = [T]>) -> VecBufferSlice<T> {
        VecBufferSlice {
            buffer: &self.buffer,
            values: &self.values[range],
        }
    }

    pub fn slice_mut(
        &mut self,
        range: impl SliceIndex<[T], Output = [T]>,
    ) -> VecBufferSliceMut<'_, T> {
        let start_ptr = self.values.as_ptr();
        let values = &mut self.values[range];
        let start = (values.as_ptr() as usize - start_ptr as usize) / mem::size_of::<T>();

        VecBufferSliceMut {
            buffer: &self.buffer,
            queue: self.queue,

            values,
            start,
        }
    }

    pub fn push(&mut self, value: T) {
        if !self.can_push() {
            panic!("pushing to full VecBuffer");
        }

        self.values.push(value);
        self.update(self.len() - 1..);
    }

    pub fn can_push(&self) -> bool {
        self.len() != self.capacity()
    }

    fn update(&self, range: impl SliceIndex<[T], Output = [T]>) {
        let slice = &self.values[range];
        let start = (slice.as_ptr() as usize - self.values.as_ptr() as usize) / mem::size_of::<T>();

        let offset = start as wgpu::BufferAddress * mem::size_of::<T>() as wgpu::BufferAddress;

        self.queue
            .write_buffer(&self.buffer, offset, bytemuck::cast_slice(slice))
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

impl RenderWindow<'_> {
    pub fn vec_buffer<T: bytemuck::NoUninit + Sized>(
        &self,
        values: Vec<T>,
        usage: wgpu::BufferUsages,
    ) -> VecBuffer<T> {
        VecBuffer::new(self, values, usage)
    }

    pub fn vec_buffer_vertex<T: bytemuck::NoUninit + Sized>(&self, values: Vec<T>) -> VecBuffer<T> {
        VecBuffer::new_vertex(self, values)
    }
}

#[derive(Clone, Copy)]
pub struct VecBufferSlice<'s, T> {
    pub buffer: &'s wgpu::Buffer,
    pub values: &'s [T],
}

pub struct VecBufferSliceMut<'s, T: bytemuck::NoUninit> {
    buffer: &'s wgpu::Buffer,
    queue: &'s wgpu::Queue,

    values: &'s mut [T],
    start: usize,
}

impl<'s, T: bytemuck::NoUninit> Index<usize> for VecBufferSliceMut<'s, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl<'s, T: bytemuck::NoUninit> IndexMut<usize> for VecBufferSliceMut<'s, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

impl<T: bytemuck::NoUninit> Drop for VecBufferSliceMut<'_, T> {
    fn drop(&mut self) {
        self.queue.write_buffer(
            self.buffer,
            self.start as wgpu::BufferAddress * mem::size_of::<T>() as wgpu::BufferAddress,
            bytemuck::cast_slice(self.values),
        );
    }
}
