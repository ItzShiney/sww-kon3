use std::any::Any;
use sww::window::RenderWindow;
use tags::mesh::DefaultTexture;
use tags::mesh::NoTransform;
use tags::mesh::UnitSquareTopLeft;
use tags::Resource;

pub mod tags {
    use sww::window::RenderWindow;

    pub trait Resource: 'static {
        fn build(rw: &RenderWindow) -> Self;
    }

    pub mod mesh {
        use super::Resource;
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

        pub struct UnitSquareTopLeft(Mesh);

        impl Resource for UnitSquareTopLeft {
            fn build(rw: &RenderWindow) -> Self {
                Self(Mesh::rect(rw, Vec2::ONE))
            }
        }

        impl Deref for UnitSquareTopLeft {
            type Target = Mesh;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        pub struct NoTransform(BindGroup0);

        impl Resource for NoTransform {
            fn build(rw: &RenderWindow) -> Self {
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

        impl Deref for NoTransform {
            type Target = BindGroup0;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        pub struct DefaultTexture(BindGroup1);

        impl Resource for DefaultTexture {
            fn build(rw: &RenderWindow) -> Self {
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
    }
}

pub struct Resources(UnitSquareTopLeft, NoTransform, DefaultTexture);

impl Resources {
    pub fn new(rw: &RenderWindow) -> Self {
        Self(
            Resource::build(rw),
            Resource::build(rw),
            Resource::build(rw),
        )
    }

    pub fn get<T: Resource>(&self) -> &T {
        [
            <dyn Any>::downcast_ref::<T>(&self.0),
            <dyn Any>::downcast_ref::<T>(&self.1),
            <dyn Any>::downcast_ref::<T>(&self.2),
        ]
        .into_iter()
        .filter_map(|resource| resource)
        .next()
        .expect("unexpected resource requested")
    }
}
