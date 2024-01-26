use {
    crate::{
        to_wgsl_bytes,
        WgslBytesWriteable,
        WgslBytesWriteableSized,
    },
    std::{
        mem,
        ops::{
            Index,
            IndexMut,
        },
        slice::SliceIndex,
    },
    wgpu::util::DeviceExt,
};

pub struct VecBuffer<T: WgslBytesWriteable> {
    buffer: wgpu::Buffer,
    values: Vec<T>,
}

impl<T: WgslBytesWriteableSized> VecBuffer<T> {
    pub fn new(device: &wgpu::Device, values: Vec<T>) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: &to_wgsl_bytes(&values),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        Self { buffer, values }
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
