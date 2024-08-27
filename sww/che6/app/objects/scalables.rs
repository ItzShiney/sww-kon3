use sww::buffers::MutBuffer;
use sww::shaders::mesh::Transform;
use sww::Vec2;

pub struct Scalable {
    pub transform_buffer: MutBuffer<Transform>,
    pub base_scale: Vec2,
}

impl Scalable {
    pub fn new(transform_buffer: MutBuffer<Transform>, base_scale: Vec2) -> Self {
        Self {
            transform_buffer,
            base_scale,
        }
    }
}

pub type Scalables = Vec<Scalable>;
