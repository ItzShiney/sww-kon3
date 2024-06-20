use std::mem::ManuallyDrop;

pub struct FrameSurface {
    texture: ManuallyDrop<wgpu::SurfaceTexture>,
    view: wgpu::TextureView,
}

impl FrameSurface {
    pub(super) fn new(surface_texture: wgpu::SurfaceTexture) -> Self {
        let view = surface_texture.texture.create_view(&Default::default());
        Self {
            texture: ManuallyDrop::new(surface_texture),
            view,
        }
    }

    pub fn texture(&self) -> &wgpu::SurfaceTexture {
        &self.texture
    }

    pub fn view(&self) -> &wgpu::TextureView {
        &self.view
    }
}

impl Drop for FrameSurface {
    fn drop(&mut self) {
        let texture = unsafe { ManuallyDrop::take(&mut self.texture) };
        texture.present();
    }
}
