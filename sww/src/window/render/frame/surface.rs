pub struct FrameSurface {
    texture: Option<wgpu::SurfaceTexture>,
    view: wgpu::TextureView,
}

impl FrameSurface {
    pub(super) fn new(surface_texture: wgpu::SurfaceTexture) -> Self {
        let view = surface_texture.texture.create_view(&Default::default());
        Self {
            texture: Some(surface_texture),
            view,
        }
    }

    pub fn texture(&self) -> &wgpu::SurfaceTexture {
        self.texture.as_ref().unwrap()
    }

    pub fn view(&self) -> &wgpu::TextureView {
        &self.view
    }
}

impl Drop for FrameSurface {
    fn drop(&mut self) {
        let texture = self.texture.take().unwrap();
        texture.present();
    }
}
