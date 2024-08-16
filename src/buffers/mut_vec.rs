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

    pub fn default(rw: &'w RenderWindow<'w>, usage: wgpu::BufferUsages) -> Self {
        Self::new(rw, Vec::default(), usage)
    }

    pub fn new_vertex(rw: &'w RenderWindow<'w>, values: Vec<T>) -> Self {
        Self::new(rw, values, wgpu::BufferUsages::VERTEX)
    }

    pub fn default_vertex(rw: &'w RenderWindow<'w>) -> Self {
        Self::default(rw, wgpu::BufferUsages::VERTEX)
    }

    pub fn values(&self) -> &[T] {
        &self.values
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
        (self.len() * mem::size_of::<T>()) as _
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }

    pub fn pop(&mut self) -> Option<T> {
        self.values.pop()
    }

    pub fn push(&mut self, value: T) {
        self.values.push(value);
    }

    pub fn push_within_capacity(&mut self, value: T) -> Result<(), T> {
        if self.values.len() < self.values.capacity() {
            self.values.push(value);
            Ok(())
        } else {
            Err(value)
        }
    }

    pub fn shrink_to_fit(&mut self) {
        self.values.shrink_to_fit();
    }

    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.values.shrink_to(min_capacity);
    }

    pub fn update_buffer(&mut self) -> wgpu::BufferSlice<'_> {
        if let Some(buffer) = &self.buffer {
            let buffer_capacity = buffer.size() as usize / mem::size_of::<T>();
            if buffer_capacity != self.values.capacity() {
                self.buffer = None;
            }
        }

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
