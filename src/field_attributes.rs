pub trait FieldAttributes {
    fn field_attributes(start: u32) -> Box<[wgpu::VertexAttribute]>;
}
