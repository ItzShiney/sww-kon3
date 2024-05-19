use std::io::Write;
use std::mem;

pub fn to_wgsl_bytes<T: bytemuck::NoUninit + ?Sized>(value: &T) -> &[u8] {
    bytemuck::bytes_of(value)
}

pub fn from_wgsl_bytes<T: bytemuck::AnyBitPattern>(bytes: &[u8]) -> &T {
    bytemuck::from_bytes(&bytes)
}

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
