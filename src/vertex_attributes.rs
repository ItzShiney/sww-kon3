use {
    crate::FieldAttributes,
    std::{
        marker::PhantomData,
        mem,
    },
};

pub struct Attributes<T: FieldAttributes> {
    attributes: Box<[wgpu::VertexAttribute]>,
    step_mode: wgpu::VertexStepMode,
    phantom: PhantomData<T>,
}

impl<T: FieldAttributes> Attributes<T> {
    pub fn new(start: u32, step_mode: wgpu::VertexStepMode) -> Self {
        Self {
            attributes: T::field_attributes(start),
            step_mode,
            phantom: PhantomData,
        }
    }

    pub fn new_vertex(start: u32) -> Self {
        Self::new(start, wgpu::VertexStepMode::Vertex)
    }

    pub fn new_instance(start: u32) -> Self {
        Self::new(start, wgpu::VertexStepMode::Instance)
    }

    pub fn layout(&self) -> wgpu::VertexBufferLayout {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<T>() as _,
            step_mode: self.step_mode,
            attributes: &self.attributes,
        }
    }
}
