use crate::buffers::create_buffer_partially_init;
use crate::window::RenderWindow;
use parking_lot::Mutex;
use std::mem;
use std::ops::Index;
use std::ops::IndexMut;
use std::slice::SliceIndex;
use std::sync::Arc;

#[derive(Default)]
struct Cache {
    buffer: Option<Arc<wgpu::Buffer>>,
    is_up_to_date: bool,
}

pub struct MutVecBuffer<T> {
    rw: Arc<RenderWindow>,
    values: Vec<T>,

    usage: wgpu::BufferUsages,
    cache: Mutex<Cache>,
}

impl<T> MutVecBuffer<T> {
    pub fn new(rw: Arc<RenderWindow>, values: Vec<T>, usage: wgpu::BufferUsages) -> Self {
        let usage = usage | wgpu::BufferUsages::COPY_DST;
        let cache = Mutex::default();

        Self {
            rw,
            values,

            usage,
            cache,
        }
    }

    pub fn default(rw: Arc<RenderWindow>, usage: wgpu::BufferUsages) -> Self {
        Self::new(rw, Vec::default(), usage)
    }

    pub fn new_vertex(rw: Arc<RenderWindow>, values: Vec<T>) -> Self {
        Self::new(rw, values, wgpu::BufferUsages::VERTEX)
    }

    pub fn default_vertex(rw: Arc<RenderWindow>) -> Self {
        Self::default(rw, wgpu::BufferUsages::VERTEX)
    }

    pub fn values(&self) -> &[T] {
        &self.values
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn capacity(&self) -> usize {
        self.values.capacity()
    }

    pub fn size(&self) -> wgpu::BufferAddress {
        (self.len() * mem::size_of::<T>()) as _
    }

    pub fn clear(&mut self) {
        self.values_mut().clear();
    }

    pub fn pop(&mut self) -> Option<T> {
        self.values_mut().pop()
    }

    pub fn push(&mut self, value: T) {
        self.values_mut().push(value);
    }

    pub fn push_within_capacity(&mut self, value: T) -> Result<(), T> {
        if self.values.len() < self.values.capacity() {
            self.values_mut().push(value);
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

    fn values_mut(&mut self) -> &mut Vec<T> {
        self.cache.lock().is_up_to_date = false;
        &mut self.values
    }
}

impl<T: bytemuck::NoUninit> MutVecBuffer<T> {
    pub fn buffer(&self) -> Arc<wgpu::Buffer> {
        let mut cache = self.cache.lock();

        if let Some(buffer) = &cache.buffer {
            let buffer_capacity = buffer.size() as usize / mem::size_of::<T>();
            if buffer_capacity != self.values.capacity() {
                *cache = Cache::default();
            }
        }

        let buffer = Arc::clone(cache.buffer.get_or_insert_with(|| {
            Arc::new(create_buffer_partially_init(
                self.rw.device(),
                &self.values,
                self.values.capacity(),
                self.usage,
            ))
        }));

        if !self.is_empty() && !cache.is_up_to_date {
            self.rw
                .queue()
                .write_buffer(&buffer, 0, bytemuck::cast_slice(&self.values));
            cache.is_up_to_date = true;
        }

        buffer
    }
}

impl<T, I: SliceIndex<[T]>> Index<I> for MutVecBuffer<T> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.values[index]
    }
}

impl<T, I: SliceIndex<[T]>> IndexMut<I> for MutVecBuffer<T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.values_mut()[index]
    }
}
