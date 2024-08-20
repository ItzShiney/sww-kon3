use std::sync::Arc;
use std::sync::Mutex;

mod lock;

pub use lock::*;

pub struct Shared<T: ?Sized + 'static>(Arc<Mutex<T>>);

impl<T: ?Sized> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<T> Shared<T> {
    pub fn new(value: T) -> Self {
        Self(Arc::new(Mutex::new(value)))
    }
}

impl<T: ?Sized> Shared<T> {
    pub fn lock(&self) -> SharedLock<'_, T> {
        SharedLock(self.0.lock().expect("shared value was already locked"))
    }
}
