pub trait Draw<'e>: 'e {
    fn draw(&self, render_pass: &mut wgpu::RenderPass<'e>);
}
