use std::io::Write;
use std::mem;

pub fn create_buffer_partially_init<T: bytemuck::NoUninit>(
    device: &wgpu::Device,
    values: &Vec<T>,
    usage: wgpu::BufferUsages,
) -> wgpu::Buffer {
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: values.capacity() as wgpu::BufferAddress * mem::size_of::<T>() as wgpu::BufferAddress,
        usage,
        mapped_at_creation: true,
    });

    (&mut buffer.slice(..).get_mapped_range_mut()[..])
        .write_all(bytemuck::cast_slice(values.as_slice()))
        .unwrap();
    buffer.unmap();

    buffer
}
