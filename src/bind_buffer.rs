use {
    crate::{
        to_wgsl_bytes,
        WgslBytesWriteable,
    },
    std::{
        mem,
        num::NonZeroU64,
    },
    wgpu::util::DeviceExt,
};

pub struct BindBuffer<T: WgslBytesWriteable> {
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    value: T,
}

impl<T: WgslBytesWriteable> BindBuffer<T> {
    pub const SIZE: wgpu::BufferAddress = mem::size_of::<T>() as _;

    pub const SIZE_NONZERO: NonZeroU64 = match NonZeroU64::new(Self::SIZE) {
        Some(res) => res,
        None => panic!("size was 0"),
    };

    pub fn new(device: &wgpu::Device, bind_group_layout: &wgpu::BindGroupLayout, value: T) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: &to_wgsl_bytes(&value),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &buffer,
                    offset: 0,
                    size: Some(Self::SIZE_NONZERO),
                }),
            }],
        });

        Self {
            buffer,
            bind_group,
            value,
        }
    }

    pub fn bind<'s>(&'s self, index: u32, render_pass: &mut wgpu::RenderPass<'s>) {
        render_pass.set_bind_group(index, &self.bind_group, &[]);
    }

    pub fn write(&mut self, queue: &wgpu::Queue, value: T) {
        self.value = value;
        queue.write_buffer(&self.buffer, 0, &to_wgsl_bytes(&self.value));
    }

    pub fn value(&self) -> &T {
        &self.value
    }
}
