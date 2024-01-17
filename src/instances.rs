use {
    std::marker::PhantomData,
    wgpu::util::DeviceExt,
};

pub struct Instances<T: bytemuck::Pod> {
    buffer: wgpu::Buffer,
    len: usize,
    phantom: PhantomData<T>,
}

impl<T: bytemuck::Pod> Instances<T> {
    pub fn new(device: &wgpu::Device, transforms: &[T]) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(transforms),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let count = transforms.len();

        Self {
            buffer,
            len: count,
            phantom: PhantomData,
        }
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
