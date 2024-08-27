use std::io::Write;
use std::mem;

pub fn create_buffer_partially_init<T: bytemuck::NoUninit>(
    device: &wgpu::Device,
    values: &[T],
    capacity: usize,
    usage: wgpu::BufferUsages,
) -> wgpu::Buffer {
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: (capacity * mem::size_of::<T>()) as wgpu::BufferAddress,
        usage,
        mapped_at_creation: true,
    });

    (&mut buffer.slice(..).get_mapped_range_mut()[..])
        .write_all(bytemuck::cast_slice(values))
        .unwrap();
    buffer.unmap();

    buffer
}
