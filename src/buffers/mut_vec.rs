use crate::buffers::create_buffer_partially_init;
use crate::window::RenderWindow;
use std::mem;
use std::ops::Index;
use std::ops::IndexMut;
use std::slice::SliceIndex;

pub struct MutVecBuffer<'w, T> {
    buffer: wgpu::Buffer,
    values: Vec<T>,
    queue: &'w wgpu::Queue,
}

impl<'w, T: bytemuck::NoUninit + Sized> MutVecBuffer<'w, T> {
    pub fn new(rw: &'w RenderWindow, values: Vec<T>, usage: wgpu::BufferUsages) -> Self {
        let buffer = create_buffer_partially_init(
            rw.device(),
            values.as_slice(),
            values.capacity(),
            usage | wgpu::BufferUsages::COPY_DST,
        );

        Self {
            buffer,
            values,
            queue: rw.queue(),
        }
    }

    pub fn new_vertex(rw: &'w RenderWindow, values: Vec<T>) -> Self {
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
    ) -> MutVecBuffer<T> {
        MutVecBuffer::new(self, values, usage)
    }

    pub fn vec_buffer_vertex<T: bytemuck::NoUninit + Sized>(
        &self,
        values: Vec<T>,
    ) -> MutVecBuffer<T> {
        MutVecBuffer::new_vertex(self, values)
    }
}

#[derive(Clone, Copy)]
pub struct VecBufferSlice<'w, T> {
    pub buffer: &'w wgpu::Buffer,
    pub values: &'w [T],
}

pub struct VecBufferSliceMut<'w, T: bytemuck::NoUninit> {
    buffer: &'w wgpu::Buffer,
    queue: &'w wgpu::Queue,

    values: &'w mut [T],
    start: usize,
}

impl<'w, T: bytemuck::NoUninit> Index<usize> for VecBufferSliceMut<'w, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl<'w, T: bytemuck::NoUninit> IndexMut<usize> for VecBufferSliceMut<'w, T> {
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
