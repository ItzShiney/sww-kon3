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

pub trait DefaultView {
    fn default_view(&self) -> wgpu::TextureView;
}

impl DefaultView for wgpu::Texture {
    fn default_view(&self) -> wgpu::TextureView {
        self.create_view(&Default::default())
    }
}
