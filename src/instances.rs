use {
    std::mem,
    wgpu::util::DeviceExt,
};

pub struct Instances<T: bytemuck::Pod> {
    buffer: wgpu::Buffer,
    pub transform: T,
}

impl<T: bytemuck::Pod> Instances<T> {
    pub fn new(device: &wgpu::Device, transform: T, transforms: &[T]) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(transforms),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self { transform, buffer }
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    pub fn len(&self) -> usize {
        self.buffer.size() as usize / mem::size_of::<T>()
    }
}
