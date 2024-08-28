use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use sww::window::RenderWindow;

pub mod mesh;

pub trait Resource: 'static {
    fn new(rw: &RenderWindow) -> Self;
}

pub struct Resources {
    rw: Arc<RenderWindow>,
    resources: Mutex<HashMap<TypeId, &'static dyn Any>>,
}

impl Resources {
    pub fn new(rw: Arc<RenderWindow>) -> Self {
        Self {
            rw,
            resources: Default::default(),
        }
    }

    pub fn get<T: Resource>(&self) -> &'static T {
        self.resources
            .lock()
            .unwrap()
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::<dyn Any>::leak(Box::new(T::new(&self.rw))))
            .downcast_ref()
            .unwrap()
    }
}
