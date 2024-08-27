pub trait Binding {
    type Output;

    fn binding(self) -> Self::Output;
}

impl<'s> Binding for &'s wgpu::Buffer {
    type Output = wgpu::BufferBinding<'s>;

    fn binding(self) -> Self::Output {
        wgpu::BufferBinding {
            buffer: self,
            offset: 0,
            size: None,
        }
    }
}
