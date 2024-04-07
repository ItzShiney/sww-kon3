use winit::window::Window;

pub trait Ratio {
    fn ratio(&self) -> f32;
}

impl Ratio for Window {
    fn ratio(&self) -> f32 {
        let size = self.inner_size();
        size.width as f32 / size.height as f32
    }
}
