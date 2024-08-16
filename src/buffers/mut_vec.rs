use crate::buffers::create_buffer_partially_init;
use crate::window::RenderWindow;
use std::mem;
use std::ops::Index;
use std::ops::IndexMut;
use std::slice::SliceIndex;

pub struct MutVecBuffer<'w, T> {
    rw: &'w RenderWindow<'w>,
    values: Vec<T>,
    buffer: Option<wgpu::Buffer>,
    usage: wgpu::BufferUsages,
}

impl<'w, T: bytemuck::NoUninit + Sized> MutVecBuffer<'w, T> {
    pub fn new(rw: &'w RenderWindow<'w>, values: Vec<T>, usage: wgpu::BufferUsages) -> Self {
        let usage = usage | wgpu::BufferUsages::COPY_DST;

        Self {
            rw,
            values,
            buffer: None,
            usage,
        }
    }

    pub fn new_vertex(rw: &'w RenderWindow<'w>, values: Vec<T>) -> Self {
        Self::new(rw, values, wgpu::BufferUsages::VERTEX)
    }

    pub fn values(&self) -> &[T] {
        &self.values
    }

    pub fn push(&mut self, value: T) {
        self.values.push(value);

        if !(self.len() < self.capacity()) {
            self.buffer = None;
        }
    }

    pub fn update_buffer(&mut self) -> wgpu::BufferSlice<'_> {
        let buffer = &*self.buffer.get_or_insert_with(|| {
            create_buffer_partially_init(
                self.rw.device(),
                &self.values,
                self.values.capacity(),
                self.usage,
            )
        });

        self.rw
            .queue()
            .write_buffer(buffer, 0, bytemuck::cast_slice(&self.values));
        buffer.slice(..)
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

impl<T, I: SliceIndex<[T]>> Index<I> for MutVecBuffer<'_, T> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.values[index]
    }
}

impl<T, I: SliceIndex<[T]>> IndexMut<I> for MutVecBuffer<'_, T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.values[index]
    }
}
