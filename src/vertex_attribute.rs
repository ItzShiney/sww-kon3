pub trait VertexAttribute {
    fn vertex_attributes(start: u32) -> Box<[wgpu::VertexAttribute]>;
}
