use parking_lot::RwLock;
use parking_lot::RwLockUpgradableReadGuard;
use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Arc;
use sww::window::RenderWindow;

pub mod mesh;

pub trait Resource: 'static {
    fn new(rw: &RenderWindow) -> Self;
}

pub struct Resources {
    rw: Arc<RenderWindow>,
    resources: RwLock<HashMap<TypeId, &'static dyn Any>>,
}

impl Resources {
    pub fn new(rw: Arc<RenderWindow>) -> Self {
        Self {
            rw,
            resources: Default::default(),
        }
    }

    pub fn get<T: Resource>(&self) -> &'static T {
        let guard = self.resources.upgradable_read();
        let key = TypeId::of::<T>();

        if let Some(&res) = guard.get(&key) {
            res
        } else {
            let mut guard = RwLockUpgradableReadGuard::upgrade(guard);
            guard.insert(key, Box::<dyn Any>::leak(Box::new(T::new(&self.rw))));
            guard[&key]
        }
        .downcast_ref()
        .unwrap()
    }
}
