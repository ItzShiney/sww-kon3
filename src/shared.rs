use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

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

pub struct SharedLock<'s, T: ?Sized>(MutexGuard<'s, T>);

impl<T: ?Sized> Deref for SharedLock<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ?Sized> DerefMut for SharedLock<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
