pub trait Draw<'c>: 'c {
    fn draw(&self, render_pass: &mut wgpu::RenderPass<'c>);
}
