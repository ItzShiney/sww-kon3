use crate::resources::Resource;
use std::ops::Deref;
use sww::buffers::Binding;
use sww::buffers::MutBuffer;
use sww::drawing::Mesh;
use sww::media::make_default_texture;
use sww::media::DefaultView;
use sww::shaders::mesh::BindGroup0;
use sww::shaders::mesh::BindGroup1;
use sww::shaders::mesh::BindGroupLayout0;
use sww::shaders::mesh::BindGroupLayout1;
use sww::shaders::mesh::Transform;
use sww::window::RenderWindow;
use sww::Vec2;

/// A mesh of a unit square with its top left corner being positioned at `(0, 0)`.
pub struct UnitSquareTopLeft(Mesh);

impl Resource for UnitSquareTopLeft {
    fn new(rw: &RenderWindow) -> Self {
        Self(Mesh::rect(rw, Vec2::ONE))
    }
}

impl Deref for UnitSquareTopLeft {
    type Target = Mesh;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A bind group 0 with no global transform.
pub struct NoGlobalTransform(BindGroup0);

impl Resource for NoGlobalTransform {
    fn new(rw: &RenderWindow) -> Self {
        Self(BindGroup0::from_bindings(
            rw.device(),
            BindGroupLayout0 {
                global_transform: MutBuffer::new_uniform(rw.device(), Transform::default())
                    .buffer()
                    .binding(),
            },
        ))
    }
}

impl Deref for NoGlobalTransform {
    type Target = BindGroup0;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A bind group 1 with no texture (a white pixel).
pub struct DefaultTexture(BindGroup1);

impl Resource for DefaultTexture {
    fn new(rw: &RenderWindow) -> Self {
        let default_texture = make_default_texture(rw.device(), rw.queue());

        Self(BindGroup1::from_bindings(
            rw.device(),
            BindGroupLayout1 {
                texture: &default_texture.default_view(),
            },
        ))
    }
}

impl Deref for DefaultTexture {
    type Target = BindGroup1;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
