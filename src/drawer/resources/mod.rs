use sww::window::RenderWindow;

pub mod mesh;

pub trait Resource: 'static {
    fn new(rw: &RenderWindow) -> Self;
}

pub trait ResourceFrom<R>: Resource {
    fn resource_from(resources: &R) -> &'static Self;
}
