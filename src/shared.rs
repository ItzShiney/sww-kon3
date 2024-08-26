use crate::InvalidateCache;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync as arc;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

struct SharedInner<T: ?Sized + 'static> {
    notify: arc::Weak<dyn InvalidateCache<T>>,
    value: T,
}

pub struct Shared<T: ?Sized + 'static>(Arc<Mutex<SharedInner<T>>>);

impl<T: ?Sized> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<T> Shared<T> {
    pub fn new(value: T, notify: arc::Weak<dyn InvalidateCache<T>>) -> Self {
        Self(Arc::new(Mutex::new(SharedInner { notify, value })))
    }
}

impl<T: ?Sized> Shared<T> {
    pub fn lock(&self) -> SharedLock<'_, T> {
        SharedLock {
            shared: self,
            guard: self.0.lock().expect("shared value was already locked"),
        }
    }

    pub(crate) fn addr(&self) -> usize {
        Arc::as_ptr(&self.0).cast::<()>() as usize
    }
}

pub struct SharedLock<'s, T: ?Sized + 'static> {
    shared: &'s Shared<T>,
    guard: MutexGuard<'s, SharedInner<T>>,
}

impl<T: ?Sized> Deref for SharedLock<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.guard.value
    }
}

impl<T: ?Sized> DerefMut for SharedLock<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        (self.guard.notify.upgrade().unwrap()).invalidate_cache(self.shared);

        &mut self.guard.value
    }
}

pub trait NewShared<T> {
    fn new_shared(&self, value: T) -> Shared<T>;
}

impl<T, N: InvalidateCache<T> + 'static> NewShared<T> for arc::Weak<N> {
    fn new_shared(&self, value: T) -> Shared<T> {
        Shared::new(value, Self::clone(self))
    }
}
