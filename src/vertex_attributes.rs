use {
    crate::VertexAttribute,
    std::{
        marker::PhantomData,
        mem,
    },
};

pub struct VertexAttributes<T> {
    attributes: Box<[wgpu::VertexAttribute]>,
    phantom: PhantomData<T>,
}

impl<T: VertexAttribute> VertexAttributes<T> {
    pub fn new(start: u32) -> Self {
        Self {
            attributes: T::vertex_attributes(start),
            phantom: PhantomData,
        }
    }

    pub fn layout(&self) -> wgpu::VertexBufferLayout {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<T>() as _,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &self.attributes,
        }
    }
}
